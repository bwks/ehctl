use reqwest;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE};

#[derive(Debug)]
pub struct ExtraHopClient {
    pub hostname: String,
    pub user_id: String,
    pub api_key: String,
    pub base_url: String,
    pub timestamp: String,
    pub reqwest_client: reqwest::Client,
}

impl ExtraHopClient {
    pub fn new(
        hostname: String,
        user_id: String,
        api_key: String,
        base_url: String,
        timestamp: String,
    ) -> Self {
        let client = if hostname.ends_with("cloud.extrahop.com") {
            reqwest_client_oauth(&user_id, &api_key)
        } else {
            request_client_basic_auth(&api_key)
        };

        Self {
            hostname,
            user_id,
            api_key,
            base_url,
            timestamp,
            reqwest_client: client,
        }
    }
}

fn request_client_basic_auth(api_key: &String) -> reqwest::Client {
    let key = match HeaderValue::from_str(&format!("ExtraHop apikey={}", api_key)) {
        Ok(k) => k,
        Err(_) => panic!("API key error"),
    };

    let mut headers = HeaderMap::new();

    headers.insert(AUTHORIZATION, key);
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

    let client = match reqwest::Client::builder()
        .default_headers(headers)
        .cookie_store(true)
        .danger_accept_invalid_certs(true)
        .build()
    {
        Ok(c) => c,
        _ => panic!("client builder error"),
    };

    client
}

/// client used for CCP which uses OAUTH
/// TODO: Implement
#[allow(unused_variables)]
fn reqwest_client_oauth(user_id: &String, api_key: &String) -> reqwest::Client {
    reqwest::Client::new()
}
