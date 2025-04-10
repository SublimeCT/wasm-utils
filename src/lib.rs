mod utils;

use wasm_bindgen::prelude::*;
use web_sys::{console, window};
use obfstr::obfstr as s;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    let message = &format!("{}{}", s!("Hello, "), name);
    alert(&message);
}

fn check() {
    let is_user = is_real_user();
    if !is_user {
        console::error_1(&s!("You are not real user").into());
    }
}

fn is_real_user() -> bool {
    let window = window().expect(s!("should have a Window"));
    let document = window.document().expect(s!("should have a Document"));

    let app_element = document.query_selector(s!("#app")).unwrap();
    if let Some(_) = app_element {
        true
    } else {
        false
    }
}

#[wasm_bindgen]
pub fn encrypt(text: &str) -> String {
    check();
    // let key = "KKKKKKKK1234567890";
    let message = format!("{}{}{}{}{}{}", s!("[wasm-utils] "), s!("encrypt() "), s!("原文: "), text, s!(" / 模拟秘钥: "), s!("KKKKKKKK1234567890"));
    console::log_1(&JsValue::from(&message));
    message
}

#[wasm_bindgen]
pub fn decrypt(text: &str) -> String {
    check();
    let message = format!("{}{}", s!("[wasm-utils] decrypt: "), text);
    console::log_1(&JsValue::from(&message));
    message
}