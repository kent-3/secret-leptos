#![allow(unused)]

use base64::prelude::{Engine, BASE64_STANDARD};
use leptos::{html::Dialog, logging::log, prelude::*};
use leptos_router::components::{Route, Router, Routes, A};
use leptos_router::StaticSegment;
use serde::{Deserialize, Serialize};

mod components;
mod constants;
mod keplr;
mod state;

use components::Spinner2;
pub use constants::{CHAIN_ID, GRPC_URL, LCD_URL};
use keplr::KeplrTests;
use state::{GlobalState, KeplrState};

#[component]
pub fn App() -> impl IntoView {
    log!("rendering <App/>");

    // Passing Signals through Context
    let ctx = GlobalState::new();
    provide_context(ctx);

    // TODO: read from local storage if keplr is enabled

    use crate::keplr::enable_keplr;
    let enable_keplr_action: Action<(), bool, SyncStorage> =
        Action::new_unsync_with_value(Some(false), |_: &()| enable_keplr(CHAIN_ID));
    let enable_keplr = move |_| enable_keplr_action.dispatch(());
    let pending_enable = enable_keplr_action.pending();
    let is_keplr_enabled = enable_keplr_action.value().read_only();

    let dialog_ref = NodeRef::<Dialog>::new();
    Effect::new(move |_| {
        if pending_enable.get() {
            let node = dialog_ref.get().expect("huh");
            let _ = node.show_modal();
        } else {
            let node = dialog_ref.get().expect("huh");
            node.close();
        }
    });

    let keplr_ctx = KeplrState {
        enable_keplr_action,
        is_keplr_enabled,
    };
    provide_context(keplr_ctx);

    view! {
        <Router>
            <header>
                <div class="flex justify-between items-center">
                    <h1>"Hello World"</h1>
                    // <button class="btn inline-flex items-center" disabled=pending_enable>
                    //     <Spinner2/ >
                    //     Processing...
                    // </button>
                    <button on:click=enable_keplr disabled=pending_enable> Connect Wallet </button>
                </div>
                <hr/>
                <nav>
                    <A exact=true href="/" >"Home"</A>
                    <A href="keplr" >"Keplr"</A>
                </nav>
                <hr/>
            </header>
            <main class="outline outline-1 outline-offset-4 outline-neutral-500">
                <Routes fallback=|| "This page could not be found." >
                    <Route
                        path=StaticSegment("/")
                        view=|| view! { <Home/> }
                    />
                    <Route
                        path=StaticSegment("keplr")
                        view=|| view! { <KeplrTests/> }
                    />
                </Routes>
            </main>
            <dialog node_ref=dialog_ref>
                <p> "Waiting for Approval..." </p>
            </dialog>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {}
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
