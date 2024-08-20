use crate::keplr::Error;
use base64::prelude::{Engine as _, BASE64_STANDARD};
use futures::TryFutureExt;
use js_sys::JsString;
use keplr_sys::KeplrOfflineSigner as RawKeplrOfflineSigner;
use keplr_sys::*;
use serde::{Deserialize, Serialize};
use std::{rc::Rc, sync::Arc};
use tracing::debug;
use web_sys::console;

/// An Amino encoded message.
#[derive(Debug, Serialize, Deserialize)]
pub struct AminoMsg {
    pub r#type: String,
    pub value: Vec<u8>,
}

/// Response after signing with Amino.
#[derive(Debug, Serialize, Deserialize)]
pub struct AminoSignResponse {
    /// The sign_doc that was signed.
    ///
    /// This may be different from the input sign_doc when the signer modifies it as part of the signing process.
    pub signed: StdSignDoc,
    pub signature: StdSignature,
}

/// Standard signature.
#[derive(Debug, Serialize, Deserialize)]
pub struct StdSignature {
    #[serde(alias = "pubKey")]
    pub pub_key: Pubkey,
    pub signature: String,
}

/// Public key.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pubkey {
    /// Possible types include:
    /// - "tendermint/PubKeySecp256k1"
    /// - "tendermint/PubKeyEd25519"
    /// - "tendermint/PubKeySr25519
    pub r#type: String,
    /// Base64 encoded String
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StdSignDoc {
    pub chain_id: String,
    pub account_number: String,
    pub sequence: String,
    pub fee: StdFee,
    pub msgs: Vec<AminoMsg>,
    pub memo: String,
}

/// Standard fee.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct StdFee {
    pub amount: Vec<Coin>,
    pub gas: String,
    pub granter: Option<String>,
}

pub type Coin = String;

#[derive(Serialize, Deserialize, Clone, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Key {
    /// Name of the selected key store.
    pub name: String,
    pub algo: String,
    pub pub_key: Vec<u8>,
    pub address: Vec<u8>,
    pub bech32_address: String,
    pub ethereum_hex_address: String,
    // Indicate whether the selected account is from the nano ledger.
    // Because current cosmos app in the nano ledger doesn't support the direct (proto) format msgs,
    // this can be used to select the amino or direct signer.
    pub is_nano_ledger: bool,
    pub is_keystone: bool,
}

impl std::fmt::Debug for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Key")
            .field("name", &self.name)
            .field("algo", &self.algo)
            .field("pub_key", &BASE64_STANDARD.encode(&self.pub_key)) // Convert pub_key to base64
            .field("address", &BASE64_STANDARD.encode(&self.address)) // Convert address to base64
            .field("bech32_address", &self.bech32_address)
            .field("ethereum_hex_address", &self.ethereum_hex_address)
            .field("is_nano_ledger", &self.is_nano_ledger)
            .field("is_keystone", &self.is_keystone)
            .finish()
    }
}

pub struct Keplr {}

impl Keplr {
    pub fn debug() {
        KEPLR.with(console::log_1)
    }

    pub fn is_available() -> bool {
        web_sys::window()
            .and_then(|window| {
                js_sys::Reflect::get(&window, &wasm_bindgen::JsValue::from_str("keplr")).ok()
            })
            .map_or(false, |keplr| !keplr.is_undefined() && !keplr.is_null())
    }

    pub async fn ping() -> Result<(), Error> {
        ping().await.map_err(Into::into)
    }

    pub async fn enable(chain_ids: Vec<String>) -> Result<(), Error> {
        enable(chain_ids).await.map_err(Into::into)
    }

    pub async fn get_key(chain_id: &str) -> Result<Key, Error> {
        get_key(chain_id)
            .await
            .and_then(|key| Ok(serde_wasm_bindgen::from_value::<Key>(key)?))
            .map_err(Into::into)
    }

    pub async fn get_account(chain_id: &str) -> Result<Account, Error> {
        let signer = get_offline_signer_only_amino(chain_id);
        let accounts = signer
            .get_accounts()
            .await
            .map_err(|_| Error::KeplrUnavailable)?;
        let accounts = js_sys::Array::from(&accounts);
        let account = accounts.get(0);

        let account: Account = serde_wasm_bindgen::from_value(account)?;

        Ok(account)
    }

    pub fn get_offline_signer(chain_id: &str) -> KeplrOfflineSigner {
        get_offline_signer(chain_id).into()
    }

    pub fn get_offline_signer_only_amino(chain_id: &str) -> KeplrOfflineSignerOnlyAmino {
        get_offline_signer_only_amino(chain_id).into()
    }

    pub fn get_enigma_utils(chain_id: &str) -> EnigmaUtils {
        get_enigma_utils(chain_id)
    }

    pub async fn suggest_token(
        chain_id: &str,
        contract_address: &str,
        viewing_key: Option<&str>,
    ) -> Result<(), Error> {
        suggest_token(chain_id, contract_address, viewing_key)
            .await
            .map_err(Into::into)
    }

    pub async fn get_secret_20_viewing_key(
        chain_id: &str,
        contract_address: &str,
    ) -> Result<String, Error> {
        get_secret_20_viewing_key(chain_id, contract_address)
            .await
            .map(|foo| JsString::from(foo).into())
            .map_err(Into::into)
    }

    pub fn disable(chain_id: &str) {
        disable(chain_id)
    }

    pub fn disable_origin() {
        disable_origin()
    }
}

#[derive(Clone)]
pub struct KeplrOfflineSigner {
    inner: Rc<keplr_sys::KeplrOfflineSigner>,
}

impl From<keplr_sys::KeplrOfflineSigner> for KeplrOfflineSigner {
    fn from(value: keplr_sys::KeplrOfflineSigner) -> Self {
        Self {
            inner: Rc::new(value),
        }
    }
}

impl KeplrOfflineSigner {
    pub fn chain_id(&self) -> String {
        self.inner
            .chain_id()
            .as_string()
            .expect("chain_id field is missing!")
    }

    pub async fn get_accounts(&self) -> Result<Account, Error> {
        self.inner
            .get_accounts()
            .await
            .map_err(|_| Error::KeplrUnavailable)
            .map(|val| js_sys::Array::from(&val))
            .map(|accounts| accounts.get(0))
            .and_then(|account| serde_wasm_bindgen::from_value(account).map_err(Into::into))
    }

    pub async fn sign_amino(
        &self,
        signer_address: impl ToString,
        sign_doc: StdSignDoc,
    ) -> Result<AminoSignResponse, Error> {
        todo!()
    }

    // pub async fn sign_direct(
    //     &self,
    //     signer_address: impl ToString,
    //     sign_doc: SignDoc,
    // ) -> Result<DirectSignResponse, Error> {
    //     todo!()
    // }
}

#[derive(Clone)]
pub struct KeplrOfflineSignerOnlyAmino {
    inner: Rc<keplr_sys::KeplrOfflineSignerOnlyAmino>,
}

impl From<keplr_sys::KeplrOfflineSignerOnlyAmino> for KeplrOfflineSignerOnlyAmino {
    fn from(value: keplr_sys::KeplrOfflineSignerOnlyAmino) -> Self {
        Self {
            inner: Rc::new(value),
        }
    }
}

impl KeplrOfflineSignerOnlyAmino {
    pub fn chain_id(&self) -> String {
        self.inner
            .chain_id()
            .as_string()
            .expect("chain_id field is missing!")
    }

    pub async fn get_accounts(&self) -> Result<Account, Error> {
        self.inner
            .get_accounts()
            .await
            .map_err(|_| Error::KeplrUnavailable)
            .map(|val| js_sys::Array::from(&val))
            .map(|accounts| accounts.get(0))
            .and_then(|account| serde_wasm_bindgen::from_value(account).map_err(Into::into))
    }

    pub async fn sign_amino(
        &self,
        signer_address: impl ToString,
        sign_doc: StdSignDoc,
    ) -> Result<AminoSignResponse, Error> {
        todo!()
    }

    // pub async fn sign_direct(
    //     &self,
    //     signer_address: impl ToString,
    //     sign_doc: SignDoc,
    // ) -> Result<DirectSignResponse, Error> {
    //     todo!()
    // }
}
#[derive(Deserialize, Clone)]
pub struct Account {
    pub address: String,
    pub algo: String,
    pub pubkey: Vec<u8>,
}

impl std::fmt::Debug for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Account")
            .field("address", &self.address)
            .field("algo", &self.algo)
            .field("pubkey", &BASE64_STANDARD.encode(&self.pubkey)) // Convert pubkey to base64
            .finish()
    }
}

pub mod suggest_chain_types {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct SuggestingChainInfo {
        pub chain_id: String,
        pub chain_name: String,
        pub rpc: String,
        pub rest: String,
        pub bip44: Bip44,
        pub bech32_config: Bech32Config,
        pub currencies: Vec<Currency>,
        pub fee_currencies: Vec<FeeCurrency>,
        pub stake_currency: Currency,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Bip44 {
        pub coin_type: u32,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Bech32Config {
        pub bech32_prefix_acc_addr: String,
        pub bech32_prefix_acc_pub: String,
        pub bech32_prefix_val_addr: String,
        pub bech32_prefix_val_pub: String,
        pub bech32_prefix_cons_addr: String,
        pub bech32_prefix_cons_pub: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Currency {
        pub coin_denom: String,
        pub coin_minimal_denom: String,
        pub coin_decimals: u8,
        pub coin_gecko_id: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct FeeCurrency {
        pub coin_denom: String,
        pub coin_minimal_denom: String,
        pub coin_decimals: u8,
        pub coin_gecko_id: String,
        pub gas_price_step: GasPriceStep,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct GasPriceStep {
        pub low: f64,
        pub average: f64,
        pub high: f64,
    }
}
