use leptos::*;
use leptos_tutorial::*;
use wasm_bindgen::JsValue;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    // set global variable to signal the wasm is starting
    let window = window();
    let _ = js_sys::Reflect::set(
        &window,
        &JsValue::from_str("myWasmIsReady"),
        &JsValue::from_bool(true),
    );

    // TODO - figure out how to set 'demo' as env var
    mount_to_body(|| view! { <App demo=true / > });
}
