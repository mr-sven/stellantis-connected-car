use phf::phf_map;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Cursor, Read, Seek};
use std::{error::Error, fmt};
use zip::read::ZipArchive;

use openssl::pkcs12::Pkcs12;

use crate::parser::FromFile;

struct BrandProperties {
    realm: &'static str,
    oauth_url: &'static str
}

static BRAND: phf::Map<&'static str, BrandProperties> = phf_map! {
    "com.psa.mym.myopel" => BrandProperties {realm: "clientsB2COpel", oauth_url: "https://idpcvs.opel.com/am/oauth2/access_token"},
    "com.psa.mym.mypeugeot" => BrandProperties {realm: "clientsB2CPeugeot", oauth_url: "https://idpcvs.peugeot.com/am/oauth2/access_token"},
    "com.psa.mym.mycitroen" => BrandProperties {realm: "clientsB2CCitroen", oauth_url: "https://idpcvs.citroen.com/am/oauth2/access_token"},
    "com.psa.mym.myds" => BrandProperties {realm: "clientsB2CDS", oauth_url: "https://idpcvs.driveds.com/am/oauth2/access_token"},
    "com.psa.mym.myvauxhall" => BrandProperties {realm: "clientsB2CVauxhall", oauth_url: "https://idpcvs.vauxhall.co.uk/am/oauth2/access_token"}
};


#[derive(Debug)]
pub struct ApkParserError {
    pub message: String
}

impl Error for ApkParserError {}

impl fmt::Display for ApkParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Apk Parser Error: {}", self.message)
    }
}

#[derive(Debug)]
pub struct APK {
    pub cvs_client_id: String,
    pub cvs_secret: String,
    pub name: String,
    pub cert: String,
    pub key: String,
    pub host_brandid_prod: String,
    pub host_api_prod: String,
    pub site_code: String,
    pub culture: String,
    pub brand_code: String,
    pub realm: String,
    pub oauth_url: String,
}

impl Default for APK {
    fn default() -> Self {
        APK {
            cvs_client_id: "".to_owned(),
            cvs_secret: "".to_owned(),
            name: "".to_owned(),
            cert: "".to_owned(),
            key: "".to_owned(),
            host_brandid_prod: "".to_owned(),
            host_api_prod: "".to_owned(),
            site_code: "".to_owned(),
            culture: "".to_owned(),
            brand_code: "".to_owned(),
            realm: "".to_owned(),
            oauth_url: "".to_owned(),
        }
    }
}

impl FromFile<APK> for APK {
    fn from_file(filename: String) -> Result<APK, Box<dyn std::error::Error>> {
        let f = File::open(filename)?;
        let mut archive = ZipArchive::new(f)?;
        let mut apk = APK::default();

        parse_parameters(&mut archive, &mut apk)?;
        parse_client_cert(&mut archive, &mut apk)?;
        parse_resources(&mut archive, &mut apk)?;

        Ok(apk)
    }
}

fn parse_resources<R: Read + Seek>(archive: &mut ZipArchive<R>, apk: &mut APK) -> Result<(), Box<dyn std::error::Error>> {

    let mut res_buf = vec![];
    archive.by_name("resources.arsc")?.read_to_end(&mut res_buf)?;
    let res_reader = Cursor::new(res_buf);
    let arsc = arsc::parse_from(res_reader)?;

    let main_package = arsc.get_main_package().unwrap();
    apk.host_brandid_prod = arsc.get_string(&main_package.name, "HOST_BRANDID_PROD".to_owned()).unwrap().to_owned();
    apk.host_api_prod = arsc.get_string(&main_package.name, "HOST_PSA_API_PROD".to_owned()).unwrap().to_owned();

    let country_code = apk.culture.split("-").collect::<Vec<&str>>()[1];
    let nologin_site_code =  arsc.get_string(&main_package.name, "nologin_siteCode".to_owned()).unwrap().to_owned();

    apk.site_code = nologin_site_code.replace("_FR_", &format!("_{}_", country_code).to_owned());
    apk.brand_code = nologin_site_code[..2].to_owned();

    let brand = BRAND.get(&main_package.name).unwrap();
    apk.oauth_url = brand.oauth_url.to_owned();
    apk.realm = brand.realm.to_owned();

    Ok(())
}

fn parse_client_cert<R: Read + Seek>(archive: &mut ZipArchive<R>, apk: &mut APK) -> Result<(), Box<dyn std::error::Error>> {
    let mut pfx_buf = vec![];
    archive.by_name("assets/MWPMYMA1.pfx")?.read_to_end(&mut pfx_buf)?;

    // support legacy RC2-40-CBC algo
    let _provider = openssl::provider::Provider::try_load(None, "legacy", true).unwrap();
    let pkcs12 = Pkcs12::from_der(&pfx_buf)?.parse2("y5Y2my5B")?;
    apk.cert = String::from_utf8_lossy(&pkcs12.cert.unwrap().to_pem()?).to_string();
    apk.key = String::from_utf8_lossy(&pkcs12.pkey.unwrap().private_key_to_pem_pkcs8()?).to_string();
    Ok(())
}

fn parse_parameters<R: Read + Seek>(archive: &mut ZipArchive<R>, apk: &mut APK) -> Result<(), Box<dyn std::error::Error>> {
    let (parameters_filename, culture) = get_parameters_file_path(&archive)?;
    // read data
    let mut parameters = String::new();
    archive.by_name(parameters_filename.as_str())?.read_to_string(&mut parameters)?;

    let json: serde_json::Value = serde_json::from_str(&parameters)?;
    apk.culture = culture;
    apk.cvs_client_id = json["cvsClientId"].as_str().unwrap().to_owned();
    apk.cvs_secret = json["cvsSecret"].as_str().unwrap().to_owned();

    Ok(())
}

fn get_parameters_file_path<R: Read + Seek>(archive: &ZipArchive<R>) -> Result<(String, String), Box<dyn std::error::Error>> {
    // file filter for detecting locales
    let raw_filter = Regex::new(r"^res/raw-([a-z]{2})-r([A-Z]{2})/parameters.json$")?;

    // list files and filter
    let parameter_files: HashMap<String, &str> = archive.file_names()
        .filter(|file| raw_filter.is_match(file))
        .map(|file| {
            let caps: regex::Captures = raw_filter.captures(file).unwrap();
            let lang = caps.get(1).map_or("", |m| m.as_str()).to_owned();
            let country: &str = caps.get(2).map_or("", |m| m.as_str());
            (lang + "-" + country, file)
        }).collect();

    // extract locales
    let mut cultures = Vec::from_iter(parameter_files.keys().map(|x| x.to_string()).collect::<Vec<_>>());
    cultures.sort();

    println!("Select culture from following list:");
    println!("{}",  cultures.join(", "));
    println!("Locale:");

    let mut culture = String::new();
    std::io::stdin().read_line(&mut culture)?;
    culture = culture.trim().to_string();

    if !cultures.contains(&culture) {
        return Err(Box::new(ApkParserError { message: format!("Selected culture {culture} not found")}));
    }

    Ok((parameter_files[&culture].to_string(), culture))
}