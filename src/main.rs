use leptos::*;
use leptos_tutorial::*;
use wasm_bindgen::JsValue;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    set_global_variable();
    // TODO - figure out how to set as env var
    mount_to_body(|| view! { <App debug=true / > });
}

fn set_global_variable() {
    // Get the global window object
    let window = window();

    // Set a global variable `myWasmModuleReady` to true
    let _ = js_sys::Reflect::set(
        &window,
        &JsValue::from_str("myWasmIsReady"),
        &JsValue::from_bool(true),
    );
}
