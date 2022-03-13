use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE};

#[derive(Debug)]
pub struct ExtraHopClient {
    pub hostname: String,
    pub user_id: String,
    pub secret: String,
    pub base_url: String,
    pub reqwest_client: reqwest::Client,
    pub timestamp: String,
}

// TODO: Make this an associated `new` function for the ExtraHopClient struct.
pub fn build_client(api_key: &String) -> reqwest::Client {
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
