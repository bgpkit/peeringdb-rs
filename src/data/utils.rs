use anyhow::Result;
use oneio::remote::create_client_with_headers;
use std::io::Read;
use tracing::warn;

pub(crate) fn get_reader(url: &str) -> Result<Box<dyn Read + Send>> {
    dotenvy::dotenv().ok();
    let api_key = std::env::var("PEERINGDB_API_KEY").unwrap_or_else(|_e| {
        warn!("missing PEERINGDB_API_KEY env var, call may fail due load restriction");
        "".to_string()
    });

    let client = create_client_with_headers([(
        "Authorization".to_string(),
        format!("Api-Key {}", api_key),
    )])?;
    let res = client
        .execute(client.get(url).build()?)?
        .error_for_status()?;
    Ok(Box::new(res))
}
