pub mod fetch;
pub mod websockets;

pub use fetch::QueryDemo;
pub use websockets::WebsocketDemo;

// use leptos::{logging::log, *};

// #[component]
// pub fn Secret() -> impl IntoView {
//     log::debug!("rendering <Secret/>");
//
//     on_cleanup(|| {
//         log!("cleaning up <Secret/>");
//     });
//
//     view! {
//         <h2>Secret Component</h2>
//         <FetchExperiment/ >
//     }
// }
