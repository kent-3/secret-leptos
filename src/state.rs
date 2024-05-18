use crate::{ClientOptionsBuilder, CHAIN_ID, GRPC_URL, LCD_URL};
use leptos::{create_rw_signal, create_signal, ReadSignal, RwSignal};

use crate::secretjs::SecretNetworkClient;

// Still deciding what else to include here.
#[derive(Copy, Clone, Debug)]
pub struct GlobalState {
    pub keplr_enabled: RwSignal<bool>,
    pub my_address: RwSignal<String>,
}

impl GlobalState {
    pub fn new() -> Self {
        Self {
            keplr_enabled: create_rw_signal(false),
            my_address: create_rw_signal("unknown".to_string()),
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
#[derive(Copy, Clone, Debug)]
pub struct MyAccount {
    pub my_client: RwSignal<SecretNetworkClient>,
}

impl MyAccount {
    pub fn new() -> Self {
        // Start out with a readonly client. Update to a signing client when connected.
        let client_options = ClientOptionsBuilder::new()
            .url(LCD_URL)
            .chain_id(CHAIN_ID)
            .build();
        let client = SecretNetworkClient::new(&client_options);

        Self {
            my_client: create_rw_signal(client),
        }
    }
}
