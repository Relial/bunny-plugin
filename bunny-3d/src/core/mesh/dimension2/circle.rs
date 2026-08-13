use crate::mesh::{EllipseMesh, Mesh, MeshBuilder};

#[derive(Clone, Copy, Debug)]
pub struct CircleMesh {
    pub radius: f32,
    pub resolution: u32,
}

impl Default for CircleMesh {
    fn default() -> Self {
        Self {
            radius: 0.5,
            resolution: 24,
        }
    }
}

impl CircleMesh {
    #[inline]
    pub const fn new(radius: f32, resolution: u32) -> Self {
        Self { radius, resolution }
    }
}

impl MeshBuilder for CircleMesh {
    fn build(&self) -> Mesh {
        EllipseMesh::new(self.radius, self.radius).into()
    }
}

impl From<CircleMesh> for Mesh {
    fn from(value: CircleMesh) -> Self {
        value.build()
    }
}
