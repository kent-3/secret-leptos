// #![allow(unused)]

use ::keplr::{
    keplr_sys::{self, KEPLR},
    Keplr, KeyInfo,
};
use codee::string::FromToStringCodec;
use leptos::{html::Dialog, logging::log, prelude::*};
use leptos_router::{
    components::{Route, Router, Routes, A},
    StaticSegment,
};
use leptos_use::storage::use_local_storage;
use rsecret::{
    query::{bank::BankQuerier, compute::ComputeQuerier},
    secret_network_client::CreateQuerierOptions,
};
use serde::Deserialize;
use serde::Serialize;
use tonic_web_wasm_client::Client;
use tracing::{debug, error, info};

mod components;
mod constants;
mod keplr;
mod state;

use components::Spinner2;
use constants::{CHAIN_ID, GRPC_URL, LCD_URL};
use keplr::KeplrTests;
use state::GlobalState;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    TokenInfo {},
    MemberCode { address: String, key: String },
    ValidCodes { codes: Vec<String> },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum QueryAnswer {
    TokenInfo {
        name: String,
        symbol: String,
        decimals: u8,
        total_supply: String,
    },
    MemberCode {
        code: String,
    },
    ValidCodes {
        codes: Vec<String>,
    },
    ViewingKeyError {
        msg: String,
    },
}

#[component]
pub fn App() -> impl IntoView {
    log!("rendering <App/>");

    // Node references

    let dialog_ref = NodeRef::<Dialog>::new();

    // Event Listeners

    let keplr_keystorechange_handle =
        window_event_listener_untyped("keplr_keystorechange", move |_| {
            log!("Key store in Keplr is changed. You may need to refetch the account info.");
        });

    // Storage Signals

    let (is_keplr_enabled, set_keplr_enabled, _remove_flag) =
        use_local_storage::<bool, FromToStringCodec>("keplr_enabled");

    // Local Functions

    async fn enable_keplr(chain_id: &str) -> bool {
        if KEPLR.is_undefined() || KEPLR.is_null() {
            window()
                .alert_with_message("keplr not found")
                .expect("alert failed");
            // false
            Keplr::enable(chain_id).await.is_ok()
        } else {
            debug!("Trying to enable Keplr...");
            Keplr::enable(chain_id).await.is_ok()
        }
    }

    // Actions

    let enable_keplr_action: Action<(), bool, SyncStorage> =
        Action::new_unsync_with_value(Some(is_keplr_enabled.get()), |_: &()| {
            enable_keplr(CHAIN_ID)
        });

    // on:click handlers

    let enable_keplr = move |_| {
        enable_keplr_action.dispatch(());
    };

    let disable_keplr = move |_| {
        keplr_sys::disable(CHAIN_ID);
        enable_keplr_action.value().set(Some(false));
    };

    // Effects

    // open the dialog whenever the "enable_keplr_action" is pending
    Effect::new(move |_| match dialog_ref.get() {
        Some(dialog) => match enable_keplr_action.pending().get() {
            true => {
                let _ = dialog.show_modal();
            }
            false => dialog.close(),
        },
        None => (),
    });

    // modify local storage any time the "enable_keplr_action" value changes
    Effect::new(move |_| match enable_keplr_action.value().get() {
        Some(status) => {
            match status {
                true => info!("Keplr is Enabled"),
                false => info!("Keplr is Disabled"),
            }
            set_keplr_enabled.set(status);
            debug!("set 'keplr_enabled={status}' in local storage");
        }
        None => (),
    });

    // Passing Signals through Context

    // let keplr_ctx = KeplrActions {
    //     enable_keplr: enable_keplr_action,
    // };
    //
    // provide_context(keplr_ctx);

    on_cleanup(move || keplr_keystorechange_handle.remove());

    // HTML Elements

    let connect_button = move || {
        view! {
            <button on:click=enable_keplr disabled=enable_keplr_action.pending()>
                Connect Wallet
            </button>
        }
    };

    let disconnect_button = move || {
        view! { <button on:click=disable_keplr>Disconnect Wallet</button> }
    };

    view! {
        <Router>
            <header>
                <div class="flex justify-between items-center">
                    <h1>"Hello World"</h1>
                    <Show when=move || !is_keplr_enabled.get() fallback=disconnect_button>
                        {connect_button}
                    </Show>
                </div>
                <hr />
                <nav>
                    <A exact=true href="/">
                        "Home"
                    </A>
                    <A href="keplr">"Keplr"</A>
                </nav>
                <hr />
            </header>
            <main class="outline outline-1 outline-offset-4 outline-neutral-500">
                <Routes fallback=|| "This page could not be found.">
                    <Route path=StaticSegment("/") view=|| view! { <Home /> } />
                    <Route path=StaticSegment("keplr") view=|| view! { <KeplrTests /> } />
                </Routes>
            </main>
            <dialog node_ref=dialog_ref class="flex items-center">
                <div class="inline-flex items-center">
                    <Spinner2 size="h-8 w-8" />
                    <div class="font-bold">Requesting Connection</div>
                </div>
            </dialog>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    use send_wrapper::SendWrapper;

    log!("rendering <Home/>");

    let (is_keplr_enabled, set_keplr_enabled, _remove_flag) =
        use_local_storage::<bool, FromToStringCodec>("keplr_enabled");

    // whenever the key store changes, this will re-set 'is_keplr_enabled' to true, triggering a
    // reload of everything subscribed to that signal
    // maybe not a good idea if that event is emitted when keplr is disabled (need to check)
    let keplr_keystorechange_handle =
        window_event_listener_untyped("keplr_keystorechange", move |_| {
            set_keplr_enabled.set(true);
        });

    on_cleanup(move || keplr_keystorechange_handle.remove());

    let user_key = Resource::new(is_keplr_enabled, move |value| {
        SendWrapper::new(async move {
            if value {
                let result = Keplr::get_key(CHAIN_ID).await;

                match result {
                    Ok(ref key_info) => debug!("{key_info:#?}"),
                    Err(ref e) => log!("{e}"),
                }

                let key_info = result.unwrap_or_default();
                // format!("{key_info:#?}")
                key_info
            } else {
                // String::new()
                KeyInfo::default()
            }
        })
    });

    let client = Client::new("https://grpc.mainnet.secretsaturn.net".to_string());
    let bank = BankQuerier::new(client.clone());
    let user_balance = Resource::new(
        move || user_key.get(),
        move |key| {
            let bank = bank.clone();
            SendWrapper::new(async move {
                if let Some(key) = key {
                    let result = match bank.balance(key.bech32_address, "uscrt").await {
                        Ok(balance) => {
                            log!("{balance:#?}");
                            balance.balance.unwrap().amount
                        }
                        Err(error) => {
                            error!("{error}");
                            format!("{error}")
                        }
                    };
                    result
                } else {
                    String::new()
                }
            })
        },
    );

    let encryption_utils = secretrs::EncryptionUtils::new(None, "secret-4").unwrap();
    let options = CreateQuerierOptions {
        url: "https://grpc.mainnet.secretsaturn.net",
        chain_id: CHAIN_ID,
        encryption_utils,
    };
    let compute = ComputeQuerier::new(client.clone(), options);

    let contract_address = "secret1s09x2xvfd2lp2skgzm29w2xtena7s8fq98v852";
    let code_hash = "9a00ca4ad505e9be7e6e6dddf8d939b7ec7e9ac8e109c8681f10db9cacb36d42";
    let query = QueryMsg::TokenInfo {};

    let snip_balance = Resource::new(
        move || user_key.get(),
        move |key| {
            let compute = compute.clone();
            let query = query.clone();
            SendWrapper::new(async move {
                if let Some(key) = key {
                    let result = match compute
                        .query_secret_contract(contract_address, code_hash, query)
                        .await
                    {
                        Ok(response) => {
                            log!("{response:#?}");
                            response
                        }
                        Err(error) => {
                            error!("{error}");
                            format!("{error}")
                        }
                    };
                    result
                } else {
                    String::new()
                }
            })
        },
    );

    view! {
        <Show when=move || is_keplr_enabled.get() fallback=|| view! { <p>Nothing to see here</p> }>
            <pre>{move || format!("{:#?}", user_key.get().unwrap_or_default()) }</pre>
            { move || user_balance.get() }
            { move || snip_balance.get() }
        </Show>
    }
}

#[component]
fn Modal(// Signal that will be toggled when the button is clicked.
    // setter: WriteSignal<bool>,
) -> impl IntoView {
    log!("rendering <Modal/>");

    on_cleanup(|| {
        log!("cleaning up <Modal/>");
    });

    // Examples using write signal as prop
    // setter.set(true);
    // setter.update(|value| *value = !*value);

    // Example using read signal from context
    // let getter =
    //     use_context::<ReadSignal<bool>>().expect("there to be an 'enabled' signal provided");

    // Example using a GlobalState struct as context
    let ctx = use_context::<GlobalState>().expect("provide global state context");
    let is_keplr_enabled = move || ctx.keplr_enabled.read_only();
    let my_address = move || ctx.my_address.read_only();

    // Creating a NodeRef allows using methods on the HtmlElement directly
    let dialog_ref = NodeRef::<Dialog>::new();

    let open_modal = move |_| {
        log!("show modal");
        let node = dialog_ref.get().unwrap();
        node.show_modal().expect("I don't know what I expected");

        // Example using context
        // ctx.keplr_enabled.update(|value| *value = !*value);
    };
    let close_modal = move |_| {
        log!("close modal");
        let node = dialog_ref.get().unwrap();
        node.close();
    };

    view! {
        <dialog node_ref=dialog_ref>
            <p>"Connected?: "{is_keplr_enabled}</p>
            <p>"Address: "{my_address}</p>
            <button on:click=close_modal>"OK"</button>
        </dialog>
        <button on:click=open_modal>"Example Modal"</button>
    }
}
