use chrono::{serde::ts_seconds_option, DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiConfig {
    pub realm: String,
    pub oauth_url: String,
    pub host_api_prod: String,
    pub client_id: String,
    pub client_secret: String,
    pub client_email: String,
    pub client_password: String,
    pub refresh_token: String,
    pub access_token: String,
    #[serde(default)]
    #[serde(with = "ts_seconds_option")]
    pub token_expires: Option<DateTime<Utc>>
}

impl Default for ApiConfig {
    fn default() -> Self {
        ApiConfig {
            realm: "".to_string(),
            oauth_url: "".to_string(),
            host_api_prod: "".to_string(),
            client_id: "".to_string(),
            client_secret: "".to_string(),
            client_email: "".to_string(),
            client_password: "".to_string(),
            refresh_token: "".to_string(),
            access_token: "".to_string(),
            token_expires: None,
        }
    }
}