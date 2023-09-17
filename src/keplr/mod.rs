use crate::{state::GlobalState, CHAIN_ID};
use js_sys::Promise;
use leptos::{error::Result, *};
use serde::Deserialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

mod tests;
pub use tests::KeplrTests;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "keplr"])]
    pub fn enable(chain_id: &str) -> Promise;

    #[wasm_bindgen(js_namespace = ["window", "keplr"], js_name = getOfflineSignerOnlyAmino)]
    pub fn get_offline_signer_only_amino(chain_id: &str) -> KeplrOfflineSigner;

    #[wasm_bindgen(js_namespace = ["window", "keplr"], js_name = getSecret20ViewingKey)]
    pub fn get_secret_20_viewing_key(chain_id: &str, contract_address: &str) -> Promise; // Or more specific type if known
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug, Clone)]
    pub type KeplrOfflineSigner;

    #[wasm_bindgen(method, structural, js_name = getAccounts)]
    pub fn get_accounts(this: &KeplrOfflineSigner) -> Promise;
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    pub type EnigmaUtils;

    #[wasm_bindgen(js_namespace = ["window", "keplr"], js_name = getEnigmaUtils)]
    pub fn get_enigma_utils(chain_id: &str) -> EnigmaUtils;
}

#[derive(Deserialize, Debug)]
pub struct Account {
    pub address: String,
    pub algo: String,
    pub pubkey: Vec<u8>,
}

pub async fn enable_keplr() -> Result<bool> {
    log::debug!("Enabling Keplr...");

    let enable_promise = enable(CHAIN_ID);
    let enable_js_value = JsFuture::from(enable_promise).await;

    match enable_js_value {
        Ok(js_value) => {
            log::debug!("Ok: {js_value:#?}");
            Ok(true)
        }
        Err(js_error) => {
            log::debug!("Err: {js_error:#?}");
            Ok(false)
        }
    }
}

pub fn get_offline_signer() -> Result<KeplrOfflineSigner> {
    let signer = get_offline_signer_only_amino(CHAIN_ID);
    log::debug!("{:#?}", signer);

    Ok(signer)
}

pub async fn get_viewing_key(token_address: String) -> Result<String> {
    // TODO - add a guard to check if keplr is enabled?
    log::debug!("Trying to get viewing key...");

    let key_promise = get_secret_20_viewing_key(CHAIN_ID, &token_address);
    let key_js_value = JsFuture::from(key_promise).await;

    match key_js_value {
        Ok(js_value) => {
            log::debug!("Ok: {js_value:#?}");
            let key = js_value.as_string().unwrap_or_default();
            Ok(key)
        }
        Err(js_error) => {
            log::debug!("Err: {js_error:#?}");
            let error = format!("{js_error:#?}");
            Ok(error)
        }
    }
}

pub async fn get_account() -> Result<String> {
    let signer = get_offline_signer_only_amino(CHAIN_ID);

    let accounts_promise = signer.get_accounts();
    let accounts_js_value = JsFuture::from(accounts_promise).await;

    match accounts_js_value {
        Ok(js_value) => {
            log::debug!("Ok: {js_value:#?}");
            let mut accounts: Vec<Account> = ::serde_wasm_bindgen::from_value(js_value).unwrap();
            let address = accounts.remove(0).address;
            log::debug!("address: {:#?}", address);
            Ok(address)
        }
        Err(js_error) => {
            log::debug!("Err: {js_error:#?}");
            Ok("Keplr is not enabled!".to_string())
        }
    }
}

async fn keplr_get_enigma_utils() -> Result<()> {
    let enigma_utils = get_enigma_utils(CHAIN_ID);
    log::debug!("{:#?}", enigma_utils);
    Ok(())
}
