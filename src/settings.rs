use clap::App;
use config::{Config, ConfigError, File};
use serde_derive::Deserialize;

use crate::bitcoin::Network;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub bind_public: String,
    pub bind_private: String,
    pub payment_url: String,
    pub node_ip: String,
    pub rpc_port: u16,
    pub rpc_username: String,
    pub rpc_password: String,
    pub zmq_port: u16,
    pub secret: String,
    pub sql: Sql,
    pub network: Network,
}

#[derive(Debug, Deserialize)]
pub struct Sql {
    pub prefix: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub db: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        // Set defaults
        let yaml = load_yaml!("cli.yml");
        let matches = App::from_yaml(yaml).get_matches();
        let home_dir = match dirs::home_dir() {
            Some(some) => some,
            None => return Err(ConfigError::Message("no home directory".to_string())),
        };
        s.set_default("bind_public", "127.0.0.1:8081").unwrap();
        s.set_default("bind_private", "127.0.0.1:8900").unwrap();
        s.set_default("payment_url", "http://127.0.0.1:8081/payment/")
            .unwrap();
        s.set_default("node_ip", "127.0.0.1").unwrap();
        s.set_default("rpc_port", "18443").unwrap();
        s.set_default("rpc_username", "username").unwrap();
        s.set_default("rpc_password", "password").unwrap();
        s.set_default("zmq_port", "28332").unwrap();
        s.set_default("secret", "secret").unwrap();
        s.set_default("sql.prefix", "postgresql").unwrap();
        s.set_default("sql.host", "127.0.0.1").unwrap();
        s.set_default("sql.username", "postgres").unwrap();
        s.set_default("sql.password", "password").unwrap();
        s.set_default("sql.port", "5432").unwrap();
        s.set_default("sql.db", "postgres").unwrap();
        s.set_default("network", "regnet").unwrap();

        // Load config from file
        let mut default_config = home_dir.clone();
        default_config.push(".bip70-server/config");
        let default_config_str = default_config.to_str().unwrap();
        let config_path = matches.value_of("config").unwrap_or(default_config_str);
        s.merge(File::with_name(config_path).required(false))?;

        // Set public bind address from cmd line
        if let Some(bind_public) = matches.value_of("bind-public") {
            s.set("bind_public", bind_public)?;
        }

        // Set private bind address from cmd line
        if let Some(bind_private) = matches.value_of("bind-private") {
            s.set("bind_private", bind_private)?;
        }

        // Payment URL
        if let Some(payment_url) = matches.value_of("payment-url") {
            s.set("payment_url", payment_url)?;
        }

        // Set node IP from cmd line
        if let Some(node_ip) = matches.value_of("node-ip") {
            s.set("node_ip", node_ip)?;
        }

        // Set rpc port from cmd line
        if let Ok(rpc_port) = value_t!(matches, "rpc-port", i64) {
            s.set("rpc_port", rpc_port)?;
        }

        // Set rpc username from cmd line
        if let Some(rpc_username) = matches.value_of("rpc-username") {
            s.set("rpc_username", rpc_username)?;
        }

        // Set rpc password from cmd line
        if let Some(rpc_password) = matches.value_of("rpc-password") {
            s.set("rpc_password", rpc_password)?;
        }

        // Set zmq port from cmd line
        if let Ok(node_zmq_port) = value_t!(matches, "zmq-port", i64) {
            s.set("zmq_port", node_zmq_port)?;
        }

        // Set secret from cmd line
        if let Some(secret) = matches.value_of("secret") {
            s.set("secret", secret)?;
        }

        // TODO: Database from commandline

        s.try_into()
    }
}
