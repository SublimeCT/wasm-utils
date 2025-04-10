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
    let message = &format!("Hello, {}!", name);
    alert(&message);
}

fn check() {
    let is_user = is_real_user();
    if !is_user {
        console::error_1(&"You are not real user".into());
    }
}

fn is_real_user() -> bool {
    let window = window().expect("should have a Window");
    let document = window.document().expect("should have a Document");

    let app_element = document.query_selector("#app").unwrap();
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
    let message = format!("[wasm-utils] encrypt() 原文: {} / 模拟秘钥: {}", text, s!("KKKKKKKK1234567890"));
    console::log_1(&JsValue::from(&message));
    message
}

#[wasm_bindgen]
pub fn decrypt(text: &str) -> String {
    check();
    let message = format!("[wasm-utils] decrypt: {}", text);
    console::log_1(&JsValue::from(&message));
    message
}