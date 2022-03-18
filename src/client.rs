use std::collections::HashMap;
use std::process::exit;

use base64::encode;
use reqwest;
use reqwest::StatusCode;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, Hash)]
pub enum ExtraHopAppliance {
    CCP,
    ECA,
    EDA,
    ETA,
    EXA,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct ExtraHopToken {
    pub access_token: String,
    pub expires_in: u16,
    pub token_type: String,
}

#[derive(Debug)]
pub struct ExtraHopClient {
    pub allow_insecure_tls: bool,
    pub api_token: String,
    pub api_key: String,
    pub appliance_type: ExtraHopAppliance,
    pub base_url: String,
    pub hostname: String,
    pub reqwest_client: reqwest::Client,
    pub timestamp: String,
    pub user_id: String,
}

impl ExtraHopClient {
    pub fn new(
        hostname: String,
        user_id: String,
        api_key: String,
        base_url: String,
        timestamp: String,
        api_token: String,
        allow_insecure_tls: bool,
        appliance_type: ExtraHopAppliance,
    ) -> Self {
        let client = build_reqwest_client(&api_key, &api_token, &allow_insecure_tls);

        Self {
            hostname,
            user_id,
            api_key,
            api_token,
            base_url,
            timestamp,
            allow_insecure_tls,
            appliance_type,
            reqwest_client: client,
        }
    }
}

fn build_reqwest_client(
    api_key: &String,
    api_token: &String,
    allow_insecure_tls: &bool,
) -> reqwest::Client {
    let auth_value = if api_token == &String::from("") {
        // no token then must be an eca, eda, exa, eta
        format!("ExtraHop apikey={api_key}")
    } else {
        // has token is ccp
        format!("Bearer {api_token}")
    };

    let auth_value_header = match HeaderValue::from_str(&auth_value) {
        Ok(avh) => avh,
        Err(_) => panic!("API key error"),
    };

    let mut headers = HeaderMap::new();

    headers.insert(AUTHORIZATION, auth_value_header);
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

    let client = match reqwest::Client::builder()
        .default_headers(headers)
        .cookie_store(true)
        .danger_accept_invalid_certs(allow_insecure_tls.to_owned())
        .build()
    {
        Ok(c) => c,
        _ => panic!("client builder error"),
    };

    client
}

pub async fn get_oauth_token(
    hostname: &String,
    user_id: &String,
    api_key: &String,
) -> Result<ExtraHopToken, Box<dyn std::error::Error>> {
    let auth_url = format!("https://{hostname}/oauth2/token");

    let mut params = HashMap::new();
    params.insert("grant_type", "client_credentials");

    let mut headers = HeaderMap::new();

    let auth = encode(format!("{user_id}:{api_key}"));
    let key = match HeaderValue::from_str(&format!("Basic {auth}")) {
        Ok(k) => k,
        Err(_) => panic!("API key error"),
    };

    headers.insert(AUTHORIZATION, key);
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/x-www-form-urlencoded"),
    );

    let client = match reqwest::Client::builder()
        .default_headers(headers)
        .cookie_store(true)
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            println!("error building oauth client");
            eprintln!("{:#?}", e);
            exit(1)
        }
    };

    let response = match client.post(auth_url).form(&params).send().await {
        Ok(r) => r,
        Err(e) => {
            println!("error getting token");
            eprintln!("{:#?}", e);
            exit(1)
        }
    };

    if response.status() == StatusCode::OK {
        let token: ExtraHopToken = serde_json::from_str(&response.text().await?)?;
        Ok(token)
    } else {
        println!("unable to get token");
        eprintln!("{:#?}", response.error_for_status());
        exit(1)
    }
}
