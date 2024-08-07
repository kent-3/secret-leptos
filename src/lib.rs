#![allow(unused)]

use base64::prelude::{Engine, BASE64_STANDARD};
use leptos::{html::Dialog, logging::log, prelude::*};
use leptos_router::components::{Route, Router, Routes, A};
use leptos_router::StaticSegment;
use serde::{Deserialize, Serialize};

pub mod components;
pub mod constants;
mod keplr;
mod state;

pub use constants::{CHAIN_ID, GRPC_URL, LCD_URL};
use keplr::KeplrTests;
use state::GlobalState;

#[component]
pub fn App() -> impl IntoView {
    log!("rendering <App/>");

    // Passing Signals through Context
    let ctx = GlobalState::new();
    provide_context(ctx);

    view! {
        <Router>
            <header>
                <div class="flex justify-between items-center">
                    <h1>"Hello World"</h1>
                </div>
                <hr/>
                <nav>
                    <A exact=true href="/" >"Home"</A>
                    <A href="keplr" >"Keplr"</A>
                </nav>
                <hr/>
            </header>
            <main>
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
