#[derive(thiserror::Error, serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum Error {
    #[error("An error occurred in JavaScript: {0}")]
    JsError(String),
    // #[error("Serialization Error: {0}")]
    // SerializationError(#[from] serde_wasm_bindgen::Error),
}

impl From<js_sys::wasm_bindgen::JsValue> for Error {
    fn from(error: js_sys::wasm_bindgen::JsValue) -> Self {
        let message = js_sys::Error::from(error)
            .message()
            .as_string()
            .unwrap_or("unknown JS error".to_string());
        Error::JsError(message)
    }
}
