#[cfg(feature = "wasm")]
mod wasm {
    use crate::utils::errors::KubeMergeError;
    use crate::utils::merger::{merge_kubeconfig_contents, KubeconfigContent};
    use wasm_bindgen::prelude::wasm_bindgen;
    use wasm_bindgen::JsValue;

    #[wasm_bindgen]
    pub fn merge_configs(configs: Vec<JsValue>) -> Result<String, JsValue> {
        let contents: Vec<KubeconfigContent> = configs
            .into_iter()
            .map(|js_val| {
                js_val
                    .as_string()
                    .ok_or_else(|| {
                        KubeMergeError::ParseError("Invalid input: expected string".to_string())
                    })
                    .map(|content| KubeconfigContent { content })
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        merge_kubeconfig_contents(&contents).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(start)]
    pub fn main_js() -> Result<(), JsValue> {
        #[cfg(debug_assertions)]
        console_error_panic_hook::set_once();
        Ok(())
    }
}
