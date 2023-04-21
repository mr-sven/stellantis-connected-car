mod config;
mod parser;
mod apk_parser;
mod psa;

use std::fs::{File, OpenOptions};

use psa::api::{request_access_token, request_customer_id, ApiClient};
use config::YamlConfigFile;
use apk_parser::APK;
use parser::FromFile;

const CARS_FILE: &str = "cars.yaml";
const CONFIG_FILE: &str = "config.yaml";

fn update_config_from_apk(cfg: &mut config::AppConfig, apk: &APK) {
    let mut api_config = cfg.api.borrow_mut();
    api_config.client_id = apk.cvs_client_id.clone();
    api_config.client_secret = apk.cvs_secret.clone();
    api_config.host_api_prod = apk.host_api_prod.clone();
    api_config.realm = apk.realm.clone();
    api_config.oauth_url = apk.oauth_url.clone();
    cfg.cert = apk.cert.clone();
    cfg.key = apk.key.clone();
    cfg.host_brandid_prod = apk.host_brandid_prod.clone();
    cfg.site_code = apk.site_code.clone();
    cfg.culture = apk.culture.clone();
    cfg.brand_code = apk.brand_code.clone();
}

fn check_config () -> Result<config::AppConfig, Box<dyn std::error::Error>> {
    let mut cfg = config::AppConfig::from_file(CONFIG_FILE.to_string())?;
    if cfg.api.borrow().client_id.is_empty() {
        println!("Please provide Car APK path: ");
        let mut car_apk_path = String::new();
        std::io::stdin().read_line(&mut car_apk_path)?;
        car_apk_path = car_apk_path.trim().to_string();
        let apk = APK::from_file(car_apk_path)?;
        update_config_from_apk(&mut cfg, &apk);
        cfg.customer_id = "".to_string();
    }

    if cfg.api.borrow().client_email.is_empty() {
        let mut input = String::new();
        println!("Please client E-Mail: ");
        std::io::stdin().read_line(&mut input)?;
        cfg.api.borrow_mut().client_email = input.trim().to_owned();

        input = String::new();
        println!("Please client Password: ");
        std::io::stdin().read_line(&mut input)?;
        cfg.api.borrow_mut().client_password = input.trim().to_owned();
    }

    if cfg.customer_id.is_empty() {
        let access_token = request_access_token(&cfg.host_brandid_prod, &cfg.site_code, &cfg.api.borrow().client_email, &cfg.api.borrow().client_password)?;
        cfg.customer_id = request_customer_id(&cfg.brand_code, &cfg.culture, &cfg.site_code, &access_token, &cfg.cert, &cfg.key)?;
    }

    cfg.to_file(CONFIG_FILE.to_string())?;
    Ok(cfg)
}

fn load_cars() -> Option<psa::model::VehiclesList> {
    if std::path::Path::new(&CARS_FILE.to_string()).exists() {
        if let Ok(f) = File::open(CARS_FILE.to_string()) {
            if let Ok(cars) = serde_yaml::from_reader::<File, psa::model::VehiclesList>(f) {
                return Some(cars);
            }
        }
    }
    None
}

fn save_cars(cars: &psa::model::VehiclesList) {
    if let Ok(f) = OpenOptions::new().write(true).create(true).open(&CARS_FILE.to_string()) {
        serde_yaml::to_writer(f, cars).unwrap();
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = check_config()?;
    let mut client = ApiClient::new(&cfg.api);

    let mut cars = load_cars();
    if let None = cars {
        let res = client.connectedcar_list_vehicles()?;
        cars = Some(res.embedded);
        save_cars(&cars.as_ref().unwrap());
    }

    let mut input = String::new();
    println!("Enter VIN of car to get status: ");
    std::io::stdin().read_line(&mut input)?;
    let vin = input.trim();

    if let Some(car) = cars.unwrap().vehicles.iter().find(|c| c.vin.eq(vin)) {
        let res = client.connectedcar_get_vehicle_status(&car.id)?;
        dbg!(res);
    }
    cfg.to_file(CONFIG_FILE.to_string())?;

    Ok(())
}
