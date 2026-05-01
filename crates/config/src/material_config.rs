use antler_material::{Ggx, Lambertian, Material, Mirror, Opaque, Reflective, Refractive, Wireframe};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum MaterialConfig {
    Ggx { roughness: f32, reflectance: f32 },
    Lambertian { albedo: f32 },
    Mirror,
    Opaque,
    Reflective { reflectance: f32 },
    Refractive { refractive_index: f32 },
    Wireframe { surface_alpha: f32, line_width: f32 },
}

impl MaterialConfig {
    pub fn build(self) -> Material {
        match self {
            Self::Ggx { roughness, reflectance } => Ggx::new(roughness, reflectance).into(),
            Self::Lambertian { albedo } => Lambertian::new(albedo).into(),
            Self::Mirror => Mirror::new().into(),
            Self::Opaque => Opaque::new().into(),
            Self::Reflective { reflectance } => Reflective::new(reflectance).into(),
            Self::Refractive { refractive_index } => Refractive::new(refractive_index).into(),
            Self::Wireframe {
                surface_alpha,
                line_width,
            } => Wireframe::new(surface_alpha, line_width).into(),
        }
    }
}
