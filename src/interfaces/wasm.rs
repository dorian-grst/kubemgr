use wasm_bindgen::prelude::*;
use crate::utils::merger::{merge_kubeconfig_contents, KubeconfigContent};

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn merge_configs(configs: Vec<JsValue>) -> Result<String, JsValue> {
    let contents: Vec<KubeconfigContent> = configs
        .into_iter()
        .map(|js_val| {
            js_val
                .as_string()
                .ok_or_else(|| JsValue::from_str("Invalid input: expected string"))
                .map(|content| KubeconfigContent { content })
        })
        .collect::<Result<Vec<_>, _>>()?;

    merge_kubeconfig_contents(&contents)
        .map_err(|e| JsValue::from_str(&format!("Failed to merge configs: {}", e)))
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    Ok(())
}