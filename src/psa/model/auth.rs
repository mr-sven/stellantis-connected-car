use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_with::skip_serializing_none;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldValue {
    pub value: String
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAccessTokenRequest {
    pub site_code: String,
    pub culture: String,
    pub action: String,
    pub fields: HashMap<String, FieldValue>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetAccessTokenResponse {
    pub return_code: String,
    pub access_token: Option<String>,
}

#[derive(Serialize)]
pub struct GetUserRequest {
    pub site_code: String,
    pub ticket: String,
}

#[derive(Deserialize, Debug)]
pub struct GetUserResponse {
    pub errors: Option<HashMap<String, String>>,
    pub success: Option<User>,
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub language: String,
    pub country: String,
    pub profile: UserProfile,
    // dealers,
    pub vehicles: Vec<Vehicle>,
    pub settings_update: u64,
    pub terms_service: HashMap<String, String>,
    // early_adopter
    pub service_state: Vec<String>,
    pub cgu_validation: bool
}

#[derive(Deserialize, Debug)]
pub struct UserProfile {
    pub email: String,
    pub civility: String,
    pub last_name: String,
    pub first_name: String,
}

#[derive(Deserialize, Debug)]
pub struct Vehicle {
    pub vin: String,
    pub lcdv: String,
    pub short_label: String,
    pub warranty_start_date: u64,
    pub visual: String,
    pub eligibility: Vec<String>,
    pub attributes: Vec<String>,
    pub type_vehicle: u32,
    pub external_ws_status: String,
    pub mileage: VehicleMileage,
}

#[derive(Deserialize, Debug)]
pub struct VehicleMileage {
    pub value: u64,
    pub source: u64,
    pub timestamp: u64,
}

#[skip_serializing_none]
#[derive(Serialize)]
pub struct TokenRequest {
    pub realm: Option<String>,
    pub grant_type: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub scope: String,
    pub refresh_token: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct TokenResponse {
    pub scope: String,
    pub expires_in: u32,
    pub token_type: String,
    pub refresh_token: String,
    pub id_token: String,
    pub access_token: String,
}