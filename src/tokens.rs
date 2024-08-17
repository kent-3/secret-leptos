use serde::{Deserialize, Serialize};
use serde_json::from_slice;
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContractInfo {
    pub contract_address: String,
    pub image_url: String,
    pub metadata: Metadata,
}

pub fn keplr_contract_registry_tokens() -> HashMap<String, ContractInfo> {
    let json = include_bytes!(concat!(env!("OUT_DIR"), "/token_map.json"));
    let token_map: HashMap<String, ContractInfo> =
        from_slice(json).expect("Failed to deserialize token_map");

    tracing::debug!("Loaded {} tokens", token_map.len());

    token_map
}
