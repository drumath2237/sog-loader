#[derive(Debug, Clone)]
pub struct SogDataV2 {
    pub count: u32,
    pub antialias: bool,
    means: Means,
    scales: Scales,
    quats: Quats,
    sh0: Sh0,
    sh_n: Option<ShN>,
}

#[derive(Debug, Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

type Codebook = [f32; 256];
type ImageData = Vec<u8>;

#[derive(Debug, Clone)]
pub struct Means {
    pub mins: Vector3,
    pub maxs: Vector3,
    pub means_u: ImageData,
    pub means_l: ImageData,
}

#[derive(Debug, Clone)]
pub struct Quats(ImageData);

#[derive(Debug, Clone)]
pub struct Scales {
    pub codebook: Codebook,
    pub scales: ImageData,
}

#[derive(Debug, Clone)]
pub struct Sh0 {
    pub codebook: Codebook,
    pub sh0: ImageData,
}

#[derive(Debug, Clone)]
pub struct ShN {
    pub count: i32,
    pub bands: i32,
    pub codebook: Codebook,
    pub labels: ImageData,
    pub centroids: ImageData,
}
