//! WebDriver Implementation
use elvis_shared::Driver;
use wasm_bindgen::prelude::*;

/// Elvis web driver
pub struct WebDriver;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    fn alert(s: &str);
}

impl Driver for WebDriver {
    fn alert(msg: &str) {
        alert(&msg);
    }
}