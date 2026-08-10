//! `netixlan` objects links net to ixlan and then to ix
//!
//! This is a useful data to connect ASNs to IXes and find connected networks that share the same IX
//! connectivity.

use crate::data::utils::get_reader_with_params;
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeeringdbNetixlan {
    pub id: u32,
    pub net_id: Option<u32>,
    pub ix_id: Option<u32>,
    pub name: Option<String>,
    pub ixlan_id: Option<u32>,
    pub notes: Option<String>,
    pub speed: Option<u64>,
    pub asn: Option<u32>,
    pub ipaddr4: Option<IpAddr>,
    pub ipaddr6: Option<IpAddr>,
    pub is_rs_peer: Option<bool>,
    pub bfd_support: Option<bool>,
    pub operational: Option<bool>,
    pub created: Option<DateTime<Utc>>,
    pub updated: Option<DateTime<Utc>>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeeringdbNetixlanResponse {
    pub data: Vec<PeeringdbNetixlan>,
}

pub fn load_peeringdb_netixlan() -> Result<Vec<PeeringdbNetixlan>> {
    load_peeringdb_netixlan_filtered(&[])
}

/// Same as [load_peeringdb_netixlan], filtered by PeeringDB API query parameters,
/// e.g. `&[("ix_id", "26")]`. See the [list netixlan][api] documentation for the available
/// fields.
///
/// [api]: https://www.peeringdb.com/apidocs/#tag/api/operation/list%20netixlan
pub fn load_peeringdb_netixlan_filtered(params: &[(&str, &str)]) -> Result<Vec<PeeringdbNetixlan>> {
    let mut reader = get_reader_with_params("https://www.peeringdb.com/api/netixlan", params)?;
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let res: PeeringdbNetixlanResponse = serde_json::from_str(&buf)?;
    Ok(res.data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loading_data() {
        let vec = load_peeringdb_netixlan().unwrap();
        dbg!(&vec[0]);
        // org records also deserialize into PeeringdbNetixlan (only `id` is required),
        // so check for a netixlan-specific field to make sure we hit the right endpoint
        assert!(vec.iter().any(|n| n.asn.is_some()));
    }
}
