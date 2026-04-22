use crate::{
    geometry::{Geometry, GeometryEnum},
    id::{GeometryId, MaterialId, ShaderId},
    material::{Material, MaterialEnum},
    shader::{Shader, ShaderEnum},
};

pub struct World {
    geometries: Vec<GeometryEnum>,
    shaders: Vec<ShaderEnum>,
    materials: Vec<MaterialEnum>,
}

impl World {
    pub fn get_geometry(&self, id: GeometryId) -> &dyn Geometry {
        todo!()
    }

    pub fn get_shader(&self, id: ShaderId) -> &dyn Shader {
        todo!()
    }

    pub fn get_material(&self, id: MaterialId) -> &dyn Material {
        todo!()
    }
}
