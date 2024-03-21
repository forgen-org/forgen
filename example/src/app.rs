use wasm_bindgen::prelude::*;

use super::signal::Signal;

#[uniffi::export]
#[wasm_bindgen]
pub async fn handle(signal: String) -> String {
    let signal: Signal = serde_json::from_str(&signal).unwrap();

    let signals = signal.handle();

    serde_json::to_string(&signals).unwrap()
}
