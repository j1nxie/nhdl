use config::{
    Config,
    ConfigError,
    Environment,
    File,
};

use serde::Deserialize;

const CONFIG_FILE_PATH: &str = "./config.toml";

#[derive(Clone, Debug, Deserialize)]
pub struct Settings {
    pub path: String,
    pub proxy: String,
    pub proxy_username: String,
    pub proxy_password: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();
        s.merge(File::with_name(CONFIG_FILE_PATH))?;
        s.merge(Environment::with_prefix("nhdl").separator("__"))?;
        
        s.try_into()
    }
}
