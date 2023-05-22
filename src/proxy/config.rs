use config::{Config as _Config, ConfigError, File};
use log::info;
use serde::Deserialize;
use std::env;
use std::net::IpAddr;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct ProxyConfig {
    #[serde(default = "ProxyConfig::default_host")]
    pub host: IpAddr,
    #[serde(default = "ProxyConfig::default_port")]
    pub port: u16,
}

impl ProxyConfig {
    fn default_host() -> IpAddr {
        "127.0.0.1".parse().unwrap()
    }

    fn default_port() -> u16 {
        8080
    }
}

impl Default for ProxyConfig {
    fn default() -> Self {
        ProxyConfig {
            host: ProxyConfig::default_host(),
            port: ProxyConfig::default_port(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub proxy: ProxyConfig,
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        let path = env::var("RS_MITM_CONFIG").unwrap_or_else(|_| "config.json".into());

        let conf = _Config::builder()
            .add_source(File::with_name(&path))
            .build()?;

        info!(
            "Configuration read from \"{}\"",
            Path::new(&path).canonicalize().unwrap().display()
        );

        conf.try_deserialize()
    }
}
