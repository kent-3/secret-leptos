// #![allow(unused)]

// use codee::string::FromToStringCodec;
// use leptos_use::storage::use_local_storage;

use ::keplr::{keplr_sys, Keplr, KeyInfo};
use leptos::{html::Dialog, logging::log, prelude::*};
use leptos_router::{
    components::{Route, Router, Routes, A},
    StaticSegment,
};
use rsecret::{
    query::{bank::BankQuerier, compute::ComputeQuerier},
    secret_network_client::CreateQuerierOptions,
};
use serde::Deserialize;
use serde::Serialize;
use tonic_web_wasm_client::Client;
use tracing::{debug, error, info};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;

mod components;
mod constants;
mod keplr;
mod state;

use components::Spinner2;
use constants::{CHAIN_ID, GRPC_URL, LCD_URL};
use keplr::KeplrTests;
use state::{GlobalState, KeplrSignals};

pub fn is_keplr_available() -> bool {
    let keplr = js_sys::Reflect::get(&window(), &JsValue::from_str("keplr"))
        .expect("unable to check for `keplr` property");

    if keplr.is_undefined() || keplr.is_null() {
        false
    } else {
        true
    }
}

#[derive(Debug, Clone, thiserror::Error, Serialize, Deserialize)]
pub enum Error {
    #[error("An error occurred: {0}")]
    GenericError(String),
}

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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecretContext {
    pub is_extension_available: ReadSignal<bool>,
    pub extension_is_available: WriteSignal<bool>,
    pub is_extension_enabled: ReadSignal<bool>,
    pub extension_is_enabled: WriteSignal<bool>,
    pub user_data: ReadSignal<Option<KeyInfo>>,
    pub set_user_data: WriteSignal<Option<KeyInfo>>,
}

// impl SecretContext {
//     pub fn keplr() -> Self {
//         let (is_extension_available, extension_is_available) = signal(false);
//         let (is_extension_enabled, extension_is_enabled) = signal(false);
//         let (user_data, set_user_data) = signal(None);
//
//         // (raw way to access something on the window)
//         let keplr = js_sys::Reflect::get(&window(), &JsValue::from_str("keplr"))
//             .expect("unable to check for `keplr` property");
//
//         if !keplr.is_undefined() && !keplr.is_null() {
//             extension_is_available.set(true);
//         }
//
//         let service = Self {
//             is_extension_available,
//             extension_is_available,
//             is_extension_enabled,
//             extension_is_enabled,
//             user_data,
//             set_user_data,
//         };
//
//         // Can't do this here because it will trigger pop-up
//         // spawn_local(async move {
//         //     let _ = service.enable_keplr(CHAIN_ID, extension_is_enabled);
//         //     let _ = service.fetch_user_data().await;
//         // });
//
//         // window_event_listener_untyped("keplr_keystorechange", move |_| {
//         //     spawn_local(async move {
//         //         let _ = service.fetch_user_data().await;
//         //     });
//         // });
//
//         service
//     }
//
//     pub async fn enable_keplr(&self, chain_id: &str) {
//         if KEPLR.is_undefined() || KEPLR.is_null() {
//             window()
//                 .alert_with_message("keplr not found")
//                 .expect("alert failed");
//             self.extension_is_enabled.set(false);
//         } else {
//             match Keplr::enable(chain_id).await {
//                 Ok(_) => {
//                     self.extension_is_enabled.set(true);
//                     let _ = self.fetch_user_data();
//                     debug!("Keplr is enabled");
//                 }
//                 Err(e) => {
//                     self.extension_is_enabled.set(false);
//                     error!("{e}");
//                 }
//             }
//         }
//     }
//
//     pub fn disable_keplr(&self) {
//         keplr_sys::disable(CHAIN_ID);
//         self.extension_is_enabled.set(false);
//         self.set_user_data.set(None);
//         debug!("Keplr is Disabled");
//     }
//
//     pub async fn fetch_user_data(&self) -> Result<(), String> {
//         debug!("fetching user data");
//         match Keplr::get_key(CHAIN_ID).await {
//             Ok(key_info) => {
//                 debug!("{key_info:#?}");
//                 self.set_user_data.set(Some(key_info));
//                 Ok(())
//             }
//             Err(e) => {
//                 error!("{e}");
//                 Err(format!("{e}"))
//             }
//         }
//     }
// }

#[component]
pub fn App() -> impl IntoView {
    log!("rendering <App/>");

    // Event Listeners

    let keplr_keystorechange_handle =
        window_event_listener_untyped("keplr_keystorechange", move |_| {
            warn!("Key store in Keplr is changed. You may need to refetch the account info.");
        });

    // Signals related to Keplr

    let (is_enabled, enabled) = signal(false);
    let (user_data, set_user_data) = signal::<Option<KeyInfo>>(None);

    // Passing Signals through Context

    let keplr_context = KeplrSignals {
        is_enabled,
        enabled,
    };
    provide_context(keplr_context);

    // Actions

    let enable_keplr_action: Action<(), bool, SyncStorage> =
        Action::new_unsync_with_value(Some(false), move |_: &()| async move {
            let keplr = js_sys::Reflect::get(&window(), &JsValue::from_str("keplr"))
                .expect("unable to check for `keplr` property");

            if keplr.is_undefined() || keplr.is_null() {
                window()
                    .alert_with_message("keplr not found")
                    .expect("alert failed");
                false
            } else {
                debug!("Trying to enable Keplr...");
                match Keplr::enable(CHAIN_ID).await {
                    Ok(_) => {
                        enabled.set(true);
                        debug!("Keplr is enabled");
                        true
                    }
                    Err(e) => {
                        enabled.set(false);
                        error!("{e}");
                        false
                    }
                }
            }
        });

    // TODO: should I use the action value as the signal?
    // let is_enabled = enable_keplr_action.value().read_only();
    // let enabled = enable_keplr_action.value().write_only();

    // on:click handlers

    let enable_keplr = move |_| {
        enable_keplr_action.dispatch(());
    };

    let disable_keplr = move |_| {
        keplr_sys::disable(CHAIN_ID);
        enabled.set(false);
        set_user_data.set(None);
    };

    // Node references

    // let dialog_ref = NodeRef::<Dialog>::new();

    // Effects

    // open the dialog whenever the "enable_keplr_action" is pending
    // Effect::new(move |_| match dialog_ref.get() {
    //     Some(dialog) => match enable_keplr_action.pending().get() {
    //         true => {
    //             let _ = dialog.show_modal();
    //         }
    //         false => dialog.close(),
    //     },
    //     None => (),
    // });

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
                    <Show when=move || !is_enabled.get() fallback=disconnect_button>
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
            <LoadingModal when=enable_keplr_action.pending() message="Requesting Connection" />
        </Router>
    }
}

#[component]
pub fn LoadingModal(when: Memo<bool>, #[prop(into)] message: String) -> impl IntoView {
    let dialog_ref = NodeRef::<Dialog>::new();

    Effect::new_sync(move |_| match dialog_ref.get() {
        Some(dialog) => match when.get() {
            true => {
                let _ = dialog.show_modal();
            }
            false => dialog.close(),
        },
        None => (),
    });

    view! {
        <dialog node_ref=dialog_ref class="flex items-center">
            <div class="inline-flex items-center">
                <Spinner2 size="h-8 w-8" />
                <div class="font-bold">{message}</div>
            </div>
        </dialog>
    }
}

#[component]
fn Home() -> impl IntoView {
    use send_wrapper::SendWrapper;

    log!("rendering <Home/>");

    let keplr = use_context::<KeplrSignals>().expect("keplr signals context missing!");
    // let is_extension_enabled = keplr_ctx.is_enabled;
    // let extension_is_enabled = keplr_ctx.enabled;

    let (user_data, set_user_data) = signal::<Option<KeyInfo>>(None);

    // whenever the key store changes, this will re-set 'is_keplr_enabled' to true, triggering a
    // reload of everything subscribed to that signal
    let keplr_keystorechange_handle =
        window_event_listener_untyped("keplr_keystorechange", move |_| {
            keplr.enabled.set(true);
        });

    on_cleanup(move || keplr_keystorechange_handle.remove());

    let user_key = Resource::new(keplr.is_enabled, move |value| {
        SendWrapper::new(async move {
            if value {
                let result = Keplr::get_key(CHAIN_ID).await;

                match result {
                    Ok(ref key_info) => debug!("{key_info:#?}"),
                    Err(ref e) => log!("{e}"),
                }

                result.ok()
            } else {
                None
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
                if let Some(Some(key)) = key {
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

    let token_info_action = Action::new(move |_: &()| {
        debug!("running token_info action");
        let compute = compute.clone();
        SendWrapper::new(async move {
            // key not needed in this case, but we would need it for permissioned queries
            let query = QueryMsg::TokenInfo {};
            compute
                .query_secret_contract(contract_address, code_hash, query)
                .await
                .map_err(|error| Error::GenericError(error.to_string()))
        })
    });

    let token_info = token_info_action.value();

    view! {
        <Show when=move || keplr.is_enabled.get() fallback=|| view! { <p>Nothing to see here</p> }>
            <pre>{move || format!("{:#?}", user_key.get())}</pre>
            {move || user_balance.get()}
            <ErrorBoundary // the fallback receives a signal containing current errors
            fallback=|errors| {
                view! {
                    <div class="error">
                        <p>"Errors: "</p>
                        // we can render a list of errors as strings, if we'd like
                        <ul>
                            {move || {
                                errors
                                    .get()
                                    .into_iter()
                                    .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                                    .collect_view()
                            }}
                        </ul>
                    </div>
                }
            }>
                <p>{move || token_info.get()}</p>
            </ErrorBoundary>
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
