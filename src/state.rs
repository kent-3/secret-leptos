use crate::{ClientOptionsBuilder, CHAIN_ID, LCD_URL};
use js_sys::Object;
use leptos::{create_rw_signal, RwSignal};
use wasm_bindgen::{JsCast, JsValue};

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
