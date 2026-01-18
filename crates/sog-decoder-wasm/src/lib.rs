mod types;

pub use crate::types::{JsSogDataV2, JsSplat};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn unpack(buffer: &[u8]) -> Result<JsSogDataV2, JsError> {
    let sog = sog_decoder::unpack(buffer)?.into();
    Ok(sog)
}

#[wasm_bindgen]
pub fn decode(js_sog_data: &JsSogDataV2) -> Result<JsSplat, JsError> {
    let sog = js_sog_data.try_into()?;
    let splat = sog_decoder::decode(&sog)?.into();
    Ok(splat)
}
