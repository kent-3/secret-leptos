// #![allow(unused)]

// use codee::string::FromToStringCodec;
// use leptos_use::storage::use_local_storage;

use ::keplr::{keplr_sys, Keplr, KeyInfo};
use leptos::{
    html::{Dialog, Input},
    logging::log,
    prelude::*,
    spawn::spawn_local,
};
use leptos_router::{
    components::{Route, Router, Routes, A},
    StaticSegment,
};
use rsecret::{
    query::{bank::BankQuerier, compute::ComputeQuerier},
    secret_network_client::CreateQuerierOptions,
};
use send_wrapper::SendWrapper;
use serde::Deserialize;
use serde::Serialize;
use tonic_web_wasm_client::Client;
use tracing::{debug, error, info};
use wasm_bindgen::JsValue;

mod components;
mod constants;
mod keplr;
mod state;

use components::Spinner2;
use constants::{CHAIN_ID, GRPC_URL, LCD_URL};
use keplr::KeplrTests;
pub use state::{GlobalState, KeplrSignals, WasmClient};

// TODO: move custom types to seperate module

#[derive(Clone, Debug)]
pub struct Coin {
    pub denom: String,
    pub amount: String,
}

impl From<secretrs::proto::cosmos::base::v1beta1::Coin> for Coin {
    fn from(value: secretrs::proto::cosmos::base::v1beta1::Coin) -> Self {
        Self {
            denom: value.denom,
            amount: value.amount,
        }
    }
}

impl std::fmt::Display for Coin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.amount, self.denom)
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

#[component]
pub fn App() -> impl IntoView {
    log!("rendering <App/>");

    // Global Context

    let keplr = KeplrSignals::new();
    let wasm_client = WasmClient::new();

    provide_context(keplr);
    provide_context(wasm_client);

    // Event Listeners

    let keplr_keystorechange_handle =
        window_event_listener_untyped("keplr_keystorechange", move |_| {
            warn!("Key store in Keplr is changed. You may need to refetch the account info.");
        });

    // Signals related to provider and signer

    let keplr = use_context::<KeplrSignals>().expect("keplr signals context missing!");
    let wasm_client = use_context::<WasmClient>().expect("wasm client context missing!");

    // let update_grpc_url = move |_| {
    //     debug!("updating client_options.grpc_url");
    //     wasm_client.set(Client::new("https://foobar.com".to_string()))
    // };

    // Passing Signals through Context

    // provide_context(keplr);
    // provide_context(wasm_client);

    // Actions

    let enable_keplr_action: Action<(), bool, SyncStorage> =
        Action::new_unsync_with_value(Some(false), move |_: &()| async move {
            let keplr_extension = js_sys::Reflect::get(&window(), &JsValue::from_str("keplr"))
                .expect("unable to check for `keplr` property");

            if keplr_extension.is_undefined() || keplr_extension.is_null() {
                window()
                    .alert_with_message("keplr not found")
                    .expect("alert failed");
                false
            } else {
                debug!("Trying to enable Keplr...");
                match Keplr::enable(CHAIN_ID).await {
                    Ok(_) => {
                        keplr.enabled.set(true);
                        debug!("Keplr is enabled");
                        true
                    }
                    Err(e) => {
                        keplr.enabled.set(false);
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

    // let disable_keplr = move |_| {
    //     keplr_sys::disable(CHAIN_ID);
    //     keplr.enabled.set(false);
    //     keplr.key_info.set(None);
    // };

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

    let encryption_utils = secretrs::EncryptionUtils::new(None, "secret-4").unwrap();
    let options = CreateQuerierOptions {
        url: "https://grpc.mainnet.secretsaturn.net",
        chain_id: CHAIN_ID,
        encryption_utils,
    };
    // let compute = ComputeQuerier::new(client.clone(), options);

    let contract_address = "secret1s09x2xvfd2lp2skgzm29w2xtena7s8fq98v852";
    let code_hash = "9a00ca4ad505e9be7e6e6dddf8d939b7ec7e9ac8e109c8681f10db9cacb36d42";

    // let amber_balance = Resource::new(
    //     move || keplr.key_info.get(),
    //     move |key| {
    //         // let compute = compute.clone();
    //         debug!("wasm_client changed. running token_info resource");
    //         let compute = ComputeQuerier::new(wasm_client.get(), options.clone());
    //         SendWrapper::new(async move {
    //             // key not needed in this case, but we would need it for permissioned queries
    //             let query = QueryMsg::TokenInfo {};
    //             compute
    //                 .query_secret_contract(contract_address, code_hash, query)
    //                 .await
    //                 .map_err(|error| Error::GenericError(error.to_string()))
    //         })
    //     },
    // );

    on_cleanup(move || keplr_keystorechange_handle.remove());

    // HTML Elements

    let connect_button = move || {
        view! {
            <button on:click=enable_keplr disabled=enable_keplr_action.pending()>
                Connect Wallet
            </button>
        }
    };

    // let disconnect_button = move || {
    //     view! { <button on:click=disable_keplr>Disconnect Wallet</button> }
    // };

    let key_name = move || keplr.key_info.get().map(|key_info| key_info.name);

    view! {
        <Router>
            <header>
                <div class="flex justify-between items-center">
                    <h1>"Hello World"</h1>
                    <Show when=move || keplr.key_info.get().is_some() >
                        <p class="outline outline-2 outline-offset-8 outline-neutral-500">
                            Connected as <strong>{key_name}</strong>
                        </p>
                    </Show>
                    <Show when=move || keplr.enabled.get() fallback=connect_button>
                        <OptionsMenu />
                    </Show>
                </div>
                <hr />
                <nav>
                    <A exact=true href="/secret-leptos/">
                        "Home"
                    </A>
                    <A href="/secret-leptos/keplr">"Keplr"</A>
                </nav>
                <hr />
            </header>
            <main class="outline outline-1 outline-offset-8 outline-neutral-500">
                <Routes fallback=|| "This page could not be found.">
                    <Route path=StaticSegment("/secret-leptos/") view=|| view! { <Home /> } />
                    <Route path=StaticSegment("/secret-leptos/keplr") view=|| view! { <KeplrTests /> } />
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
pub fn OptionsMenu() -> impl IntoView {
    let dialog_ref = NodeRef::<Dialog>::new();
    let input_element = NodeRef::<Input>::new();

    let keplr = use_context::<KeplrSignals>().expect("keplr signals context missing!");
    let wasm_client = use_context::<WasmClient>().expect("wasm client context missing!");

    let disable_keplr = move |_| {
        keplr_sys::disable(CHAIN_ID);
        keplr.enabled.set(false);
        keplr.key_info.set(None);
    };

    let toggle_options_menu = move |_| match dialog_ref.get() {
        Some(dialog) => match dialog.open() {
            false => {
                let _ = dialog.show_modal();
            }
            true => dialog.close(),
        },
        None => {
            let _ = window().alert_with_message("Something is wrong!");
        }
    };

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        debug!("updating wasm_client grpc_url");

        // here, we'll extract the value from the input
        let value = input_element
            .get()
            // event handlers can only fire after the view
            // is mounted to the DOM, so the `NodeRef` will be `Some`
            .expect("<input> should be mounted")
            // `leptos::HtmlElement<html::Input>` implements `Deref`
            // to a `web_sys::HtmlInputElement`.
            // this means we can call`HtmlInputElement::value()`
            // to get the current value of the input
            .value();
        wasm_client.set(Client::new(value));
    };

    view! {
        <button on:click=toggle_options_menu>"Options"</button>
        <dialog node_ref=dialog_ref class="flex flex-col gap-4 items-center">
            <button on:click=toggle_options_menu class="self-stretch">"Close Menu"</button>
                <form class="flex gap-4" on:submit=on_submit>
                    <input type="text"
                        value=GRPC_URL
                        node_ref=input_element
                    />
                    <input type="submit" value="Submit" class="min-w-fit"/>
                </form>
            <button
                on:click=disable_keplr
                class="border-blue-500 text-blue-500 border-solid hover:bg-neutral-800 rounded-sm bg-[initial]"
            >
                Disconnect Wallet
            </button>
        </dialog>
    }
}

#[component]
fn Home() -> impl IntoView {
    log!("rendering <Home/>");

    let keplr = use_context::<KeplrSignals>().expect("keplr signals context missing!");
    let wasm_client = use_context::<WasmClient>().expect("wasm client context missing!");

    Effect::new(move |_| debug!("{:#?}", wasm_client.get()));

    // whenever the key store changes, this will re-set 'is_keplr_enabled' to true, triggering a
    // reload of everything subscribed to that signal
    let keplr_keystorechange_handle =
        window_event_listener_untyped("keplr_keystorechange", move |_| {
            keplr.enabled.set(true);
        });

    on_cleanup(move || keplr_keystorechange_handle.remove());

    Effect::new(move |_| {
        if keplr.enabled.get() {
            spawn_local(async move {
                let key_info = Keplr::get_key(CHAIN_ID).await.ok();
                keplr.key_info.set(key_info);
            })
        }
    });

    // let client = Client::new("https://grpc.mainnet.secretsaturn.net".to_string());
    // let bank = BankQuerier::new(client.clone());

    let (user_balance, set_user_balance) = signal::<Option<String>>(None);
    Effect::new(move |_| {
        let key = keplr.key_info.get();
        if let Some(key) = key {
            // let bank = bank.clone();
            let bank = BankQuerier::new(wasm_client.get());
            spawn_local(async move {
                match bank.balance(key.bech32_address, "uscrt").await {
                    Ok(balance) => {
                        log!("{balance:#?}");
                        let balance: Coin = balance.balance.unwrap().into();
                        set_user_balance.set(Some(balance.to_string()));
                    }
                    Err(error) => {
                        error!("{error}");
                        set_user_balance.set(None);
                    }
                };
            })
        }
    });

    let encryption_utils = secretrs::EncryptionUtils::new(None, "secret-4").unwrap();
    let options = CreateQuerierOptions {
        url: "https://grpc.mainnet.secretsaturn.net",
        chain_id: CHAIN_ID,
        encryption_utils,
    };
    // let compute = ComputeQuerier::new(client.clone(), options);

    let contract_address = "secret1s09x2xvfd2lp2skgzm29w2xtena7s8fq98v852";
    let code_hash = "9a00ca4ad505e9be7e6e6dddf8d939b7ec7e9ac8e109c8681f10db9cacb36d42";

    let token_info = Resource::new(
        move || wasm_client.url.get(),
        move |_| {
            // let compute = compute.clone();
            debug!("wasm_client changed. running token_info resource");
            let compute = ComputeQuerier::new(wasm_client.get(), options.clone());
            SendWrapper::new(async move {
                // key not needed in this case, but we would need it for permissioned queries
                let query = QueryMsg::TokenInfo {};
                compute
                    .query_secret_contract(contract_address, code_hash, query)
                    .await
                    .map_err(|error| Error::GenericError(error.to_string()))
            })
        },
    );

    view! {
        <Show when=move || keplr.enabled.get() fallback=|| view! { <p>Nothing to see here</p> }>
            <pre>{move || format!("{:#?}", keplr.key_info.get())}</pre>
            <Show when=move || user_balance.get().is_some() fallback=|| ()>
                {move || user_balance.get()}
            </Show>
            // the fallback receives a signal containing current errors
            <ErrorBoundary fallback=|errors| {
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
