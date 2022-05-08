use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::process::exit;

use anyhow::Result;
use base64::encode;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::StatusCode;
use serde::Deserialize;

use crate::core::config::ExtraHopCredential;

#[derive(Debug, Deserialize)]
pub struct ExtraHopToken {
    pub access_token: String,
    pub expires_in: u16,
    pub token_type: String,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ExtraHopAppliance {
    CCP,
    ECA,
    EDA,
    ETA,
    EXA,
}

impl Display for ExtraHopAppliance {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            ExtraHopAppliance::CCP => write!(f, "ccp"),
            ExtraHopAppliance::ECA => write!(f, "eca"),
            ExtraHopAppliance::EDA => write!(f, "eda"),
            ExtraHopAppliance::EXA => write!(f, "exa"),
            ExtraHopAppliance::ETA => write!(f, "eta"),
        }
    }
}

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

#[allow(clippy::too_many_arguments)]
impl ExtraHopClient {
    pub fn new(
        hostname: &str,
        user_id: &str,
        api_key: &str,
        base_url: &str,
        timestamp: &str,
        api_token: &str,
        allow_insecure_tls: &bool,
        appliance_type: ExtraHopAppliance,
    ) -> Self {
        let client = build_reqwest_client(api_key, api_token, allow_insecure_tls);

        Self {
            hostname: hostname.to_string(),
            user_id: user_id.to_string(),
            api_key: api_key.to_string(),
            api_token: api_token.to_string(),
            base_url: base_url.to_string(),
            timestamp: timestamp.to_string(),
            allow_insecure_tls: *allow_insecure_tls,
            appliance_type,
            reqwest_client: client,
        }
    }
}

fn build_reqwest_client(
    api_key: &str,
    api_token: &str,
    allow_insecure_tls: &bool,
) -> reqwest::Client {
    let auth_value = if api_token.is_empty() {
        // no token then must be an eca, eda, exa, eta
        format!("ExtraHop apikey={api_key}")
    } else {
        // has token is ccp
        format!("Bearer {api_token}")
    };

    let auth_value_header = match HeaderValue::from_str(&auth_value) {
        Ok(avh) => avh,
        Err(_) => {
            eprintln!("=> API key error");
            exit(1)
        }
    };

    let mut headers = HeaderMap::new();

    headers.insert(AUTHORIZATION, auth_value_header);
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

    let client = match reqwest::Client::builder()
        .default_headers(headers)
        .cookie_store(true)
        .danger_accept_invalid_certs(*allow_insecure_tls)
        .build()
    {
        Ok(c) => c,
        _ => {
            eprintln!("=> client builder error");
            exit(1)
        }
    };

    client
}

pub async fn get_oauth_token(
    hostname: &str,
    user_id: &str,
    api_key: &str,
) -> Result<ExtraHopToken> {
    let auth_url = format!("https://{hostname}/oauth2/token");

    let mut params = HashMap::new();
    params.insert("grant_type", "client_credentials");

    let mut headers = HeaderMap::new();

    let auth = encode(format!("{user_id}:{api_key}"));
    let key = match HeaderValue::from_str(&format!("Basic {auth}")) {
        Ok(k) => k,
        Err(_) => {
            eprintln!("=> API key error");
            exit(1)
        }
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
            eprintln!("=> error building oauth client");
            eprintln!("{:#?}", e);
            exit(1)
        }
    };

    let response = match client.post(auth_url).form(&params).send().await {
        Ok(r) => r,
        Err(e) => {
            eprintln!("=> error getting token");
            eprintln!("{:#?}", e);
            exit(1)
        }
    };

    match response.status() {
        StatusCode::OK => {
            let token: ExtraHopToken = serde_json::from_str(&response.text().await?)?;
            Ok(token)
        }
        _ => {
            eprintln!("=> unable to get token");
            eprintln!("{:#?}", response.error_for_status());
            exit(1)
        }
    }
}

pub async fn build_clients(
    credentials: &[ExtraHopCredential],
    appliance_type: ExtraHopAppliance,
    timestamp: &str,
) -> Result<Vec<ExtraHopClient>> {
    let mut extrahop_appliances: Vec<ExtraHopClient> = Vec::new();

    match appliance_type {
        ExtraHopAppliance::CCP => {
            for c in credentials {
                let token = get_oauth_token(&c.hostname, &c.user_id, &c.api_key).await?;
                let base_url = format!("https://{}/api/v1", &c.hostname);
                let client = ExtraHopClient::new(
                    &c.hostname,
                    &c.user_id,
                    &c.api_key,
                    &base_url,
                    timestamp,
                    &token.access_token,
                    &c.allow_insecure_tls,
                    appliance_type,
                );
                extrahop_appliances.push(client);
            }
        }
        _ => {
            for c in credentials {
                let base_url = format!("https://{}/api/v1", &c.hostname);
                let client = ExtraHopClient::new(
                    &c.hostname,
                    &c.user_id,
                    &c.api_key,
                    &base_url,
                    timestamp,
                    "",
                    &c.allow_insecure_tls,
                    appliance_type,
                );
                extrahop_appliances.push(client);
            }
        }
    }
    Ok(extrahop_appliances)
}
