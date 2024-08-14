use leptos::prelude::*;

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

#[derive(Copy, Clone)]
pub struct KeplrSignals {
    pub is_enabled: ReadSignal<bool>,
    pub enabled: WriteSignal<bool>,
}

impl KeplrSignals {
    pub fn new() -> Self {
        let (is_enabled, enabled) = signal(false);
        Self {
            is_enabled,
            enabled,
        }
    }
}

impl Default for KeplrSignals {
    fn default() -> Self {
        let (is_enabled, enabled) = signal(false);
        Self {
            is_enabled,
            enabled,
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
