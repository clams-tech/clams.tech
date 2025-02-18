use crate::db::{upsert_zap, Zap};
use crate::State;
use anyhow::{anyhow, Result};
use axum::body::Bytes;
use axum::extract::{Path, Query};
use axum::http::{HeaderMap, StatusCode};
use axum::{Extension, Json};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use bitcoin::hashes::{sha256, Hash};
use chrono::{DateTime, Utc};
use hmac::{Hmac, Mac};
use lightning_invoice::Bolt11Invoice;
use lnurl::pay::PayResponse;
use lnurl::Tag;
use nostr::{Event, JsonUtil};
use reqwest::header::AUTHORIZATION;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::Sha256;
use std::collections::HashMap;
use std::str::FromStr;
use std::time::Duration;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct PhoenixdInvoiceCreate {
    description_hash: String,
    amount_sat: u64,
    expiry_seconds: u64,
    external_id: String,
}

#[derive(Deserialize)]
pub struct InvoiceParams {
    hash: String,
    name: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct InvoiceResponse {
    amount_sat: u64,
    payment_hash: String,
    serialized: String,
}

pub(crate) async fn get_invoice_impl(
    state: State,
    params: InvoiceParams,
    amount_msats: u64,
    zap_request: Option<Event>,
) -> anyhow::Result<String> {
    let desc_hash = match zap_request.as_ref() {
        None => sha256::Hash::from_str(&params.hash)?,
        Some(event) => {
            // todo validate as valid zap request
            if event.kind != nostr::Kind::ZapRequest {
                return Err(anyhow!("Invalid zap request"));
            }
            sha256::Hash::hash(event.as_json().as_bytes())
        }
    };

    let url = format!(
        "http://{}:{}/createinvoice",
        &state.config.phoenixd_host, &state.config.phoenixd_port
    );

    let auth = format!(":{}", &state.config.phoenixd_password);
    let encoded_auth = BASE64.encode(auth.as_bytes());
    let amount_sat = amount_msats / 1000;

    let invoice_request = PhoenixdInvoiceCreate {
        description_hash: desc_hash.to_string(),
        amount_sat,
        expiry_seconds: 3600,
        external_id: params.name,
    };

    let response = state
        .http_client
        .post(url)
        .header(AUTHORIZATION, format!("Basic {}", encoded_auth))
        .form(&invoice_request)
        .send()
        .await?;

    let status = response.status();

    let invoice = match status {
        reqwest::StatusCode::OK => {
            let invoice = response.json::<InvoiceResponse>().await?;
            Ok(invoice)
        }
        _ => Err(anyhow!("Could not get invoice")),
    }?;

    if invoice.amount_sat != amount_sat {
        eprintln!("Invoice amount does not match");
        return Err(anyhow!("Could not create invoice"));
    }

    if let Some(zap_request) = zap_request {
        let bolt11 = Bolt11Invoice::from_str(&invoice.serialized)?;
        let zap = Zap {
            invoice: bolt11,
            request: zap_request,
            note_id: None,
        };
        upsert_zap(&state.db, invoice.payment_hash, zap)?;
    }

    Ok(invoice.serialized)
}

pub async fn get_invoice(
    Path(params): Path<InvoiceParams>,
    Query(query): Query<HashMap<String, String>>,
    Extension(state): Extension<State>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let (amount_msats, zap_request) = match query.get("amount").and_then(|a| a.parse::<u64>().ok())
    {
        None => Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "status": "ERROR",
                "reason": "Missing amount parameter",
            })),
        )),
        Some(amount_msats) => {
            let zap_request = query.get("nostr").map_or_else(
                || Ok(None),
                |event_str| {
                    Event::from_json(event_str)
                        .map_err(|_| {
                            (
                                StatusCode::BAD_REQUEST,
                                Json(json!({
                                    "status": "ERROR",
                                    "reason": "Invalid zap request",
                                })),
                            )
                        })
                        .map(Some)
                },
            )?;

            Ok((amount_msats, zap_request))
        }
    }?;

    match get_invoice_impl(state, params, amount_msats, zap_request).await {
        Ok(invoice) => Ok(Json(json!({
            "pr": invoice,
            "routers": []
        }))),
        Err(e) => Err(handle_anyhow_error(e)),
    }
}

pub async fn get_lnurl_pay(
    Path(name): Path<String>,
    Extension(state): Extension<State>,
) -> Result<Json<PayResponse>, (StatusCode, Json<Value>)> {
    let metadata = format!(
        "[[\"text/identifier\",\"{name}@{}\"],[\"text/plain\",\"Sats for {name}\"]]",
        state.config.domain,
    );

    let hash = sha256::Hash::hash(metadata.as_bytes());
    let callback = format!(
        "https://{}/get-invoice/{}/{}",
        state.config.domain,
        hex::encode(hash),
        name
    );

    let resp = PayResponse {
        callback,
        min_sendable: 1_000,
        max_sendable: 11_000_000_000,
        tag: Tag::PayRequest,
        metadata,
        comment_allowed: None,
        allows_nostr: Some(true),
        nostr_pubkey: Some(*state.keys.public_key()),
    };

    Ok(Json(resp))
}

pub(crate) fn handle_anyhow_error(err: anyhow::Error) -> (StatusCode, Json<Value>) {
    let err = json!({
        "status": "ERROR",
        "reason": format!("{err}"),
    });
    (StatusCode::BAD_REQUEST, Json(err))
}

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Serialize)]
struct DiscordWebhookMessage {
    content: Option<String>,
    embeds: Vec<DiscordEmbed>,
}

#[derive(Debug, Serialize)]
struct DiscordEmbed {
    title: String,
    description: String,
    color: i32,
    fields: Vec<DiscordField>,
}

#[derive(Debug, Serialize)]
struct DiscordField {
    name: String,
    value: String,
    inline: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WebhookPayload {
    #[serde(rename = "type")]
    event_type: String,
    timestamp: i64,
    amount_sat: i64,
    payment_hash: String,
    external_id: Option<String>,
    payer_note: Option<String>,
    payer_key: Option<String>,
}

impl WebhookPayload {
    pub async fn send_to_discord(&self, webhook_url: &str) -> Result<()> {
        let client = Client::new();

        // Convert timestamp to readable format
        let datetime = DateTime::from_timestamp(self.timestamp, 0).unwrap_or_else(|| Utc::now());
        let formatted_time = datetime.format("%Y-%m-%d %H:%M:%S UTC").to_string();

        // Create fields for the embed
        let mut fields = vec![
            DiscordField {
                name: "Amount".to_string(),
                value: format!("{} sats", self.amount_sat),
                inline: true,
            },
            DiscordField {
                name: "Payment Hash".to_string(),
                value: self.payment_hash.clone(),
                inline: true,
            },
            DiscordField {
                name: "Timestamp".to_string(),
                value: formatted_time,
                inline: true,
            },
        ];

        // Add optional fields if they exist
        if let Some(external_id) = &self.external_id {
            fields.push(DiscordField {
                name: "External ID".to_string(),
                value: external_id.clone(),
                inline: true,
            });
        }

        if let Some(note) = &self.payer_note {
            fields.push(DiscordField {
                name: "Payer Note".to_string(),
                value: note.clone(),
                inline: true,
            });
        }

        if let Some(key) = &self.payer_key {
            fields.push(DiscordField {
                name: "Payer Key".to_string(),
                value: key.clone(),
                inline: true,
            });
        }

        // Create the embed
        let embed = DiscordEmbed {
            title: format!("New {} Event", self.event_type),
            description: "A payment has been received.".to_string(),
            color: 0x00ff00, // Green color
            fields,
        };

        // Create the final message
        let message = DiscordWebhookMessage {
            content: None,
            embeds: vec![embed],
        };

        // Send the webhook
        client
            .post(webhook_url)
            .json(&message)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}

#[derive(Debug)]
enum WebhookError {
    InvalidSignature,
    ExpiredTimestamp,
    MissingSignature,
}

impl From<WebhookError> for (StatusCode, String) {
    fn from(err: WebhookError) -> Self {
        match err {
            WebhookError::InvalidSignature => {
                (StatusCode::UNAUTHORIZED, "Invalid signature".to_string())
            }
            WebhookError::ExpiredTimestamp => (
                StatusCode::BAD_REQUEST,
                "Webhook timestamp too old".to_string(),
            ),
            WebhookError::MissingSignature => (
                StatusCode::BAD_REQUEST,
                "Missing signature header".to_string(),
            ),
        }
    }
}

pub async fn handle_payments_webhook(
    Extension(state): Extension<State>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<(), (StatusCode, String)> {
    println!("Checking sig");

    dbg!(&state.config);
    // Extract signature from headers
    let signature = headers
        .get("x-phoenix-signature")
        .ok_or(WebhookError::MissingSignature)?
        .to_str()
        .map_err(|_| WebhookError::InvalidSignature)?;

    let secret_bytes = state.config.phoenixd_webhook_secret.as_bytes();

    // Verify the signature
    let decoded_signature = hex::decode(signature).map_err(|_| WebhookError::InvalidSignature)?;
    let mut mac = HmacSha256::new_from_slice(secret_bytes).expect("Invalid secret key");
    mac.update(&body);
    mac.verify_slice(&decoded_signature)
        .map_err(|_| WebhookError::InvalidSignature)?;

    println!("Sig good");
    // If signature is valid, process the payload
    let webhook_payload: WebhookPayload = serde_json::from_slice(&body).map_err(|e| {
        eprintln!("error deserializing: {}", e.to_string());
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to serialize payload".to_string(),
        )
    })?;

    dbg!(&webhook_payload);

    // Verify timestamp
    const TIMESTAMP_TOLERANCE: Duration = Duration::from_secs(5 * 60);
    let event_time = DateTime::<Utc>::from_timestamp(webhook_payload.timestamp / 1000, 0)
        .ok_or(WebhookError::ExpiredTimestamp)?;
    let now = Utc::now();
    if (now - event_time) > chrono::Duration::from_std(TIMESTAMP_TOLERANCE).unwrap() {
        return Err(WebhookError::ExpiredTimestamp.into());
    }

    if let Err(e) = webhook_payload
        .send_to_discord(&state.config.discord_webhook_url)
        .await
    {
        eprintln!(
            "Error sending payment notification to discord webhook: {}",
            e.to_string()
        );
    }

    Ok(())
}
