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
    pub fn add_geometry<G>(&mut self, geometry: G) -> GeometryId
    where
        G: Into<GeometryEnum>,
    {
        let id = GeometryId::new(self.geometries.len());
        self.geometries.push(geometry.into());
        id
    }

    #[inline]
    pub fn add_shader<S>(&mut self, shader: S) -> ShaderId
    where
        S: Into<ShaderEnum>,
    {
        let id = ShaderId::new(self.shaders.len());
        self.shaders.push(shader.into());
        id
    }

    #[inline]
    pub fn add_material<M>(&mut self, material: M) -> MaterialId
    where
        M: Into<MaterialEnum>,
    {
        let id = MaterialId::new(self.materials.len());
        self.materials.push(material.into());
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
