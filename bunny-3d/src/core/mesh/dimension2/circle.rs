use crate::mesh::{EllipseBuilder, Extrudable, Mesh, MeshBuilder, PerimeterSegment, Primitive2d};

#[derive(Clone, Copy, Debug)]
pub struct CircleBuilder {
    pub radius: f32,
    pub resolution: u32,
}

impl Default for CircleBuilder {
    fn default() -> Self {
        Self {
            radius: 0.5,
            resolution: 24,
        }
    }
}

impl CircleBuilder {
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

impl Primitive2d for CircleBuilder {}

impl MeshBuilder for CircleBuilder {
    fn build(&self) -> Mesh {
        EllipseBuilder::new(self.radius, self.radius)
            .resolution(self.resolution)
            .build()
    }
}

impl Extrudable for CircleBuilder {
    fn perimeter(&self) -> Vec<PerimeterSegment> {
        vec![PerimeterSegment::Smooth {
            indices: (0..self.resolution).chain([0]).collect(),
        }]
    }
}
