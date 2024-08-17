use crate::{
    constants::{CHAIN_ID, GRPC_URL},
    tokens::ContractInfo,
};
use ::keplr::Keplr;
use ::keplr::KeyInfo;
use leptos::prelude::*;
use send_wrapper::SendWrapper;
use std::collections::HashMap;
use tonic_web_wasm_client::Client;
use tracing::debug;

// Still deciding what else to include here.
#[derive(Copy, Clone, Debug)]
pub struct GlobalState {
    pub keplr_enabled: RwSignal<bool, SyncStorage>,
    pub my_address: RwSignal<String, SyncStorage>,
}

impl GlobalState {
    pub fn new() -> Self {
        Self {
            keplr_enabled: RwSignal::new(false),
            my_address: RwSignal::new("unknown".to_string()),
        }
    }
}

#[derive(Copy, Clone, Debug)]
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

        Self { 0: token_map }
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

// #[derive(Copy, Clone)]
// pub struct ClientSignals {
//     pub grpc_url: RwSignal<String>,
//     pub client: RwSignal<Client>,
// }
//
// impl ClientSignals {
//     pub fn new() -> Self {
//         Self {
//             grpc_url: RwSignal::new(GRPC_URL.to_string()),
//             client: RwSignal::new(Client::new(GRPC_URL.to_string())),
//         }
//     }
// }
//
// impl Default for ClientSignals {
//     fn default() -> Self {
//         Self {
//             grpc_url: RwSignal::new(GRPC_URL.to_string()),
//             client: RwSignal::new(Client::new(GRPC_URL.to_string())),
//         }
//     }
// }

#[derive(Copy, Clone)]
pub struct KeplrSignals {
    pub enabled: RwSignal<bool>,
    pub key_info: RwSignal<Option<KeyInfo>>,
}

impl KeplrSignals {
    pub fn new() -> Self {
        // TODO: make keplr crate return actual Errors, not Strings

        // let enabled = RwSignal::new(false);
        // let key_info = Resource::new(enabled, move |enabled| {
        //     SendWrapper::new(async move {
        //         if enabled {
        //             debug!("happy path");
        //             Keplr::get_key(CHAIN_ID).await
        //         } else {
        //             debug!("sad path");
        //             Err(())
        //         }
        //     })
        // });

        Self {
            enabled: RwSignal::new(false),
            key_info: RwSignal::new(None),
        }
    }
}

impl Default for KeplrSignals {
    fn default() -> Self {
        Self {
            enabled: RwSignal::new(false),
            key_info: RwSignal::new(None),
        }
    }
}

// not sure this a good approach...
// use secretrs::clients::AuthQueryClient;
// #[derive(Clone, Debug)]
// pub struct SecretQueryClient {
//     pub auth: AuthQueryClient<::tonic_web_wasm_client::Client>,
// }
//
// impl SecretQueryClient {
//     pub fn new() -> Self {
//         let web_client = ::tonic_web_wasm_client::Client::new(GRPC_URL.to_string());
//
//         let mut auth = AuthQueryClient::new(web_client);
//
//         Self { auth }
//     }
// }

// Still deciding what else to include here.
// #[derive(Clone, Debug)]
// pub struct MyAccount {
//     pub my_client: RwSignal<SecretNetworkClient>,
// }
//
// impl MyAccount {
//     pub fn new() -> Self {
//         // Start out with a readonly client. Update to a signing client when connected.
//         let client_options = ClientOptionsBuilder::new()
//             .url(LCD_URL)
//             .chain_id(CHAIN_ID)
//             .build();
//         let client = SecretNetworkClient::new(&client_options);
//
//         Self {
//             my_client: RwSignal::new(client),
//         }
//     }
// }
