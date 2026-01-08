use crate::error::ParseError;

#[derive(Debug, Clone)]
pub struct SogDataV2 {
    pub count: u32,
    pub antialias: bool,
    pub means: Means,
    pub scales: Scales,
    pub quats: Quats,
    pub sh0: Sh0,
    pub sh_n: Option<ShN>,
}

#[derive(Debug, Clone, Default)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

impl TryFrom<Vec<f32>> for Vector3 {
    type Error = ParseError;
    fn try_from(value: Vec<f32>) -> Result<Self, Self::Error> {
        if value.len() >= 3 {
            Ok(Self::new(value[0], value[1], value[2]))
        } else {
            Err(ParseError::ParseVector(
                "Vector must have at least 3 elements".to_string(),
            ))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Codebook(pub [f32; 256]);

impl TryFrom<&[f32]> for Codebook {
    type Error = ParseError;
    fn try_from(value: &[f32]) -> Result<Self, Self::Error> {
        if value.len() >= 256 {
            let mut arr = [0.0f32; 256];
            arr.copy_from_slice(&value[..256]);
            Ok(Self(arr))
        } else {
            Err(ParseError::ParseCodebook(
                "Codebook must have at least 256 elements".to_string(),
            ))
        }
    }
}

type ImageData = Vec<u8>;

#[derive(Debug, Clone)]
pub struct Means {
    pub mins: Vector3,
    pub maxs: Vector3,
    pub means_u: ImageData,
    pub means_l: ImageData,
}

#[derive(Debug, Clone)]
pub struct Quats(pub ImageData);

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
