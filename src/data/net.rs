use crate::data::utils::get_reader;
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PeeringdbNet {
    pub id: u32,
    pub name: Option<String>,
    pub name_long: Option<String>,
    pub aka: Option<String>,
    pub asn: Option<u32>,
    pub org_id: Option<u32>,
    pub irr_as_set: Option<String>,
    pub website: Option<String>,
    pub notes: Option<String>,
    pub fac_count: Option<usize>,
    pub ix_count: Option<u32>,

    pub policy_contracts: Option<String>,
    pub policy_general: Option<String>,
    pub policy_locations: Option<String>,
    pub policy_ratio: Option<bool>,
    pub policy_url: Option<String>,

    pub info_ipv6: Option<bool>,
    pub info_multicast: Option<bool>,
    pub info_never_via_route_servers: Option<bool>,
    pub info_prefixes4: Option<u32>,
    pub info_prefixes6: Option<u32>,
    pub info_ratio: Option<String>,
    pub info_scope: Option<String>,
    pub info_traffic: Option<String>,
    pub info_type: Option<String>,
    pub info_types: Option<Vec<String>>,
    pub info_unicast: Option<bool>,

    pub rir_status: Option<String>,
    pub status: Option<String>,
    pub status_dashboard: Option<String>,
    pub created: Option<String>,
    pub updated: Option<String>,
    pub netfac_updated: Option<DateTime<Utc>>,
    pub netixlan_updated: Option<DateTime<Utc>>,
    pub rir_status_updated: Option<DateTime<Utc>>,
    pub poc_updated: Option<String>,
    pub route_server: Option<String>,
    pub social_media: Option<Vec<Value>>,
    pub looking_glass: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PeeringdbNetResponse {
    data: Vec<PeeringdbNet>,
}

pub fn load_peeringdb_net() -> Result<Vec<PeeringdbNet>> {
    let mut reader = get_reader("https://www.peeringdb.com/api/net")?;
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let res: PeeringdbNetResponse = serde_json::from_str(&buf)?;
    Ok(res.data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loading_data() {
        let data = load_peeringdb_net();
        assert!(data.is_ok());
    }
}
