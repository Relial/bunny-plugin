use crate::mesh::{EllipseMesh, Extrudable, Mesh, MeshBuilder, PerimeterSegment, Primitive2d};

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
    pub const fn new(radius: f32) -> Self {
        Self {
            radius,
            resolution: 24,
        }
    }

    #[inline]
    pub const fn resolution(mut self, resolution: u32) -> Self {
        self.resolution = resolution;
        self
    }
}

impl Primitive2d for CircleMesh {}

impl MeshBuilder for CircleMesh {
    fn build(&self) -> Mesh {
        EllipseMesh::new(self.radius, self.radius)
            .resolution(self.resolution)
            .build()
    }
}

impl Extrudable for CircleMesh {
    fn perimeter(&self) -> Vec<PerimeterSegment> {
        vec![PerimeterSegment::Smooth {
            indices: (0..self.resolution).chain([0]).collect(),
        }]
    }
}
