use crate::db::{get_zap, upsert_zap};
use crate::Config;
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

use tokio_tungstenite::{connect_async, tungstenite::http::Request};
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
    loop {
        println!("Starting invoice subscription");

        let url = format!(
            "ws://{}:{}/websocket",
            config.phoenixd_host, config.phoenixd_port
        );

        // Create the request with authentication header
        let request = Request::builder()
            .uri(url)
            .header("Sec-WebSocket-Protocol", &config.phoenixd_password)
            .body(())?;

        // Connect to the WebSocket server
        let (mut ws_stream, _) = connect_async(request).await?;

        println!("WebSocket connection established");

        // Handle incoming messages
        while let Some(msg) = ws_stream.next().await {
            match msg {
                Ok(msg) => {
                    if msg.is_text() {
                        let text = msg.into_text()?;
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
                                            hex::encode(payment.payment_hash),
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
                                                eprintln!("Timeout");
                                            }
                                        }
                                    });
                                }
                            }
                            Err(e) => eprintln!("Error parsing payment: {}", e),
                        }
                    }
                }
                Err(e) => {
                    println!("Error receiving message: {}", e);
                    break;
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
