#![allow(unused)]

use leptos::{html::Dialog, logging::log, *};
use leptos_router::*;
use state::MyAccount;

pub mod components;
pub mod constants;
mod demo;
mod keplr;
mod secretjs;
mod state;

pub use constants::{CHAIN_ID, LCD_URL};
use demo::{QueryDemo, WebsocketDemo};
use keplr::KeplrTests;
use secretjs::SecretJsTests;
use secretjs::{ClientOptionsBuilder, SecretNetworkClient};
use state::GlobalState;

#[component]
pub fn App(demo: bool) -> impl IntoView {
    log::debug!("rendering <App/>");

    let (demo_mode, _) = create_signal(demo);

    // TODO - look into saving/loading app state from localStorage
    //      - figure out localStorage interactions
    //      - do I use JSON or RON or what?
    //
    // let local_storage: Option<GlobalState> = read_local_storage();
    // let ctx = local_storage.unwrap_or_else(|| GlobalState::new());

    // Passing Signals through Context
    let ctx = GlobalState::new();
    provide_context(ctx);
    provide_context(MyAccount::new());

    let keplr_is_enabled = move || ctx.keplr_enabled.get();

    let connect_action = create_action(move |_: &()| async move {
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

    let connect_button = move || {
        view! {
                <button
                    on:click=move |_| connect_action.dispatch(())
                >"Connect Ye Wallet"</button>

            // cool button / notification badge animation
            // <span class="group relative inline-flex">
            //     <button class="btn" >"Connect Ye Wallet"</button>
            //     <span class="flex absolute h-3 w-3 top-0 right-0 -mt-1 -mr-1">
            //       <span class="group-hover:animate-ping absolute inline-flex h-full w-full rounded-full bg-sky-400 opacity-75"></span>
            //       <span class="relative inline-flex rounded-full h-3 w-3 bg-sky-500"></span>
            //     </span>
            // </span>
        }
    };

    view! {
        <Router>
            <header>
                <div class="flex justify-between items-center">
                    <h1>"Hello World"</h1>
                    <Show
                        when=keplr_is_enabled
                        fallback=connect_button
                    >
                        <p>"Yer Address is "<code>{ctx.my_address}</code></p>
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
                        <A href="query-demo" >"Query"</A>
                        <A href="websocket-demo" >"Websocket"</A>
                    </Show>
                </nav>
                <hr/>
            </header>
            <main>
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
                        path="keplr-demo"
                        view=|| view! {
                            <KeplrTests/>
                            <hr/>
                            <SecretJsTests/>
                            <hr/>
                            <h2>"UI Tests"</h2>
                            <Modal/>
                        }
                    />
                    <Route
                        path="query-demo"
                        view=|| view! { <QueryDemo/> }
                    />
                    <Route
                        path="websocket-demo"
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
fn Modal(// Signal that will be toggled when the button is clicked.
    // setter: WriteSignal<bool>,
) -> impl IntoView {
    log::debug!("rendering <Modal/>");

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
    let is_keplr_enabled = move || ctx.keplr_enabled;
    let my_address = move || ctx.my_address;

    // Creating a NodeRef allows using methods on the HtmlElement directly
    let dialog_ref = create_node_ref::<Dialog>();

    let open_modal = move |_| {
        log::debug!("show modal");
        let node = dialog_ref.get().unwrap();
        node.show_modal().expect("I don't know what I expected");

        // Example using context
        // ctx.keplr_enabled.update(|value| *value = !*value);
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
