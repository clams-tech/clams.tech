use crate::db::{upsert_zap, Zap};
use crate::State;
use anyhow::anyhow;
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::{Extension, Json};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use bitcoin::hashes::{sha256, Hash};
use lightning_invoice::Bolt11Invoice;
use lnurl::pay::PayResponse;
use lnurl::Tag;
use nostr::{Event, JsonUtil};
use reqwest::header::AUTHORIZATION;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::str::FromStr;

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
