use wasm_bindgen::prelude::*;
use web_sys::{window, Document};

#[wasm_bindgen]
pub fn detect_technologies() -> JsValue {
    let document = window().unwrap().document().unwrap();
    let mut detected = vec![];

    // Example: Detect jQuery by checking if "$" function exists
    if js_sys::eval("typeof $ === 'function' && $.fn && $.fn.jquery").unwrap().as_bool().unwrap() {
        detected.push("jQuery");
    }

    // Example: Detect Bootstrap by checking for specific classes in the DOM
    if document.query_selector(".container").is_ok() {
        detected.push("Bootstrap");
    }

    JsValue::from_serde(&detected).unwrap()
}
