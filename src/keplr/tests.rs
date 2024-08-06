use ::keplr::*;
use js_sys::Error;
use keplr_sys::disable;
use leptos::html::Dialog;
use leptos::prelude::*;
use leptos::web_sys::console;

fn alert(msg: impl AsRef<str>) {
    let _ = window().alert_with_message(msg.as_ref());
}

// if keplr is enabled, this will return Ok(())
// otherwise, this will return Err(String) with the reason for the error
async fn enable_keplr(chain_id: &str) -> Result<(), String> {
    log!("Trying to enable Keplr...");
    let result = Keplr::enable(chain_id).await.map_err(|js_value| {
        let error = Error::from(js_value)
            .message()
            .as_string()
            .unwrap_or("unknown error".to_string());
        error
    });
    match result {
        Ok(_) => log!("Keplr is enabled."),
        Err(ref e) => log!("{e}"),
    }
    result
}

// this method seems dumb since `get_key` returns the same and more
async fn get_account(chain_id: &str) -> String {
    let signer = Keplr::get_offline_signer_only_amino(chain_id);
    let accounts = signer.get_accounts().await;
    let accounts = js_sys::Array::from(&accounts);
    let account = accounts.get(0);

    let account: Account = serde_wasm_bindgen::from_value(account).unwrap();
    log!("{account:#?}");

    account.address
}

async fn get_key(chain_id: &str) -> KeyInfo {
    let key_info = Keplr::get_key(chain_id).await;

    log!("{key_info:#?}");

    key_info
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

#[component]
pub fn KeplrTests() -> impl IntoView {
    let dialog_ref = NodeRef::<Dialog>::new();

    let enable_keplr_action: Action<(), std::result::Result<(), String>, SyncStorage> =
        Action::new_unsync(move |_: &()| async move {
            if let Some(dialog) = dialog_ref.get_untracked() {
                let _ = dialog.show_modal();
            }

            let result = enable_keplr("secret-4").await;

            if let Some(dialog) = dialog_ref.get_untracked() {
                dialog.close();
            }

            result
        });
    let get_account_action: Action<(), String, SyncStorage> =
        Action::new_unsync(|_: &()| get_account("secret-4"));
    let get_key_action: Action<(), KeyInfo, SyncStorage> =
        Action::new_unsync(|_: &()| get_key("secret-4"));
    let get_viewing_key_action: Action<String, String, LocalStorage> =
        Action::new_unsync(|input: &String| {
            let token_address = input.clone();
            get_secret_20_viewing_key("secret-4", token_address)
        });

    let enable_keplr = move |_| enable_keplr_action.dispatch(());
    let get_account = move |_| get_account_action.dispatch(());
    let get_key = move |_| get_key_action.dispatch(());
    let get_viewing_key = move |_| {
        get_viewing_key_action.dispatch("secret1vkq022x4q8t8kx9de3r84u669l65xnwf2lg3e6".to_string())
    };

    // non-Actions
    let get_enigma_utils = move |_| get_enigma_utils("secret-4");
    let disable_keplr = move |_| {
        disable("secret-4");
        log!("Keplr Disabled")
    };

    // whether the call is pending
    let pending_enable = enable_keplr_action.pending();

    // the most recent returned result
    let enabled = enable_keplr_action.value();
    let address = get_account_action.value();
    let key = get_key_action.value();
    let viewing_key = get_viewing_key_action.value();

    // how many times the action has run
    // useful for reactively updating something else in response to a `dispatch` and response
    let version = enable_keplr_action.version();

    view! {
        <h2>"Keplr Tests"</h2>

        <div class="grid grid-cols-[auto,1fr] gap-x-4 gap-y-2">

            <button on:click=enable_keplr > "Enable" </button>
            <code class="font-mono max-w-max"> "window.keplr.enable(CHAIN_ID)" </code>

            <button on:click=get_account > "Get Account" </button>
            <code class="font-mono max-w-max"> "keplrOfflineSigner.getAccounts(CHAIN_ID)"</code>

            <button on:click=get_key > "Get Key" </button>
            <code class="font-mono max-w-max"> "window.keplr.getKey(CHAIN_ID)"</code>

            <button on:click=get_enigma_utils > "Get Enigma Utils" </button>
            <code class="font-mono max-w-max"> "window.keplr.getEnigmaUtils(CHAIN_ID)"</code>

            <button on:click=get_viewing_key > "Get Viewing Key (USDC)" </button>
            <code class="font-mono max-w-max"> "window.keplr.getSecret20ViewingKey(CHAIN_ID, contract_address)"</code>

            <button on:click=disable_keplr > "Disable" </button>
            <code class="font-mono max-w-max"> "window.keplr.disable(CHAIN_ID)" </code>

        </div>

        // Example of how to show a dialog while an action is pending
        // <Show
        //     when=pending_enable
        //     fallback=|| ()
        // >
            <dialog node_ref=dialog_ref >
                <p> "Waiting for Approval..." </p>
            </dialog>
        // </Show>
    }
}
