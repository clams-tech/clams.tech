use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(version, author, about)]
/// A simple LNURL pay server. Allows you to have a lightning address for your own node.
pub struct Config {
    #[arg(default_value = ".", env = "DATA_DIR", long)]
    /// Location of database and keys files
    pub data_dir: String,
    #[arg(default_value = "0.0.0.0", long)]
    /// Bind address for lnurl-server's webserver
    pub bind: String,
    #[arg(default_value = "8080", long)]
    /// Port for lnurl-server's webserver
    pub port: u16,
    #[arg(long, env = "PHOENIXD_HOST")]
    /// Host of the Phoenixd server
    pub phoenixd_host: String,
    #[arg(default_value = "9740", long)]
    /// Port of the Phoenixd server
    pub phoenixd_port: u32,
    #[arg(long, env = "PHOENIXD_PASSWORD")]
    /// Password for Phoenixd server
    pub phoenixd_password: String,
    #[arg(long, env = "PHOENIXD_WEBHOOK_SECRET")]
    /// Webhook secret for Phoenixd server to verify webhook payloads
    pub phoenixd_webhook_secret: String,
    #[arg(long, env = "DOMAIN")]
    pub domain: String,
    #[arg(long, env = "DISCORD_WEBHOOK_URL")]
    pub discord_webhook_url: String,
}
