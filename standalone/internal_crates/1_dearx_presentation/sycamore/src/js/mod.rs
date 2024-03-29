use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(message: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}
