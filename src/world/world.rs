use crate::{
    geometry::Geometry,
    id::{GeometryId, MaterialId, ShaderId},
    material::Material,
    shader::Shader,
};

pub struct World {
    geometries: Vec<Box<dyn Geometry>>,
    shaders: Vec<Box<dyn Shader>>,
    materials: Vec<Box<dyn Material>>,
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
