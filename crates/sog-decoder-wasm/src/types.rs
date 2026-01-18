use sog_decoder::types::{Codebook, Means, Quats, Scales, Sh0, ShN, SogDataV2, Splat, Vector3};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(js_name = "Splat", getter_with_clone)]
#[derive(Debug, Clone)]
pub struct JsSplat {
    pub count: usize,
    pub antialias: bool,
    pub sh_degree: usize,
    pub position: Vec<f32>,
    pub rotation: Vec<f32>,
    pub scale: Vec<f32>,
    pub color: Vec<f32>,
    pub sh: Option<Vec<f32>>,
}

impl From<Splat> for JsSplat {
    fn from(splat: Splat) -> Self {
        Self {
            count: splat.count,
            antialias: splat.antialias,
            sh_degree: splat.sh_degree,
            position: splat.position,
            rotation: splat.rotation,
            scale: splat.scale,
            color: splat.color,
            sh: splat.sh,
        }
    }
}

#[wasm_bindgen(js_name = "Vector3", getter_with_clone)]
#[derive(Debug, Clone, Default)]
pub struct JsVector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<Vector3> for JsVector3 {
    fn from(vector3: Vector3) -> Self {
        Self {
            x: vector3.x,
            y: vector3.y,
            z: vector3.z,
        }
    }
}

impl From<JsVector3> for Vector3 {
    fn from(js_vector3: JsVector3) -> Self {
        Self {
            x: js_vector3.x,
            y: js_vector3.y,
            z: js_vector3.z,
        }
    }
}

#[wasm_bindgen(js_name = "Codebook", getter_with_clone)]
#[derive(Debug, Clone)]
pub struct JsCodebook(pub Vec<f32>);

impl From<Codebook> for JsCodebook {
    fn from(codebook: Codebook) -> Self {
        Self(codebook.0.to_vec())
    }
}

impl TryFrom<JsCodebook> for Codebook {
    type Error = &'static str;

    fn try_from(js_codebook: JsCodebook) -> Result<Self, Self::Error> {
        let vec = js_codebook.0;
        if vec.len() == 256 {
            let mut arr = [0.0f32; 256];
            arr.copy_from_slice(&vec);
            Ok(Codebook(arr))
        } else {
            Err("Codebook must have 256 elements")
        }
    }
}

#[wasm_bindgen(js_name = "Means", getter_with_clone)]
#[derive(Debug, Clone)]
pub struct JsMeans {
    pub mins: JsVector3,
    pub maxs: JsVector3,
    #[wasm_bindgen(js_name = "meansU")]
    pub means_u: Vec<u8>,
    #[wasm_bindgen(js_name = "meansL")]
    pub means_l: Vec<u8>,
}

impl From<Means> for JsMeans {
    fn from(means: Means) -> Self {
        Self {
            mins: means.mins.into(),
            maxs: means.maxs.into(),
            means_u: means.means_u,
            means_l: means.means_l,
        }
    }
}

impl From<JsMeans> for Means {
    fn from(js_means: JsMeans) -> Self {
        Self {
            mins: js_means.mins.into(),
            maxs: js_means.maxs.into(),
            means_u: js_means.means_u,
            means_l: js_means.means_l,
        }
    }
}

#[wasm_bindgen(js_name = "Quats", getter_with_clone)]
#[derive(Debug, Clone)]
pub struct JsQuats(pub Vec<u8>);

impl From<Quats> for JsQuats {
    fn from(quats: Quats) -> Self {
        Self(quats.0)
    }
}

impl From<JsQuats> for Quats {
    fn from(js_quats: JsQuats) -> Self {
        Self(js_quats.0)
    }
}

#[wasm_bindgen(js_name = "Scales", getter_with_clone)]
#[derive(Debug, Clone)]
pub struct JsScales {
    pub codebook: JsCodebook,
    pub scales: Vec<u8>,
}

impl From<Scales> for JsScales {
    fn from(scales: Scales) -> Self {
        Self {
            codebook: scales.codebook.into(),
            scales: scales.scales,
        }
    }
}

impl From<JsScales> for Scales {
    fn from(js_scales: JsScales) -> Self {
        Self {
            codebook: js_scales.codebook.try_into().expect("Invalid codebook"),
            scales: js_scales.scales,
        }
    }
}

#[wasm_bindgen(js_name = "Sh0", getter_with_clone)]
#[derive(Debug, Clone)]
pub struct JsSh0 {
    pub codebook: JsCodebook,
    pub sh0: Vec<u8>,
}

impl From<Sh0> for JsSh0 {
    fn from(sh0: Sh0) -> Self {
        Self {
            codebook: sh0.codebook.into(),
            sh0: sh0.sh0,
        }
    }
}

impl From<JsSh0> for Sh0 {
    fn from(js_sh0: JsSh0) -> Self {
        Self {
            codebook: js_sh0.codebook.try_into().expect("Invalid codebook"),
            sh0: js_sh0.sh0,
        }
    }
}

#[wasm_bindgen(js_name = "ShN", getter_with_clone)]
#[derive(Debug, Clone)]
pub struct JsShN {
    pub count: i32,
    pub bands: i32,
    pub codebook: JsCodebook,
    pub labels: Vec<u8>,
    pub centroids: Vec<u8>,
}

impl From<ShN> for JsShN {
    fn from(sh_n: ShN) -> Self {
        Self {
            count: sh_n.count,
            bands: sh_n.bands,
            codebook: sh_n.codebook.into(),
            labels: sh_n.labels,
            centroids: sh_n.centroids,
        }
    }
}

impl From<JsShN> for ShN {
    fn from(js_sh_n: JsShN) -> Self {
        Self {
            count: js_sh_n.count,
            bands: js_sh_n.bands,
            codebook: js_sh_n.codebook.try_into().expect("Invalid codebook"),
            labels: js_sh_n.labels,
            centroids: js_sh_n.centroids,
        }
    }
}

#[wasm_bindgen(js_name = "SogDataV2", getter_with_clone)]
#[derive(Debug, Clone)]
pub struct JsSogDataV2 {
    pub count: u32,
    pub antialias: bool,
    pub means: JsMeans,
    pub scales: JsScales,
    pub quats: JsQuats,
    pub sh0: JsSh0,
    #[wasm_bindgen(js_name = "shN")]
    pub sh_n: Option<JsShN>,
}

impl From<SogDataV2> for JsSogDataV2 {
    fn from(sog_data: SogDataV2) -> Self {
        Self {
            count: sog_data.count,
            antialias: sog_data.antialias,
            means: sog_data.means.into(),
            scales: sog_data.scales.into(),
            quats: sog_data.quats.into(),
            sh0: sog_data.sh0.into(),
            sh_n: sog_data.sh_n.map(|sh_n| sh_n.into()),
        }
    }
}

impl From<&JsSogDataV2> for SogDataV2 {
    fn from(js_sog_data: &JsSogDataV2) -> Self {
        Self {
            count: js_sog_data.count,
            antialias: js_sog_data.antialias,
            means: js_sog_data.means.clone().into(),
            scales: js_sog_data.scales.clone().into(),
            quats: js_sog_data.quats.clone().into(),
            sh0: js_sog_data.sh0.clone().into(),
            sh_n: js_sog_data.sh_n.clone().map(|sh_n| sh_n.into()),
        }
    }
}
