use crate::data::utils::get_reader;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PeeringdbOrg {
    pub id: u32,
    pub name: Option<String>,
    pub aka: Option<String>,
    pub name_long: Option<String>,
    pub website: Option<String>,
    // pub social_media: Option<Vec<Value>>,
    pub notes: Option<String>,
    pub require_2fa: Option<bool>,
    pub address1: Option<String>,
    pub address2: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub state: Option<String>,
    pub zipcode: Option<String>,
    pub floor: Option<String>,
    pub suite: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub created: Option<DateTime<Utc>>,
    pub updated: Option<DateTime<Utc>>,
    pub status: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct PeeringdbOrgResponse {
    pub data: Vec<PeeringdbOrg>,
}

pub fn load_peeringdb_org() -> anyhow::Result<Vec<PeeringdbOrg>> {
    let mut reader = get_reader("https://www.peeringdb.com/api/org")?;
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let res: PeeringdbOrgResponse = serde_json::from_str(&buf)?;
    Ok(res.data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loading_data() {
        let data = load_peeringdb_org();
        assert!(data.is_ok());
    }
}
