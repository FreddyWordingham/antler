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
    pub fn new() -> Self {
        Self {
            geometries: Vec::new(),
            shaders: Vec::new(),
            materials: Vec::new(),
        }
    }

    #[inline]
    pub fn add_geometry(&mut self, geometry: GeometryEnum) -> GeometryId {
        let id = GeometryId::new(self.geometries.len());
        self.geometries.push(geometry);
        id
    }

    #[inline]
    pub fn add_shader(&mut self, shader: ShaderEnum) -> ShaderId {
        let id = ShaderId::new(self.shaders.len());
        self.shaders.push(shader);
        id
    }

    #[inline]
    pub fn add_material(&mut self, material: MaterialEnum) -> MaterialId {
        let id = MaterialId::new(self.materials.len());
        self.materials.push(material);
        id
    }

    #[inline]
    pub fn get_geometry(&self, id: GeometryId) -> &GeometryEnum {
        &self.geometries[id.index()]
    }

    #[inline]
    pub fn get_shader(&self, id: ShaderId) -> &ShaderEnum {
        &self.shaders[id.index()]
    }

    #[inline]
    pub fn get_material(&self, id: MaterialId) -> &MaterialEnum {
        &self.materials[id.index()]
    }
}
