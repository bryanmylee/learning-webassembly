use wasm_bindgen::prelude::*;

// expose an external function from the JavaScript environment.
#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

// expose an API method to the JavaScript environment.
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
