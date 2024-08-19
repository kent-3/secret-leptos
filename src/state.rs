use crate::keplr::{Keplr, KeyInfo};
use crate::{constants::*, error::Error, tokens::ContractInfo};
use codee::string::JsonSerdeCodec;
use leptos::prelude::*;
use send_wrapper::SendWrapper;
use std::collections::HashMap;
use tonic_web_wasm_client::Client;
use tracing::debug;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WasmClient {
    pub client: RwSignal<Client>,
    pub url: RwSignal<String>,
}

impl WasmClient {
    pub fn new() -> Self {
        Self {
            client: RwSignal::new(Client::new(GRPC_URL.to_string())),
            url: RwSignal::new(GRPC_URL.to_string()),
        }
    }
}

impl std::ops::Deref for WasmClient {
    type Target = RwSignal<Client>;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

#[derive(Clone, Debug)]
pub struct TokenMap(HashMap<String, ContractInfo>);

impl TokenMap {
    pub fn new() -> Self {
        let json = include_bytes!(concat!(env!("OUT_DIR"), "/token_map.json"));
        let token_map: HashMap<String, ContractInfo> =
            serde_json::from_slice(json).expect("Failed to deserialize token_map");

        Self(token_map)
    }
}

impl std::ops::Deref for TokenMap {
    type Target = HashMap<String, ContractInfo>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<HashMap<String, ContractInfo>> for TokenMap {
    fn as_ref(&self) -> &HashMap<String, ContractInfo> {
        &self.0
    }
}

#[derive(Copy, Clone)]
pub struct KeplrSignals {
    pub enabled: RwSignal<bool>,
    pub key_info: Resource<Result<KeyInfo, Error>, JsonSerdeCodec>,
    // pub key_info: RwSignal<Option<KeyInfo>>,
}

impl KeplrSignals {
    pub fn new() -> Self {
        let enabled = RwSignal::new(false);
        let key_info = Resource::new(enabled, move |enabled| {
            SendWrapper::new(async move {
                if enabled {
                    debug!("keplr is enabled! getting key_info");
                    Keplr::get_key(CHAIN_ID).await.map_err(Into::into)
                } else {
                    Err(Error::KeplrDisabled)
                }
            })
        });

        Self { enabled, key_info }
    }
}
