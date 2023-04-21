use serde::{Deserialize, Serialize};
use std::{fs::{File, OpenOptions}, cell::RefCell};

use crate::psa::model::ApiConfig;

pub trait YamlConfigFile<T> {
    fn from_file(filename: String) -> Result<T, Box<dyn std::error::Error>>;
    fn to_file(&self, filename: String) -> Result<(), Box<dyn std::error::Error>>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub api: RefCell<ApiConfig>,
    pub cert: String,
    pub key: String,
    pub host_brandid_prod: String,
    pub site_code: String,
    pub culture: String,
    pub brand_code: String,
    pub customer_id: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            api: RefCell::new(ApiConfig::default()),
            cert: "".to_string(),
            key: "".to_string(),
            host_brandid_prod: "".to_string(),
            site_code: "".to_string(),
            culture: "".to_string(),
            brand_code: "".to_string(),
            customer_id: "".to_string(),
        }
    }
}

impl YamlConfigFile<AppConfig> for AppConfig {
    fn from_file(filename: String) -> Result<AppConfig, Box<dyn std::error::Error>> {
        match File::open(filename) {
            Ok(f) => {
                let cfg: AppConfig = serde_yaml::from_reader(f)?;
                return Ok(cfg)
            },
            Err(_) => return Ok(AppConfig::default()),
        };
    }

    fn to_file(&self, filename: String) -> Result<(), Box<dyn std::error::Error>> {
        let f = OpenOptions::new().write(true).create(true).open(filename)?;
        serde_yaml::to_writer(f, &self)?;
        Ok(())
    }
}