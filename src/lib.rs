#![allow(unused)]

use js_sys::{Object, JSON};
use leptos::{error::Result, html::Dialog, logging::log, *};
use leptos_router::*;
use state::MyAccount;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

pub mod constants;
mod keplr;
pub(crate) mod secret;
mod secretjs;
mod spinner;
mod state;
mod websockets;
use websockets::WebsocketDemo;

pub use constants::{CHAIN_ID, LCD_URL};
use keplr::KeplrTests;
pub use secret::Secret;
use secretjs::{ClientOptionsBuilder, SecretNetworkClient};
use state::GlobalState;

#[component]
pub fn App(debug: bool) -> impl IntoView {
    log::debug!("rendering <App/>");

    let (debug_mode, _) = create_signal(debug);
    // TODO - add the keplr/secretjs init stuff here?
    // TODO - look into saving/loading app state from localStorage
    //      - figure out localStorage interactions
    //      - do I use JSON or RON or what?

    // Passing Signals through Context
    // let (enabled, set_enabled) = create_signal(false);
    // provide_context(enabled);
    let ctx = GlobalState::new();
    provide_context(ctx);
    provide_context(MyAccount::new());

    let get_account_action = create_action(|_input: &()| async move { keplr::get_account().await });
    let get_account = move || get_account_action.dispatch(());
    let address = get_account_action.value();

    let connect_action = create_action(move |input: &()| async move {
        let address = keplr::get_account().await.unwrap_or_default();
        let keplr_offline_signer = keplr::get_offline_signer().unwrap();
        let encryption_utils = keplr::get_enigma_utils(CHAIN_ID);
        let client_options = secretjs::ClientOptionsBuilder::new()
            .url(LCD_URL)
            .chain_id(CHAIN_ID)
            .encryption_utils(encryption_utils)
            .wallet(keplr_offline_signer)
            .wallet_address(address.as_str())
            .build();
        let client = SecretNetworkClient::new(&client_options);
        log::debug!("{:#?}", &client.address());
        ctx.keplr_enabled.set(true);
        ctx.my_address.set(client.address());

        address
    });

    // let create_signing_client = move |_| {
    //     // Example running a Future
    //     // log!("running future");
    //     // leptos::spawn_local(async move {
    //     //     let address = keplr::get_account().await.unwrap_or_default();
    //     //     log!("inside future: {address}");
    //     //     ctx.my_address.set(address);
    //     // });
    //     // log!("future over");
    //
    //     log!("dispatch start");
    //     get_account_action.dispatch(());
    //     log!("dispatch end?");
    //
    //     let keplr_offline_signer = keplr::get_offline_signer().unwrap();
    //     let encryption_utils = keplr::get_enigma_utils(CHAIN_ID);
    //     let client_options = secretjs::ClientOptionsBuilder::new()
    //         .url(LCD_URL)
    //         .chain_id(CHAIN_ID)
    //         .encryption_utils(encryption_utils)
    //         .wallet(keplr_offline_signer)
    //         .wallet_address("secret12kw8ja5rgcxq66x3q48m9ec4n7g8a29xayprgy")
    //         .build();
    //     let client = SecretNetworkClient::new(&client_options);
    //     log::debug!("{:#?}", &client.address());
    //     ctx.keplr_enabled.set(true);
    //     ctx.my_address.set(client.address());
    // };

    let connect_button = move || {
        view! {
                <button
                    on:click=move |_| connect_action.dispatch(())
                    // class="btn w-48"
                >"Connect Ye Wallet"</button>
        }
    };

    view! {
        <div class="flex justify-between items-center">
            <h1>"Hello World"</h1>
        <Show
            when=move || ctx.keplr_enabled.get()
            fallback=connect_button
        >
            <p>"Yer Address is "<code>{connect_action.value()}</code></p>
        </Show>

            // cool button / notification badge animation
            // <span class="group relative inline-flex">
            //     <button class="btn" >"Connect Ye Wallet"</button>
            //     <span class="flex absolute h-3 w-3 top-0 right-0 -mt-1 -mr-1">
            //       <span class="group-hover:animate-ping absolute inline-flex h-full w-full rounded-full bg-sky-400 opacity-75"></span>
            //       <span class="relative inline-flex rounded-full h-3 w-3 bg-sky-500"></span>
            //     </span>
            // </span>
        </div>
        <Router>
            <main>
                <hr/>
                <nav>
                    <A exact=true href="/" >"Home"</A>
                    <A href="secret" >"Secret"</A>
                    <Show
                        when=debug_mode
                        fallback=|| view! { <meta/> }
                    >
                        <A href="keplr-tests" >"Keplr"</A>
                        <A href="websocket-tests" >"Websocket Demo"</A>
                    </Show>
                </nav>
                <hr/>
                <AnimatedRoutes
                    intro="slideIn"
                    outro="fadeOut"
                    intro_back="slideInBack"
                    outro_back="fadeOut"
                >
                    <Route
                        path="/"
                        view=|| {
                            view! { <Home/> }
                        }
                    />
                    <Route
                        path="secret"
                        view=|| view! { <Secret/> }
                    />
                    <Route
                        path="keplr-tests"
                        view=|| view! {
                            <KeplrTests/>
                            <hr/>
                            <IntegrationTests/>
                            <hr/>
                            <h2>"UI Tests"</h2>
                            <Modal/>
                        }
                    />
                    <Route
                        path="websocket-tests"
                        view=|| view! {
                            <WebsocketDemo/>
                        }
                    />
                </AnimatedRoutes>
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
pub fn IntegrationTests() -> impl IntoView {
    let ctx = use_context::<MyAccount>().expect("there should be a MyAccount context provided");
    let my_client = ctx.my_client.get_untracked();

    let create_random_wallet = move |_| {
        log::debug!("trying to create wallet...");

        let wallet = crate::secretjs::Wallet::new();

        log::debug!("{:#?}", &wallet);
    };
    fn create_readonly_client() {
        log::debug!("trying to create client...");

        let client_options = ClientOptionsBuilder::new()
            .url(LCD_URL)
            .chain_id(CHAIN_ID)
            .build();
        let client = SecretNetworkClient::new(&client_options);

        log::debug!("{:#?}", &client);
    }

    fn create_signing_client() -> SecretNetworkClient {
        log::debug!("trying to create client...");

        let keplr_offline_signer = keplr::get_offline_signer().unwrap();
        let encryption_utils = keplr::get_enigma_utils(CHAIN_ID);
        let client_options = secretjs::ClientOptionsBuilder::new()
            .url(LCD_URL)
            .chain_id(CHAIN_ID)
            .encryption_utils(encryption_utils)
            .wallet(keplr_offline_signer)
            .wallet_address("secret12kw8ja5rgcxq66x3q48m9ec4n7g8a29xayprgy")
            .build();
        let client = SecretNetworkClient::new(&client_options);

        log::debug!("{:#?}", &client);

        client
    }

    async fn do_a_query(client: SecretNetworkClient) -> Result<String> {
        // let client = create_signing_client();

        // ------------------------------------------------------------
        let query = Object::new();
        // query = {}
        //
        let _ = js_sys::Reflect::set(&query, &JsValue::from_str("token_info"), &Object::new());
        // query = {"token_info": {}}

        // ------------------------------------------------------------

        // { code_hash?: string; contract_address: string; query: T }
        let query_contract_request = Object::new();

        let _ = js_sys::Reflect::set(
            &query_contract_request,
            &JsValue::from_str("code_hash"),
            &JsValue::from_str("5a085bd8ed89de92b35134ddd12505a602c7759ea25fb5c089ba03c8535b3042"),
        );

        let _ = js_sys::Reflect::set(
            &query_contract_request,
            &JsValue::from_str("contract_address"),
            &JsValue::from_str("secret1s09x2xvfd2lp2skgzm29w2xtena7s8fq98v852"),
        );

        let _ = js_sys::Reflect::set(&query_contract_request, &JsValue::from_str("query"), &query);

        log::debug!("query_contract_request: {:#?}", &query_contract_request);

        let query_promise = client
            .query()
            .compute()
            .query_contract(&query_contract_request.into());
        let query_js_value = JsFuture::from(query_promise).await;

        match query_js_value {
            Ok(js_value) => {
                log::debug!("{js_value:#?}");
                Ok(JSON::stringify(&js_value).unwrap().into())
            }
            Err(js_error) => {
                log::debug!("{js_error:#?}");
                Ok(JSON::stringify(&js_error).unwrap().into())
            }
        }
    }

    let query_action = create_action(|input: &SecretNetworkClient| {
        let input = input.to_owned();
        async move { do_a_query(input).await }
    });
    let dispatch_query_action = move |_| query_action.dispatch(ctx.my_client.get());
    let pending_query = query_action.pending();
    let query_response = query_action.value();

    let create_readonly_client = move |_| {
        create_readonly_client();
    };

    let create_signing_client = move |_| {
        create_signing_client();
    };

    view! {
        <h2>"SecretJS Tests"</h2>

        <button on:click=create_random_wallet >
        "create_random_wallet"
        </button>
        <br/>
        <button on:click=create_readonly_client >
        "create_readonly_client"
        </button>
        <br/>
        <button on:click=create_signing_client >
        "create_signing_client"
        </button>
        <br/>
        <button on:click=dispatch_query_action >
        "dispatch_query_action"
        </button>
        <br/>
        <Show when=move || query_response().is_some() fallback=|| ()>
        <p>
            "Response: "
            <code>
                {move || {
                    match query_response() {
                        Some(resp) => resp.unwrap(),
                        None => "".to_string(),
                    }
                } }
            </code>
        </p>
        </Show>
    }
}

#[component]
pub fn Modal(// Signal that will be toggled when the button is clicked.
    // setter: WriteSignal<bool>,
) -> impl IntoView {
    log::debug!("rendering <Modal/>");

    on_cleanup(|| {
        log!("cleaning up <Modal/>");
    });

    // Example using signal as prop
    // setter.set(true);

    // Example showing how to use context
    // let getter =
    //     use_context::<ReadSignal<bool>>().expect("there to be an 'enabled' signal provided");

    // Example using a GlobalState struct as context
    let ctx = use_context::<GlobalState>().expect("provide global state context");

    let dialog_ref = create_node_ref::<Dialog>();

    let open_modal = move |_| {
        log::debug!("show modal");
        let node = dialog_ref.get().unwrap();
        node.show_modal().expect("I don't know what I expected");

        // Example using signal as prop
        // setter.update(|value| *value = !*value);

        // Example showing how to use context
        // ctx.enabled.update(|value| *value = !*value);
    };
    let close_modal = move |_| {
        log::debug!("close modal");
        let node = dialog_ref.get().unwrap();
        node.close();
    };

    view! {
        <dialog
            _ref=dialog_ref
        >
            <p>"Greetings, one and all!"</p>
            // <p>"Keplr enabled: "{getter}</p>
            <p>"Connected?: "{move || ctx.keplr_enabled.get()}</p>
            <p>"Address: "{move || ctx.my_address}</p>
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
