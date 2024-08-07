use crate::CHAIN_ID;
use ::keplr::*;
use js_sys::Error;
use keplr_sys::disable;
use leptos::html::Dialog;
use leptos::prelude::*;
use leptos::web_sys::console;

async fn enable_keplr(chain_id: &str) -> bool {
    log!("Trying to enable Keplr...");

    let result = Keplr::enable(chain_id).await.map_err(|js_value| {
        let error = Error::from(js_value)
            .message()
            .as_string()
            .unwrap_or("unknown error".to_string());
        error
    });

    match &result {
        Ok(_) => log!("Keplr is enabled."),
        Err(ref e) => log!("{e}"),
    }

    result.is_ok()
}

// this method seems dumb since `get_key` returns the same and more
async fn get_account(chain_id: &str) -> Account {
    let signer = Keplr::get_offline_signer_only_amino(chain_id);
    let accounts = signer.get_accounts().await;
    let accounts = js_sys::Array::from(&accounts);
    let account = accounts.get(0);

    let account: Account = serde_wasm_bindgen::from_value(account).unwrap();
    log!("{account:#?}");

    account
}

async fn get_key(chain_id: &str) -> KeyInfo {
    let result = Keplr::get_key(chain_id).await.map_err(|js_value| {
        let error = Error::from(js_value)
            .message()
            .as_string()
            .unwrap_or("unknown error".to_string());
        error
    });

    match result {
        Ok(ref key_info) => log!("{key_info:#?}"),
        Err(ref e) => log!("{e}"),
    }

    result.unwrap_or_default()
}

fn get_enigma_utils(chain_id: &str) -> () {
    let js_value = Keplr::get_enigma_utils(chain_id);

    console::log_1(&js_value.inner.into());
}

async fn get_secret_20_viewing_key(chain_id: &str, contract_address: String) -> String {
    let key = Keplr::get_secret_20_viewing_key(chain_id, &contract_address)
        .await
        .into();

    log!("{key}");

    key
}

async fn suggest_token(chain_id: &str, contract_address: &str) {
    let _ = ::keplr::suggest_token(chain_id, contract_address).await;

    // if you want to handle the error case (user closes the pop up):

    // let result = ::keplr::suggest_token(chain_id, contract_address)
    //     .await
    //     .map_err(|js_value| {
    //         let error = Error::from(js_value)
    //             .message()
    //             .as_string()
    //             .unwrap_or("unknown error".to_string());
    //         error
    //     });
    //
    // match result {
    //     Ok(_) => log!("token added?"),
    //     Err(ref e) => error!("{e}"),
    // }
}

#[component]
pub fn KeplrTests() -> impl IntoView {
    log!("rendering <KeplrTests/>");

    on_cleanup(|| {
        log!("cleaning up <KeplrTests/>");
    });

    let dialog_ref = NodeRef::<Dialog>::new();

    let enable_keplr_action: Action<(), bool, SyncStorage> =
        Action::new_unsync_with_value(Some(false), |_: &()| enable_keplr(CHAIN_ID));
    let get_account_action: Action<(), Account, SyncStorage> =
        Action::new_unsync(|_: &()| get_account(CHAIN_ID));
    let get_key_action: Action<(), KeyInfo, SyncStorage> =
        Action::new_unsync(|_: &()| get_key(CHAIN_ID));
    let get_viewing_key_action: Action<String, String, LocalStorage> =
        Action::new_unsync(|input: &String| {
            let token_address = input.clone();
            get_secret_20_viewing_key(CHAIN_ID, token_address)
        });
    let suggest_token_action: Action<(), (), SyncStorage> = Action::new_unsync(move |_: &()| {
        suggest_token(CHAIN_ID, "secret1s09x2xvfd2lp2skgzm29w2xtena7s8fq98v852")
    });

    let suggest_chain_action: Action<(), (), SyncStorage> =
        Action::new_unsync(move |_: &()| suggest());

    let enable_keplr = move |_| enable_keplr_action.dispatch(());
    let get_account = move |_| get_account_action.dispatch(());
    let get_key = move |_| get_key_action.dispatch(());
    let get_viewing_key = move |_| {
        get_viewing_key_action.dispatch("secret1vkq022x4q8t8kx9de3r84u669l65xnwf2lg3e6".to_string())
    };
    let suggest_token = move |_| suggest_token_action.dispatch_local(());
    let suggest_chain = move |_| suggest_chain_action.dispatch_local(());

    // non-Actions
    let get_enigma_utils = move |_| get_enigma_utils(CHAIN_ID);
    let disable_keplr = move |_| {
        disable(CHAIN_ID);
        log!("Keplr Disabled")
    };

    // whether the call is pending
    let pending_enable = enable_keplr_action.pending();

    Effect::new(move |_| {
        if pending_enable.get() {
            let node = dialog_ref.get().expect("huh");
            let _ = node.show_modal();
        } else {
            let node = dialog_ref.get().expect("huh");
            node.close();
        }
    });

    // the most recent returned result
    let account = get_account_action.value();
    let key = get_key_action.value();
    let viewing_key = get_viewing_key_action.value();

    view! {
        <h2>"Keplr Tests"</h2>

        <div class="grid grid-cols-[auto,1fr] gap-x-4 gap-y-2">

            <button on:click=enable_keplr > "Enable" </button>
            <code class="font-mono max-w-max"> "window.keplr.enable(CHAIN_ID)" </code>

            <button on:click=suggest_chain > "Suggest Chain" </button>
            <code class="font-mono max-w-max"> "window.keplr.experimentalSuggestChain(...)" </code>

            <button on:click=get_account > "Get Account" </button>
            <code class="font-mono max-w-max"> "keplrOfflineSigner.getAccounts(CHAIN_ID)"</code>

            <button on:click=get_key > "Get Key" </button>
            <code class="font-mono max-w-max"> "window.keplr.getKey(CHAIN_ID)"</code>

            <button on:click=get_enigma_utils > "Get Enigma Utils" </button>
            <code class="font-mono max-w-max"> "window.keplr.getEnigmaUtils(CHAIN_ID)"</code>

            <button on:click=suggest_token > "Suggest Token (AMBER)" </button>
            <code class="font-mono max-w-max"> "window.keplr.suggestToken(CHAIN_ID, contract_address)"</code>

            <button on:click=get_viewing_key > "Get Viewing Key (USDC)" </button>
            <code class="font-mono max-w-max"> "window.keplr.getSecret20ViewingKey(CHAIN_ID, contract_address)"</code>

            <button on:click=disable_keplr > "Disable" </button>
            <code class="font-mono max-w-max"> "window.keplr.disable(CHAIN_ID)" </code>

        </div>

        <pre> { move || account.get().and_then(|value| Some(format!("{value:#?}"))) } </pre>
        <pre> { move || key.get().and_then(|value| Some(format!("{value:#?}"))) } </pre>
        <pre> { move || viewing_key.get().and_then(|value| Some(format!("Viewing Key: {value:#?}"))) } </pre>

        <dialog node_ref=dialog_ref>
            <p> "Waiting for Approval..." </p>
        </dialog>
    }
}

use ::keplr::suggest_chain_types::*;

pub async fn suggest() {
    let chain_info = SuggestingChainInfo {
        chain_id: "secretdev-1".to_string(),
        chain_name: "localsecret".to_string(),
        rpc: "http://127.0.0.1:26657".to_string(),
        rest: "http://127.0.0.1:1317".to_string(),
        bip44: Bip44 { coin_type: 529 },
        bech32_config: Bech32Config {
            bech32_prefix_acc_addr: "secret".to_string(),
            bech32_prefix_acc_pub: "secretpub".to_string(),
            bech32_prefix_val_addr: "secretvaloper".to_string(),
            bech32_prefix_val_pub: "secretvaloperpub".to_string(),
            bech32_prefix_cons_addr: "secretvalcons".to_string(),
            bech32_prefix_cons_pub: "secretvalconspub".to_string(),
        },
        currencies: vec![Currency {
            coin_denom: "SCRT".to_string(),
            coin_minimal_denom: "uscrt".to_string(),
            coin_decimals: 6,
            coin_gecko_id: "secret".to_string(),
        }],
        fee_currencies: vec![FeeCurrency {
            coin_denom: "SCRT".to_string(),
            coin_minimal_denom: "uscrt".to_string(),
            coin_decimals: 6,
            coin_gecko_id: "secret".to_string(),
            gas_price_step: GasPriceStep {
                low: 0.1,
                average: 0.25,
                high: 0.5,
            },
        }],
        stake_currency: Currency {
            coin_denom: "SCRT".to_string(),
            coin_minimal_denom: "uscrt".to_string(),
            coin_decimals: 6,
            coin_gecko_id: "secret".to_string(),
        },
    };

    let chain_info_js = serde_wasm_bindgen::to_value(&chain_info).unwrap();
    let _ = suggest_chain(chain_info_js).await;
}
