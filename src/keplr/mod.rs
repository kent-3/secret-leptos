use leptos::{error::Result, logging::log, *};
use serde::Deserialize;
use wasm_bindgen::prelude::*;

mod tests;
pub use tests::KeplrTests;

use js_sys::{JsString, Uint8Array};

#[wasm_bindgen(js_namespace = ["window", "keplr"])]
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = keplr)]
    pub static KEPLR: JsValue;

    pub type KeplrOfflineSigner;
    pub type JsEnigmaUtils;

    #[wasm_bindgen(catch)]
    pub async fn enable(chain_id: &str) -> Result<(), JsValue>;

    #[wasm_bindgen]
    pub fn disable(chain_id: &str);

    #[wasm_bindgen(js_name = disable)]
    /// Disable all chains for this origin (website).
    pub fn disable_origin();

    #[wasm_bindgen(catch, js_name = experimentalSuggestChain)]
    pub async fn suggest_chain(chainInfo: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = getChainInfoWithoutEndpoints)]
    pub async fn get_chain_info(chain_id: &str) -> JsValue;

    #[wasm_bindgen(js_name = getKey)]
    pub async fn get_key(chain_id: &str) -> JsValue;

    #[wasm_bindgen(js_name = getOfflineSigner)]
    pub fn get_offline_signer(chain_id: &str) -> KeplrOfflineSigner;

    #[wasm_bindgen(js_name = getOfflineSignerAuto)]
    pub fn get_offline_signer_auto(chain_id: &str) -> KeplrOfflineSigner;

    #[wasm_bindgen(js_name = getOfflineSignerOnlyAmino)]
    pub fn get_offline_signer_only_amino(chain_id: &str) -> KeplrOfflineSigner;

    #[wasm_bindgen(js_name = getEnigmaUtils)]
    pub fn get_enigma_utils(chain_id: &str) -> JsEnigmaUtils;

    #[wasm_bindgen(catch, js_name = sendTx)]
    pub async fn sendTx(chainId: &str, tx: &[u8], mode: &str) -> Result<JsValue, JsValue>;

    // KeplrOfflineSigner methods

    #[wasm_bindgen(method, getter, js_name = chainId)]
    pub fn chain_id(this: &KeplrOfflineSigner) -> JsValue;

    #[wasm_bindgen(method, js_name = getAccounts)]
    pub async fn get_accounts(this: &KeplrOfflineSigner) -> JsValue;

    #[wasm_bindgen(method, js_name = signAmino)]
    pub async fn sign_amino(
        this: &KeplrOfflineSigner,
        signerAddress: JsString,
        signDoc: JsValue, // StdSignDoc
    ) -> JsValue; // AminoSignResponse

    #[wasm_bindgen(method, js_name = signDirect)]
    pub async fn sign_direct(
        this: &KeplrOfflineSigner,
        signerAddress: JsString,
        signDoc: JsValue, // SignDoc
    ) -> JsValue; // DirectSignResponse

    // EnigmaUtils methods (all of these return Uint8Array)

    #[wasm_bindgen(method, getter, js_name = chainId)]
    pub fn chain_id(this: &JsEnigmaUtils) -> JsValue;

    #[wasm_bindgen(method, js_name = decrypt)]
    pub async fn decrypt(
        this: &JsEnigmaUtils,
        ciphertext: Uint8Array,
        nonce: Uint8Array,
    ) -> JsValue;

    #[wasm_bindgen(method, js_name = encrypt)]
    pub async fn encrypt(
        this: &JsEnigmaUtils,
        contract_code_hash: JsString,
        msg: JsValue,
    ) -> JsValue;

    #[wasm_bindgen(method, js_name = getPubkey)]
    pub async fn get_pubkey(this: &JsEnigmaUtils) -> JsValue;

    #[wasm_bindgen(method, js_name = getTxEncryptionKey)]
    pub async fn get_tx_encryption_key(this: &JsEnigmaUtils, nonce: Uint8Array) -> JsValue;

    // Enigma functions (all of these return Uint8Array)

    #[wasm_bindgen(js_name = enigmaEncrypt)]
    pub async fn enigma_encrypt(chain_id: &str, code_hash: &str, msg: JsValue) -> JsValue;

    #[wasm_bindgen(js_name = enigmaDecrypt)]
    pub async fn enigma_decrypt(chain_id: &str, ciphertext: &[u8], nonce: &[u8]) -> JsValue;

    #[wasm_bindgen(js_name = getEnigmaPubKey)]
    pub async fn get_enigma_pub_key(chain_id: &str) -> JsValue;

    #[wasm_bindgen(js_name = getEnigmaTxEncryptionKey)]
    pub async fn get_enigma_tx_encryption_key(chain_id: &str, nonce: &[u8]) -> JsValue;

    // other Secret functions

    #[wasm_bindgen(js_name = suggestToken)]
    pub async fn suggest_token(chainId: &str, contract_address: &str);

    #[wasm_bindgen(js_name = getSecret20ViewingKey)]
    pub fn get_secret_20_viewing_key(chain_id: &str, contract_address: &str) -> String;
}

#[derive(Deserialize, Debug)]
pub struct Account {
    pub address: String,
    pub algo: String,
    pub pubkey: Vec<u8>,
}

// pub async fn enable_keplr() -> bool {
//     log!("Enabling Keplr...");
//
//     let enable_result = enable(CHAIN_ID).await;
//     // let enable_js_value = JsFuture::from(enable_promise).await;
//
//     match enable_result {
//         Ok(js_value) => {
//             log!("Ok: {js_value:#?}");
//             true
//         }
//         Err(js_error) => {
//             log!("Err: {js_error:#?}");
//             false
//         }
//     }
// }
