//! `ix` objects contains information of Internet Exchanges

use crate::data::utils::get_reader_with_params;
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeeringdbIx {
    pub id: u32,
    pub org_id_id: Option<u32>,
    pub name: Option<String>,
    pub aka: Option<String>,
    pub name_long: Option<String>,

    pub city: Option<String>,
    pub country: Option<String>,
    pub region_continent: Option<String>,
    pub media: Option<String>,
    pub notes: Option<String>,

    pub proto_unicast: Option<bool>,
    pub proto_multicast: Option<bool>,
    pub proto_ipv6: Option<bool>,
    pub website: Option<String>,
    pub url_stats: Option<String>,
    pub status_dashboard: Option<String>,

    pub tech_email: Option<String>,
    pub tech_phone: Option<String>,
    pub policy_email: Option<String>,
    pub policy_phone: Option<String>,
    pub sales_email: Option<String>,
    pub sales_phone: Option<String>,

    pub ixf_net_count: Option<usize>,
    pub ixf_last_import: Option<String>,
    pub ixf_import_request: Option<DateTime<Utc>>,
    pub ixf_import_request_status: Option<String>,

    pub net_count: Option<usize>,
    pub fac_count: Option<usize>,
    pub service_level: Option<String>,
    pub terms: Option<String>,

    pub created: Option<DateTime<Utc>>,
    pub updated: Option<DateTime<Utc>>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PeeringdbIxResponse {
    data: Vec<PeeringdbIx>,
}

pub fn load_peeringdb_ix() -> Result<Vec<PeeringdbIx>> {
    load_peeringdb_ix_filtered(&[])
}

/// Same as [load_peeringdb_ix], filtered by PeeringDB API query parameters,
/// e.g. `&[("country", "DE")]`. See the [list ix][api] documentation for the available
/// fields.
///
/// [api]: https://www.peeringdb.com/apidocs/#tag/api/operation/list%20ix
pub fn load_peeringdb_ix_filtered(params: &[(&str, &str)]) -> Result<Vec<PeeringdbIx>> {
    let mut reader = get_reader_with_params("https://www.peeringdb.com/api/ix", params)?;
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let res: PeeringdbIxResponse = serde_json::from_str(&buf)?;
    Ok(res.data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loading_data() {
        let data = load_peeringdb_ix();
        assert!(data.is_ok());
    }
}
