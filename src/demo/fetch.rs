//! Adapted from https://github.com/leptos-rs/leptos/tree/main/examples/fetch

use leptos::{error::Result, logging::log, *};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueryResponse {
    block_id: BlockId,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockId {
    hash: String,
}

#[derive(Error, Clone, Debug)]
pub enum QueryError {
    #[error("No query")]
    NoQuery,
}

type RawQuery = String;

async fn fetch_query(query: RawQuery) -> Result<String> {
    if !query.is_empty() {
        // make the request
        let res = gloo_net::http::Request::get(&format!("https://lcd.secret.express/{query}",))
            .send()
            .await?
            // convert it to JSON
            .json::<QueryResponse>()
            .await?
            .block_id
            .hash;
        Ok(format!("Latest Block Hash: {res}"))
    } else {
        Err(QueryError::NoQuery.into())
    }
}

#[component]
pub fn QueryDemo() -> impl IntoView {
    log::debug!("rendering <QueryDemo/>");

    on_cleanup(|| {
        log!("cleaning up <QueryDemo/>");
    });

    let (query, set_query) = create_signal::<RawQuery>("".into());

    // we use local_resource here because
    // our error type isn't serializable/deserializable
    let response = create_local_resource(query, fetch_query);

    let fallback = move |errors: RwSignal<Errors>| {
        let error_list = move || {
            errors.with(|errors| {
                errors
                    .iter()
                    .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                    .collect_view()
            })
        };

        view! {
            <div class="error">
                <h2>"Error"</h2>
                <ul>{error_list}</ul>
            </div>
        }
    };

    let response_view = move || response.and_then(|data| view! { <p>{data}</p> });

    view! {
        <h2>"Query Demo"</h2>
        <div>
            <label>
                "What is your query?"
                <input
                    style="margin: 0.5rem;"
                    type="text"
                    prop:value=move || query.get().to_string()
                    // TODO - make this an on:click event on a button instead
                    on:input=move |ev| {
                        let val = event_target_value(&ev).parse::<RawQuery>().unwrap_or("".into());
                        set_query(val);
                    }
                />
            </label>
            <ErrorBoundary fallback>
                <Transition fallback=move || {
                    view! { <div>"Loading (Suspense Fallback)..."</div> }
                }>
                <div>
                    {response_view}
                </div>
                </Transition>
            </ErrorBoundary>
        </div>
    }
}
