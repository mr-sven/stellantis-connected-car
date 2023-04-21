use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkElement {
    pub href: String,
    pub templated: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListResponse<T> {
    #[serde(alias = "_links")]
    pub links: HashMap<String, LinkElement>,
    pub total: u32,
    #[serde(alias = "_embedded")]
    pub embedded: T,
    pub current_page: u32,
    pub total_page: u32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VehiclesList {
    pub vehicles: Vec<VehiclesListElement>
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VehiclesListElement {
    pub id: String,
    pub vin: String,
    pub brand: String,
    pub pictures: Vec<String>,
    #[serde(alias = "_links")]
    pub links: HashMap<String, LinkElement>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VehicleStatus {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_position: VehiclePosition,
    pub ignition: VehicleIgnition,
    pub battery: VehicleBattery,
    pub privacy: VehiclePrivacy,
    pub service: VehicleService,
    pub environment: VehicleEnvironment,
    pub odometer: VehicleOdometer,
    pub kinetic: VehicleKinetic,
    #[serde(alias = "_links")]
    pub links: HashMap<String, LinkElement>,
    pub preconditioning: VehiclePreconditioning,
    pub energies: Vec<VehicleEnergy>,
    pub preconditionning: VehiclePreconditioning,
    pub energy: Vec<VehicleEnergy>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VehiclePosition {
    #[serde(alias = "type")]
    pub _type: String,
    pub geometry: PositionGeometry,
    pub properties: PositionProperties
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionGeometry {
    #[serde(alias = "type")]
    pub _type: String,
    pub coordinates: Vec<f32>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionProperties {
    #[serde(alias = "type")]
    pub _type: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VehicleIgnition {
    #[serde(alias = "type")]
    pub _type: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VehicleBattery {
    #[serde(alias = "type")]
    pub voltage: f32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VehiclePrivacy {
    #[serde(alias = "type")]
    pub state: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VehicleService {
    #[serde(alias = "type")]
    pub _type: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VehicleEnvironment {
    pub luminosity: EnvironmentLuminosity,
    pub air: EnvironmentAir,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EnvironmentLuminosity {
    pub day: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EnvironmentAir {
    pub temp: f32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VehicleOdometer {
    pub mileage: f32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VehicleKinetic {
    pub moving: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VehiclePreconditioning {
    pub air_conditioning: AirConditioning,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AirConditioning {
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VehicleEnergy {
    pub created_at: DateTime<Utc>,
    #[serde(alias = "type")]
    pub _type: String,
    pub level: u32,
    pub sub_type: Option<String>,
    pub autonomy: Option<u32>,
    pub extension: Option<EnergyExtension>,
    pub charging: Option<EnergyCharging>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EnergyExtension {
    pub electric: EnergyElectric,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EnergyElectric {
    pub battery: EnergyBattery,
    pub charging: EnergyCharging,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EnergyBattery {
    pub load: EnergyBatteryLoad,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EnergyBatteryLoad {
    pub created_at: DateTime<Utc>,
    pub capacity: u32,
    pub residual: u32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EnergyCharging {
    pub plugged: bool,
    pub status: String,
    pub remaining_time: String,
    pub charging_rate: u32,
    pub charging_mode: String,
    pub next_delayed_time: String,
}
