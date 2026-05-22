use wasm_bindgen::prelude::wasm_bindgen;

const GIT_HASH: &str = env!("GIT_HASH");
const DECODER_VERSION: &str = env!("CARGO_PKG_VERSION");

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone)]
pub struct DecoderInfo {
    pub git_hash: String,
    pub version: String,
}

#[wasm_bindgen(js_name = "getDecoderInfo")]
pub fn get_decoder_info() -> DecoderInfo {
    DecoderInfo {
        git_hash: GIT_HASH.to_string(),
        version: DECODER_VERSION.to_string(),
    }
}
