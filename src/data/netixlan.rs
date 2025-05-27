//! `netixlan` objects links net to ixlan and then to ix
//!
//! This is a useful data to connect ASNs to IXes and find connected networks that share the same IX
//! connectivity.

use crate::data::utils::get_reader;
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
    let mut reader = get_reader("https://www.peeringdb.com/api/org")?;
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
    }
}
