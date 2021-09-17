use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

extern crate wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn hello() {
    // log("Hello from Rust");
}

#[wasm_bindgen]
pub struct Client {

}

#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        log("New was hit");
        Self {

        }
    }

    pub fn update(&mut self, _time: f32, _height:f32, _width: f32) -> Result<(), JsValue> {
        // log("Update was hit");
        Ok(())
    }

    pub fn render(&self) {

    }
}