use crate::{
    geometry::GeometryEnum,
    id::{GeometryId, MaterialId, ShaderId},
    material::MaterialEnum,
    shader::ShaderEnum,
};

pub struct World {
    geometries: Vec<GeometryEnum>,
    shaders: Vec<ShaderEnum>,
    materials: Vec<MaterialEnum>,
}

impl World {
    pub fn get_geometry(&self, id: GeometryId) -> &GeometryEnum {
        &self.geometries[*id]
    }

    pub fn get_shader(&self, id: ShaderId) -> &ShaderEnum {
        &self.shaders[*id]
    }

    pub fn get_material(&self, id: MaterialId) -> &MaterialEnum {
        &self.materials[*id]
    }
}
