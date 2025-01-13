use crate::config::*;
use crate::routes::*;
use crate::subscriber::start_invoice_subscription;
use axum::http::Method;
use axum::routing::get;
use axum::{http, Extension, Router};
use clap::Parser;
use futures_util::TryFutureExt;
use nostr::prelude::ToBech32;
use nostr::Keys;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{from_reader, to_string};
use sled::Db;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::PathBuf;
use tokio::spawn;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use tower_http::services::ServeFile;

mod config;
mod db;
mod routes;
mod subscriber;

#[derive(Clone)]
pub struct State {
    pub db: Db,
    pub http_client: Client,
    pub keys: Keys,
    pub config: Config,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config: Config = Config::parse();
    let http_client = Client::new();

    // Create the datadir if it doesn't exist
    let path = PathBuf::from(&config.data_dir);
    std::fs::create_dir_all(path.clone())?;

    let db_path = {
        let mut path = path.clone();
        path.push("zaps.db");
        path
    };

    // DB management
    let db = sled::open(&db_path)?;

    let keys_path = {
        let mut path = path.clone();
        path.push("keys.json");
        path
    };

    let keys = get_keys(keys_path);

    let state = State {
        db,
        http_client,
        keys: keys.clone(),
        config: config.clone(),
    };

    // Invoice event stream
    spawn(
        start_invoice_subscription(state.db.clone(), keys, config.clone())
            .map_err(|e| eprintln!("Failed to subscribe to invoice payments: {}", e.to_string())),
    );

    let website_files_path = "./static";

    let website_service = ServeDir::new(website_files_path)
        .precompressed_br()
        .precompressed_gzip()
        .not_found_service(ServeFile::new(format!("{}/404.html", website_files_path)));

    let addr: std::net::SocketAddr = format!("{}:{}", config.bind, &config.port)
        .parse()
        .expect("Failed to parse bind/port for webserver");

    let app = Router::new()
        .route("/get-invoice/{hash}/{name}", get(get_invoice))
        .route("/.well-known/lnurlp/{name}", get(get_lnurl_pay))
        .fallback_service(website_service)
        .layer(Extension(state.clone()))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_headers(vec![http::header::CONTENT_TYPE])
                .allow_methods([Method::GET, Method::POST]),
        );

    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("Webserver running on http://{}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct NostrKeys {
    server_key: String,
}

impl NostrKeys {
    fn generate() -> Self {
        let server_key = Keys::generate();

        NostrKeys {
            server_key: server_key.secret_key().unwrap().to_bech32().unwrap(),
        }
    }
}

fn get_keys(path: PathBuf) -> Keys {
    match File::open(&path) {
        Ok(file) => {
            let reader = BufReader::new(file);
            let n: NostrKeys = from_reader(reader).expect("Could not parse JSON");

            Keys::parse(n.server_key).expect("Could not parse key")
        }
        Err(_) => {
            let keys = NostrKeys::generate();
            let json_str = to_string(&keys).expect("Could not serialize data");

            let mut file = File::create(path).expect("Could not create file");
            file.write_all(json_str.as_bytes())
                .expect("Could not write to file");

            Keys::parse(&keys.server_key).expect("Could not parse key")
        }
    }
}
