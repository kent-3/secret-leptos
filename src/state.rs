use crate::ClientOptionsBuilder;
use crate::CHAIN_ID;
use crate::LCD_URL;
use js_sys::Object;
use leptos::{create_rw_signal, RwSignal};
use wasm_bindgen::{JsCast, JsValue};

use crate::secretjs::SecretNetworkClient;

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

#[derive(Copy, Clone, Debug)]
pub struct MyAccount {
    pub my_address: RwSignal<String>,
    pub my_client: RwSignal<SecretNetworkClient>,
}

impl MyAccount {
    pub fn new() -> Self {
        let client_options = ClientOptionsBuilder::new()
            .url(LCD_URL)
            .chain_id(CHAIN_ID)
            .build();
        let client = SecretNetworkClient::new(&client_options);

        Self {
            my_address: create_rw_signal("unknown".to_string()),
            my_client: create_rw_signal(client),
        }
    }
}
