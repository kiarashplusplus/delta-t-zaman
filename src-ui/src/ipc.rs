#![allow(dead_code)]
#![allow(unused_variables)]

use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;
use crate::models::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"])]
    async fn listen(event: &str, handler: &Closure<dyn FnMut(JsValue)>) -> JsValue;
}

pub async fn get_system_info() -> Result<SystemInfo, JsValue> {
    let res = invoke("get_system_info", JsValue::NULL).await;
    serde_wasm_bindgen::from_value(res).map_err(|e| e.into())
}

#[derive(Serialize)]
struct SetAlwaysOnTopArgs {
    always_on_top: bool,
}

pub async fn set_always_on_top(always_on_top: bool) -> Result<(), JsValue> {
    let args = serde_wasm_bindgen::to_value(&SetAlwaysOnTopArgs { always_on_top }).unwrap();
    invoke("set_always_on_top", args).await;
    Ok(())
}

#[derive(Serialize)]
struct StoreArgs<'a> {
    store_name: &'a str,
    key: &'a str,
}

#[derive(Serialize)]
struct StoreSetArgs<'a, T: Serialize> {
    store_name: &'a str,
    key: &'a str,
    value: T,
}

pub async fn get_store_value<T: for<'de> Deserialize<'de>>(store: &str, key: &str) -> Option<T> {
    if let Ok(args) = serde_wasm_bindgen::to_value(&StoreArgs { store_name: store, key }) {
        let res = invoke("load_store_value", args).await;
        if let Ok(val) = serde_wasm_bindgen::from_value::<Option<T>>(res) {
            return val;
        }
    }
    None
}

pub async fn set_store_value<T: Serialize>(store: &str, key: &str, value: T) {
    if let Ok(args) = serde_wasm_bindgen::to_value(&StoreSetArgs { store_name: store, key, value }) {
        invoke("save_store_value", args).await;
    }
}
