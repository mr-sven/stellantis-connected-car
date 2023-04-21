use reqwest::header::{USER_AGENT, CONTENT_TYPE};
use serde::de::DeserializeOwned;
use std::{collections::HashMap, fmt, error::Error, cell::RefCell};
use chrono::{Duration, Utc};

use super::model::*;

const APP_VERSION: &str = "1.33.0";

#[derive(Debug)]
pub struct ApiError {
    pub message: String
}

impl Error for ApiError {}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Api Error: {}", self.message)
    }
}

pub fn request_access_token(
        host_brandid_prod: &String,
        site_code: &String,
        client_email: &String,
        client_password: &String) -> Result<String, Box<dyn Error>> {
    let req = GetAccessTokenRequest {
        site_code: site_code.to_owned(),
        culture: "fr-FR".to_owned(),
        action: "authenticate".to_owned(),
        fields: HashMap::from([
            ("USR_EMAIL".to_owned(), FieldValue {value: client_email.to_owned()}),
            ("USR_PASSWORD".to_owned(), FieldValue {value: client_password.to_owned()})
        ])
    };

    let params = [
        ("jsonRequest", serde_json::to_string(&req).unwrap()),
    ];
    let url = reqwest::Url::parse_with_params(&format!("{}/GetAccessToken", host_brandid_prod), &params)?;
    let client = reqwest::blocking::Client::new();
    let res = client.post(url)
        .header(USER_AGENT, "okhttp/2.3.0")
        .header(CONTENT_TYPE, "application/json")
        .send()?;

    let token_response = res.json::<GetAccessTokenResponse>()?;

    if token_response.return_code.eq("OK") {
        Ok(token_response.access_token.unwrap())
    }
    else {
        Err(Box::new(ApiError { message: "GetAccessToken Error".to_owned()}))
    }
}

pub fn request_customer_id(
        brand_code: &String,
        culture: &String,
        site_code: &String,
        token: &String,
        cert: &String,
        key: &String) -> Result<String, Box<dyn Error>> {
    let req = GetUserRequest {
        site_code: site_code.to_owned(),
        ticket: token.to_owned(),
    };

    let params = [
        ("culture", culture),
        ("width", &"1080".to_owned()),
        ("version", &APP_VERSION.to_owned()),
    ];

    let url = reqwest::Url::parse_with_params(&format!("https://mw-{}-m2c.mym.awsmpsa.com/api/v1/user", brand_code.to_lowercase()), &params)?;
    let identity = reqwest::Identity::from_pkcs8_pem(&cert.as_bytes(), &key.as_bytes())?;

    let client = reqwest::blocking::Client::builder()
        .identity(identity)
        .build()?;

    let res = client.post(url)
        .header(CONTENT_TYPE, "application/json;charset=UTF-8")
        .json(&req)
        .header("Source-Agent", "App-Android")
        .header("Token", token)
        .header("Version", APP_VERSION)
        .header(USER_AGENT, "okhttp/4.8.0")
        .send()?;

    let user_response = res.json::<GetUserResponse>()?;

    match user_response.success {
        Some(user) => Ok(user.id),
        None => Err(Box::new(ApiError { message: format!("Request User Error{:?}", user_response.errors)}))
    }
}

pub struct ApiClient<'a> {
    config: &'a RefCell<ApiConfig>
}

impl<'a> ApiClient<'a> {
    pub fn new(config: &'a RefCell<ApiConfig>) -> ApiClient<'a> {
        ApiClient {
            config: config.clone()
        }
    }

    pub fn token_request(&mut self) -> Result<(), Box<dyn Error>> {

        let mut config = self.config.borrow_mut();
        if let Some(exp) = config.token_expires {
            if exp > Utc::now() {
                return Ok(());
            }
        }

        let req = if config.refresh_token.is_empty() {
            TokenRequest {
                realm: Some(config.realm.to_owned()),
                grant_type: "password".to_owned(),
                password: Some(config.client_password.to_owned()),
                username: Some(config.client_email.to_owned()),
                scope: "profile openid".to_owned(),
                refresh_token: None
            }
        } else {
            TokenRequest {
                realm: Some(config.realm.to_owned()),
                grant_type: "refresh_token".to_owned(),
                password: None,
                username: None,
                scope: "profile openid".to_owned(),
                refresh_token: Some(config.refresh_token.to_owned()),
            }
        };

        let client = reqwest::blocking::Client::new();
        let res = client.post(config.oauth_url.to_owned())
            .form(&req)
            .basic_auth(config.client_id.to_owned(), Some(config.client_secret.to_owned()))
            .header("Source-Agent", "App-Android")
            .header("Version", APP_VERSION)
            .header(USER_AGENT, "okhttp/4.8.0")
            .send()?;

        let auth_response = res.json::<TokenResponse>()?;
        config.refresh_token = auth_response.refresh_token.to_owned();
        config.access_token = auth_response.access_token.to_owned();
        config.token_expires = Some(Utc::now() + Duration::seconds(auth_response.expires_in as i64));

        Ok(())
    }

    fn check_token(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(exp) = self.config.borrow().token_expires {
            if exp > Utc::now() {
                return Ok(());
            }
        }
        self.token_request()?;
        Ok(())
    }

    fn get_list<T>(&mut self, path: String) -> Result<ListResponse<T>, Box<dyn Error>> where T: DeserializeOwned {
        self.check_token()?;
        let config = self.config.borrow();

        let params = [
            ("client_id", config.client_id.to_owned()),
        ];

        let url = reqwest::Url::parse_with_params(format!("{}/{}", config.host_api_prod, path).as_str(), &params)?;
        let client = reqwest::blocking::Client::new();
        let res = client.get(url)
            .bearer_auth(config.access_token.to_owned())
            .header("x-introspect-realm", config.realm.to_owned())
            .send()?;

        Ok(res.json::<ListResponse<T>>()?)
    }

    fn get_item<T>(&mut self, path: String) -> Result<T, Box<dyn Error>> where T: DeserializeOwned {
        self.check_token()?;
        let config = self.config.borrow();

        let params = [
            ("client_id", config.client_id.to_owned()),
        ];

        let url = reqwest::Url::parse_with_params(format!("{}/{}", config.host_api_prod, path).as_str(), &params)?;
        let client = reqwest::blocking::Client::new();
        let res = client.get(url)
            .bearer_auth(config.access_token.to_owned())
            .header("x-introspect-realm", config.realm.to_owned())
            .send()?;

        Ok(res.json::<T>()?)
    }

    pub fn connectedcar_list_vehicles(&mut self) -> Result<ListResponse<VehiclesList>, Box<dyn Error>> {
        Ok(self.get_list::<VehiclesList>("connectedcar/v4/user/vehicles".to_string())?)
    }

    pub fn connectedcar_get_vehicle_status(&mut self, id: &String) -> Result<VehicleStatus, Box<dyn Error>> {
        Ok(self.get_item::<VehicleStatus>(format!("connectedcar/v4/user/vehicles/{}/status", id))?)
    }

}