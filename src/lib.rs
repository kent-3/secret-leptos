#![allow(unused)]

use rsecret::secret_network_client::CreateQuerierOptions;

// use futures::lock::Mutex;
use leptos::{html::Dialog, logging::log, prelude::*};
use leptos_router::components::{Route, Router, Routes, A};
use leptos_router::StaticSegment;
use serde::{Deserialize, Serialize};
// use state::MyAccount;
use std::sync::{Arc, Mutex};

pub mod components;
pub mod constants;
// mod demo;
mod keplr;
// mod secretjs;
mod state;

pub use constants::{CHAIN_ID, GRPC_URL, LCD_URL};
// use demo::{QueryDemo, WebsocketDemo};
use keplr::KeplrTests;
// use secretjs::SecretJsTests;
// use secretjs::{ClientOptionsBuilder, SecretNetworkClient};
use state::GlobalState;

use base64::prelude::{Engine, BASE64_STANDARD};
// use secretrs::{
//     clients::{AuthQueryClient, ComputeQueryClient, TendermintQueryClient},
//     proto::cosmos::auth::v1beta1::QueryParamsRequest,
//     proto::secret::compute::v1beta1::{QueryByContractAddressRequest, QuerySecretContractRequest},
//     EncryptionUtils,
// };
// use state::SecretQueryClient;
use thiserror::Error;
use wasm_bindgen::UnwrapThrowExt;

#[component]
pub fn App(demo: bool) -> impl IntoView {
    log!("rendering <App/>");

    let (demo_mode, _) = signal(demo);

    // TODO - look into saving/loading app state from localStorage
    //      - figure out localStorage interactions
    //      - do I use JSON or RON or what?
    //
    // let local_storage: Option<GlobalState> = read_local_storage();
    // let ctx = local_storage.unwrap_or_else(|| GlobalState::new());

    // Passing Signals through Context
    let ctx = GlobalState::new();
    provide_context(ctx);
    // provide_context(MyAccount::new());

    // let secret = SecretQueryClient::new();
    // provide_context(secret);

    let keplr_is_enabled = move || ctx.keplr_enabled.get();

    // log::debug!("Creating Clients");
    // let web_client = ::tonic_web_wasm_client::Client::new(GRPC_URL.to_string());

    // let client_options = rsecret::CreateClientOptions {
    //     url: GRPC_URL,
    //     chain_id: CHAIN_ID,
    //     ..Default::default()
    // };
    // let secretrs_master =
    //     Arc::new(rsecret::SecretNetworkClient::new(web_client, client_options).unwrap());

    // let secretrs = Arc::clone(&secretrs_master);
    // let get_auth_params_action = Action::new(move |_: &()| {
    //     // let secretrs = Arc::clone(&secretrs);
    //     async move {
    //         let web_client = ::tonic_web_wasm_client::Client::new(GRPC_URL.to_string());
    //         let auth = rsecret::query::auth::AuthQuerier::new(web_client);
    //         let response = auth.params().await;
    //         match response {
    //             Ok(result) => log::debug!("{:#?}", result),
    //             Err(status) => log::error!("{}", status),
    //         }
    //     }
    // });

    // log::debug!("    Auth");
    // let mut auth_client = AuthQueryClient::new(web_client.clone());
    // log::debug!("    Tendermint");
    // let mut tendermint_client = TendermintQueryClient::new(web_client.clone());
    // log::debug!("    Compute");
    // let mut compute_client = ComputeQueryClient::new(web_client.clone());
    //
    // let get_auth_params_action = create_action(move |_: &()| {
    //     let mut auth = auth_client.clone();
    //     async move {
    //         let response = auth.params(QueryParamsRequest {}).await;
    //         match response {
    //             Ok(result) => log::debug!("{:#?}", result.into_inner()),
    //             Err(status) => log::error!("{}", status),
    //         }
    //     }
    // });
    //
    // let get_auth_params_button = move || {
    //     view! {
    //             <button
    //                 on:click=move |_| get_auth_params_action.dispatch(())
    //             >"TEST"</button>
    //     }
    // };

    // let secretrs = Arc::clone(&secretrs_master);
    let query_contract_action = Action::new(move |_: &()| {
        // let secretrs = Arc::clone(&secretrs);
        async move {
            let client_options = CreateQuerierOptions {
                url: GRPC_URL,
                chain_id: CHAIN_ID,
                encryption_utils: todo!(),
            };
            let web_client = ::tonic_web_wasm_client::Client::new(GRPC_URL.to_string());
            let compute = rsecret::query::compute::ComputeQuerier::new(web_client, client_options);

            let contract_address = "secret1s09x2xvfd2lp2skgzm29w2xtena7s8fq98v852".to_string();

            log!("Computing code hash...");
            let response = compute
                .code_hash_by_contract_address(contract_address.clone())
                .await;

            match response {
                Ok(ref code_hash) => log!("{}", code_hash),
                Err(ref error) => log!("{}", error),
            }

            let code_hash = response.unwrap();
            log!("code_hash => {}", code_hash);

            let code_hash = "9a00ca4ad505e9be7e6e6dddf8d939b7ec7e9ac8e109c8681f10db9cacb36d42";

            let query = QueryMsg::MemberCode {
                address: "secret1jj30ulmuxem55awzhfnr802ml7rddufe0jadf7".to_string(),
                key: "amber-rocks".to_string(),
            };

            log!("Querying contract...");
            let response = compute
                .query_secret_contract(contract_address, code_hash, query)
                .await;

            match response {
                Ok(result) => {
                    log!("{}", result);
                }
                Err(error) => log!("{}", error),
            }
        }
    });

    let query_contract_button = move || {
        view! {
                <button
                    on:click=move |_| query_contract_action.dispatch(())
                >"TEST"</button>
        }
    };

    let connect_action = Action::new(move |_: &()| async move {
        todo!()

        // let address = keplr::get_account().await.unwrap_or_default();
        // let keplr_offline_signer = keplr::get_offline_signer().unwrap();
        // let encryption_utils = keplr::get_enigma_utils(CHAIN_ID);
        // let client_options = secretjs::ClientOptionsBuilder::new()
        //     .url(LCD_URL)
        //     .chain_id(CHAIN_ID)
        //     .encryption_utils(encryption_utils)
        //     .wallet(keplr_offline_signer)
        //     .wallet_address(address.as_str())
        //     .build();
        // let client = SecretNetworkClient::new(&client_options);
        // log::debug!("{:#?}", &client.address());
        // ctx.keplr_enabled.set(true);
        // ctx.my_address.set(client.address());
        //
        // address
    });

    // let connect_button = move || {
    //     view! {
    //             <button
    //                 on:click=move |_| connect_action.dispatch(())
    //             >"Connect Ye Wallet"</button>
    //     }
    // };

    view! {
        <Router>
            <header>
                <div class="flex justify-between items-center">
                    <h1>"Hello World"</h1>
                    <Show
                        when=keplr_is_enabled
                        fallback=query_contract_button
                    >
                        <p>"Yer Address is "<code>{ctx.my_address.get()}</code></p>
                    </Show>
                </div>
                <hr/>
                <nav>
                    <A exact=true href="/" >"Home"</A>
                    <Show
                        when=demo_mode
                        fallback=|| ()
                    >
                        <A href="keplr-demo" >"Keplr"</A>
                        // <A href="query-demo" >"Query"</A>
                        // <A href="websocket-demo" >"Websocket"</A>
                    </Show>
                </nav>
                <hr/>
            </header>
            <main>
                <Routes
                    fallback=|| "This page could not be found."
                    // intro="slideIn"
                    // outro="fadeOut"
                    // intro_back="slideInBack"
                    // outro_back="fadeOut"
                >
                    <Route
                        path=StaticSegment("/")
                        view=|| {
                            view! { <Home/> }
                        }
                    />
                    <Route
                        path=StaticSegment("keplr-demo")
                        view=|| view! {
                            <KeplrTests/>
                            <hr/>
                            // <SecretJsTests/>
                            <hr/>
                            <h2>"UI Tests"</h2>
                            <Modal/>
                        }
                    />
                    // <Route
                    //     path="query-demo"
                    //     view=|| view! { <QueryDemo/> }
                    // />
                    // <Route
                    //     path="websocket-demo"
                    //     view=|| view! {
                    //         <WebsocketDemo/>
                    //     }
                    // />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <p>
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nam ac tincidunt nisl, nec faucibus arcu. Nullam venenatis mi justo, eget consequat eros iaculis nec. Nullam sed purus sem. Duis ac aliquam enim. Curabitur vitae urna lectus. Sed tincidunt quis est non consequat. Nunc pellentesque maximus eros eget rutrum. Interdum et malesuada fames ac ante ipsum primis in faucibus. Nam in massa pulvinar, varius massa quis, bibendum diam. Etiam non mi faucibus, pretium lacus eget, dignissim risus."
        </p>
        <p>
            "Phasellus in fermentum nisl, eget luctus urna. Nullam sed nulla urna. Morbi consequat, ante eget tincidunt faucibus, tortor purus elementum nulla, quis dignissim dui justo vel orci. Duis placerat neque sit amet velit consequat hendrerit. Proin arcu mauris, vestibulum ac enim sed, feugiat imperdiet turpis. Mauris id faucibus massa. In id scelerisque sapien, eu interdum metus. Quisque dui orci, viverra at sodales commodo, pellentesque a ipsum. In id pretium enim. Vestibulum arcu massa, blandit a condimentum ac, semper et sapien. Fusce posuere erat urna. Proin dictum nisi nec tortor mattis pretium at quis risus. Sed vel pellentesque orci."
        </p>
        <p>
            "Morbi non vestibulum magna, a iaculis eros. Donec eget quam nec dui vulputate efficitur. Morbi id ipsum suscipit, rhoncus nibh ut, cursus erat. Donec nec vestibulum risus, ultrices condimentum libero. Fusce vel nibh non eros viverra rutrum eu eu neque. Vestibulum vitae dignissim felis. Pellentesque a venenatis massa, sit amet molestie enim. Ut accumsan at sapien at tristique. Ut convallis, eros id venenatis euismod, lacus metus iaculis turpis, a mattis lorem elit sodales magna. Integer ullamcorper sodales erat. Vestibulum malesuada ullamcorper ex at ornare."
        </p>
        <p>
            "Sed sit amet egestas tortor, eget rutrum lorem. Duis et enim semper, molestie turpis a, scelerisque quam. Proin nec mi felis. Aliquam tincidunt dui purus, eu semper metus lobortis eu. Etiam dapibus dolor lacus, non molestie purus ultricies at. Quisque hendrerit, tellus nec pretium viverra, nisi metus convallis sapien, at lobortis turpis orci vitae lectus. Ut purus dui, convallis eu interdum sit amet, malesuada ac sapien. Etiam tristique luctus arcu et euismod. Donec sodales lacus eu eros pretium pretium non nec ligula. Integer cursus est et tellus iaculis laoreet. Phasellus eget sapien orci."
        </p>
        <p>
            "Duis sed cursus leo. Proin leo erat, viverra sed rutrum eu, sagittis a arcu. Phasellus in dolor scelerisque, elementum enim id, ultrices est. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Duis ullamcorper urna ac massa condimentum facilisis. Proin non ex in est dictum dapibus. Maecenas consequat auctor enim quis sollicitudin. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Maecenas cursus sagittis augue et tempor. Nam eget laoreet mauris."
        </p>
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
        <dialog
            node_ref=dialog_ref
        >
            <p>"Greetings, one and all!"</p>
            <p>"Connected?: "{is_keplr_enabled}</p>
            <p>"Address: "{my_address}</p>
            <button
                on:click=close_modal
            >
                "OK"
            </button>
        </dialog>
        <button
            on:click=open_modal
        >
            "Example Modal"
        </button>
    }
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
