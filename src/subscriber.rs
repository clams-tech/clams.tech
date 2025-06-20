use crate::db::{get_zap, upsert_zap};
use crate::Config;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use bitcoin::hashes::sha256::Hash as Sha256;
use bitcoin::hashes::Hash;
use bitcoin::key::Secp256k1;
use bitcoin::secp256k1::SecretKey;
use futures_util::StreamExt;
use lightning_invoice::{Currency, InvoiceBuilder, PaymentSecret};
use nostr::prelude::ToBech32;
use nostr::{EventBuilder, Keys};
use nostr_sdk::Client;
use serde::Deserialize;
use sled::Db;
use std::time::Duration;
use std::cmp::min;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;

const RELAYS: [&str; 8] = [
    "wss://nostr.mutinywallet.com",
    "wss://relay.snort.social",
    "wss://relay.nostr.band",
    "wss://eden.nostr.land",
    "wss://nos.lol",
    "wss://nostr.fmt.wiz.biz",
    "wss://relay.damus.io",
    "wss://nostr.wine",
];

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum PaymentNotificationType {
    PaymentReceived,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PaymentNotification {
    #[serde(rename = "type")]
    pub notification_type: PaymentNotificationType,
    // amount_sat: u64,
    pub payment_hash: String,
    // timestamp: u64,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // external_id: Option<String>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // payer_note: Option<String>,
}

pub async fn start_invoice_subscription(db: Db, key: Keys, config: Config) -> anyhow::Result<()> {
    let mut reconnect_delay = Duration::from_secs(1);
    let max_reconnect_delay = Duration::from_secs(60);
    let mut consecutive_failures = 0;
    
    loop {
        println!("Starting invoice subscription (attempt {})", consecutive_failures + 1);

        let host = format!("{}:{}", config.phoenixd_host, config.phoenixd_port);
        let url = format!("ws://{}/websocket", &host);
        let auth_string = format!(":{}", config.phoenixd_password);
        let auth_header = format!("Basic {}", BASE64.encode(auth_string.as_bytes()));

        let mut request = url.into_client_request().expect("Valid URL");

        request.headers_mut().insert(
            "Authorization",
            auth_header.parse().expect("auth header can be parsed"),
        );

        // Attempt to connect to the WebSocket server
        match connect_async(request).await {
            Ok((mut ws_stream, _)) => {
                println!("WebSocket connection established");
                consecutive_failures = 0;
                reconnect_delay = Duration::from_secs(1);

                // Handle incoming messages
                let mut connection_broken = false;
                while let Some(msg) = ws_stream.next().await {
                    match msg {
                        Ok(msg) => {
                            if msg.is_text() {
                                let text = match msg.into_text() {
                                    Ok(text) => text,
                                    Err(e) => {
                                        eprintln!("Error converting message to text: {}", e);
                                        continue;
                                    }
                                };
                                
                                match serde_json::from_str::<PaymentNotification>(&text) {
                                    Ok(payment) => {
                                        if matches!(
                                            payment.notification_type,
                                            PaymentNotificationType::PaymentReceived
                                        ) {
                                            let db = db.clone();
                                            let key = key.clone();
                                            tokio::spawn(async move {
                                                let fut = handle_paid_invoice(
                                                    &db,
                                                    payment.payment_hash,
                                                    key.clone(),
                                                );

                                                match tokio::time::timeout(Duration::from_secs(30), fut)
                                                    .await
                                                {
                                                    Ok(Ok(_)) => {
                                                        println!("Handled paid invoice!");
                                                    }
                                                    Ok(Err(e)) => {
                                                        eprintln!("Failed to handle paid invoice: {}", e);
                                                    }
                                                    Err(_) => {
                                                        eprintln!("Timeout handling paid invoice");
                                                    }
                                                }
                                            });
                                        }
                                    }
                                    Err(e) => eprintln!("Error parsing payment notification: {}", e),
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("WebSocket error: {}", e);
                            connection_broken = true;
                            break;
                        }
                    }
                }

                if connection_broken {
                    println!("WebSocket connection lost, attempting to reconnect...");
                }
            }
            Err(e) => {
                consecutive_failures += 1;
                eprintln!("Failed to connect to WebSocket (attempt {}): {}", consecutive_failures, e);
                
                // Apply exponential backoff with jitter
                if consecutive_failures > 1 {
                    println!("Waiting {} seconds before reconnecting...", reconnect_delay.as_secs());
                    tokio::time::sleep(reconnect_delay).await;
                    
                    // Exponential backoff: double the delay, but cap it at max_reconnect_delay
                    reconnect_delay = min(reconnect_delay * 2, max_reconnect_delay);
                }
            }
        }
    }
}

async fn handle_paid_invoice(db: &Db, payment_hash: String, keys: Keys) -> anyhow::Result<()> {
    match get_zap(db, payment_hash.clone())? {
        None => Ok(()),
        Some(mut zap) => {
            if zap.note_id.is_some() {
                return Ok(());
            }

            let preimage = zap.request.id.to_bytes();
            let invoice_hash = Sha256::hash(&preimage);

            let payment_secret = zap.request.id.to_bytes();

            let private_key = SecretKey::from_hashed_data::<Sha256>(zap.request.id.as_bytes());

            let amt_msats = zap
                .invoice
                .amount_milli_satoshis()
                .expect("Invoice must have an amount");

            let fake_invoice = InvoiceBuilder::new(Currency::Bitcoin)
                .amount_milli_satoshis(amt_msats)
                .invoice_description(zap.invoice.description())
                .current_timestamp()
                .payment_hash(invoice_hash)
                .payment_secret(PaymentSecret(payment_secret))
                .min_final_cltv_expiry_delta(144)
                .basic_mpp()
                .build_signed(|hash| {
                    Secp256k1::signing_only().sign_ecdsa_recoverable(hash, &private_key)
                })?;

            let event = EventBuilder::zap_receipt(
                fake_invoice.to_string(),
                Some(hex::encode(preimage)),
                zap.request.clone(),
            )
            .to_event(&keys)?;

            // Create new client
            let client = Client::new(&keys);
            client.add_relays(RELAYS).await?;
            client.connect().await;

            let event_id = client.send_event(event).await?;
            let _ = client.disconnect().await;

            println!(
                "Broadcasted event id: {}!",
                event_id.to_bech32().expect("bech32")
            );

            zap.note_id = Some(event_id.to_bech32().expect("bech32"));
            upsert_zap(db, payment_hash, zap)?;

            Ok(())
        }
    }
}
