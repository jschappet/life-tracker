use std::env;

use config::{Config, ConfigError, Environment, File};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Database {
    pub url: String,
    pub migration: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Gpt {
    pub api_key: String, 
    pub url: String,
    pub max_tokens: u32,
    pub model: String,
    pub temperature: f32,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct WebConfig {
    pub host: String,
    pub port: String,
    pub context_path: String,
    pub cookie_key: String,
    pub login_url: String, 
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Settings {
    pub debug: bool,
    pub database: Database,
    pub web_config: WebConfig,

    pub gpt: Gpt,
    pub templates: String, 
}

impl Settings {
    
    pub fn get_bind(self) -> String {
        format!("{}:{}", self.web_config.host, self.web_config.port)
    } 

    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let s = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name("config/application"))
            // Add in the current environment file
            // Default to 'development' env
            // Note that this file is _optional_
            .add_source(
                File::with_name(&format!("config/{run_mode}"))
                    .required(false),
            )
            // Add in a local configuration file
            // This file shouldn't be checked in to git
            .add_source(Environment::with_prefix("APP_"))
            // You may also programmatically change settings
            //.set_override("database.url", "postgres://")?
            .build()?;

        // Now that we're done, let's access our configuration
        //println!("debug: {:?}", s.get_bool("debug"));
        //println!("database: {:?}", s.get::<String>("database.url"));

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_deserialize()
    }
}