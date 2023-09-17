pub mod fetch;

use fetch::FetchExperiment;
use leptos::{logging::log, *};

#[component]
pub fn Secret() -> impl IntoView {
    log::debug!("rendering <Secret/>");

    on_cleanup(|| {
        log!("cleaning up <Secret/>");
    });

    view! {
        <h2>Secret Component</h2>
        <FetchExperiment/ >
    }
}
