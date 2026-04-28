use antler_geometry::Geometry;
use antler_id::{GeometryId, MaterialId, ShaderId};
use antler_material::Material;
use antler_shader::Shader;

pub struct Resources {
    geometries: Vec<Geometry>,
    materials: Vec<Material>,
    shaders: Vec<Shader>,
}

impl Resources {
    pub fn new() -> Self {
        Self {
            geometries: Vec::new(),
            materials: Vec::new(),
            shaders: Vec::new(),
        }
    }

    #[inline]
    pub fn add_geometry<G>(&mut self, geometry: G) -> GeometryId
    where
        G: Into<Geometry>,
    {
        let id = GeometryId::new(self.geometries.len());
        self.geometries.push(geometry.into());
        id
    }

    #[inline]
    pub fn add_material<M>(&mut self, material: M) -> MaterialId
    where
        M: Into<Material>,
    {
        let id = MaterialId::new(self.materials.len());
        self.materials.push(material.into());
        id
    }

    #[inline]
    pub fn add_shader<S>(&mut self, shader: S) -> ShaderId
    where
        S: Into<Shader>,
    {
        let id = ShaderId::new(self.shaders.len());
        self.shaders.push(shader.into());
        id
    }

    #[inline]
    pub fn get_geometry(&self, id: GeometryId) -> &Geometry {
        &self.geometries[id.index()]
    }

    #[inline]
    pub fn get_material(&self, id: MaterialId) -> &Material {
        &self.materials[id.index()]
    }

    #[inline]
    pub fn get_shader(&self, id: ShaderId) -> &Shader {
        &self.shaders[id.index()]
    }
}
