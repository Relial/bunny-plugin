use crate::core::mesh::{Mesh, ellipse::EllipseMesh};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CircleMesh {
    pub radius: f32,
    pub resolution: u32,
}

impl Default for CircleMesh {
    fn default() -> Self {
        Self {
            radius: 0.5,
            resolution: 32,
        }
    }
}

impl CircleMesh {
    #[inline]
    pub const fn new(radius: f32, resolution: u32) -> Self {
        Self { radius, resolution }
    }
}

impl From<CircleMesh> for Mesh {
    fn from(value: CircleMesh) -> Self {
        EllipseMesh::new(value.radius, value.radius).into()
    }
}
