mod types;

use crate::types::{JsSogDataV2, JsSplat};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = "unpackSog")]
pub fn unpack_sog(buffer: &[u8]) -> Result<JsSogDataV2, JsError> {
    let sog = sog_decoder::unpack(buffer)?.into();
    Ok(sog)
}

#[wasm_bindgen(js_name = "decodeSplat")]
pub fn decode_splat(js_sog_data: &JsSogDataV2) -> Result<JsSplat, JsError> {
    let sog_data = js_sog_data.try_into()?;
    let splat = sog_decoder::decode(&sog_data)?.into();
    Ok(splat)
}
