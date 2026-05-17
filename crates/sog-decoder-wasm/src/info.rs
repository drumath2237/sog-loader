use wasm_bindgen::prelude::wasm_bindgen;

const GIT_HASH: &str = env!("GIT_HASH");

#[wasm_bindgen(js_name = "getGitHash")]
pub fn get_git_hash() -> String {
    GIT_HASH.to_string()
}

const DECODER_VERSION: &str = env!("CARGO_PKG_VERSION");

#[wasm_bindgen(js_name = "getDecoderVersion")]
pub fn get_decoder_version() -> String {
    DECODER_VERSION.to_string()
}
