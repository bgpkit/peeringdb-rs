use anyhow::Result;
use oneio::remote::create_client_with_headers;
use std::io::Read;
use tracing::warn;

pub(crate) fn get_reader_with_params(
    url: &str,
    params: &[(&str, &str)],
) -> Result<Box<dyn Read + Send>> {
    dotenvy::dotenv().ok();
    let mut headers = vec![(
        "User-Agent".to_string(),
        format!("peeringdb-rs/{}", env!("CARGO_PKG_VERSION")),
    )];
    match std::env::var("PEERINGDB_API_KEY") {
        Ok(api_key) if !api_key.is_empty() => {
            headers.push(("Authorization".to_string(), format!("Api-Key {}", api_key)));
        }
        _ => {
            warn!("missing PEERINGDB_API_KEY env var, call may fail due load restriction");
        }
    }

    let client = create_client_with_headers(headers)?;
    let mut request = client.get(url);
    if !params.is_empty() {
        request = request.query(params);
    }
    let res = client.execute(request.build()?)?.error_for_status()?;
    Ok(Box::new(res))
}
