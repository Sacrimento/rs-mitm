use std::env;
use config::{Config, ConfigError, File};
use serde::Deserialize;
use std::net::IpAddr;

#[derive(Debug, Deserialize)]
pub struct ProxySettings {
    #[serde(default = "ProxySettings::default_host")]
    pub host: String,
    #[serde(default = "ProxySettings::default_port")]
    pub port: u16,
}

impl ProxySettings {
    fn default_host() -> String { "127.0.0.1".into() }
    fn default_port() -> u16 { 8080 }

    pub fn host_as_ip(&self) -> IpAddr {
        // TODO: Check that the host is a *valid* IP
        self.host.parse().unwrap()
    }
}

impl Default for ProxySettings {
    fn default() -> Self {
        ProxySettings {
            host: ProxySettings::default_host(),
            port: ProxySettings::default_port(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub proxy: ProxySettings,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let path = env::var("RS_MITM_CONFIG").unwrap_or_else(|_| "config.json".into());

        let conf = Config::builder()
            .add_source(File::with_name(&path))
            .build()?;

        conf.try_deserialize()
    }
}
