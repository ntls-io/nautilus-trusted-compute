// https://github.com/mehcode/config-rs/blob/master/examples/hierarchical-env/settings.rs

use config::{Config, ConfigError, File};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Purestake {
    pub indexer_url: String,
    pub indexer_token: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub purestake: Purestake,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name("config/default.toml"))
            // Add in a local configuration file
            // This file shouldn't be checked in to git
            .add_source(File::with_name("config/local.toml").required(false))
            .build()?;

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_deserialize()
    }
}
