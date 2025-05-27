use anyhow::Result;
use reqwest::header::HeaderMap;
use std::collections::HashMap;
use std::io::Read;
use tracing::warn;

pub(crate) fn get_reader(url: &str) -> Result<Box<dyn Read + Send>> {
    dotenvy::dotenv().ok();
    let api_key = match std::env::var("PEERINGDB_API_KEY") {
        Ok(k) => k,
        Err(_e) => {
            warn!("missing PEERINGDB_API_KEY env var, call may fail due load restriction");
            "".to_string()
        }
    };
    let headers: HeaderMap =
        (&HashMap::from([("Authorization".to_string(), format!("Api-Key {}", api_key))]))
            .try_into()
            .expect("invalid headers");
    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()?;
    let res = client
        .execute(client.get(url).build()?)?
        .error_for_status()?;
    Ok(Box::new(res))
}
