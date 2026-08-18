use glam::Vec2;

use crate::{
    draw_list::PrimitiveTopology,
    mesh::{Extrudable, Mesh, MeshBuilder, PerimeterSegment, Primitive2d},
};

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EllipseBuilder {
    pub half_size: Vec2,
    pub resolution: u32,
}

impl Default for EllipseBuilder {
    fn default() -> Self {
        Self {
            half_size: Vec2::new(1.0, 0.5),
            resolution: 24,
        }
    }
}

impl EllipseBuilder {
    #[inline]
    pub const fn new(half_width: f32, half_height: f32) -> Self {
        Self {
            half_size: Vec2::new(half_width, half_height),
            resolution: 24,
        }
    }

    #[inline]
    pub const fn from_size(size: Vec2) -> Self {
        Self {
            half_size: Vec2::new(size.x / 2.0, size.y / 2.0),
            resolution: 24,
        }
    }

    #[inline]
    pub const fn resolution(mut self, resolution: u32) -> Self {
        self.resolution = resolution;
        self
    }
}

impl Primitive2d for EllipseBuilder {}

impl MeshBuilder for EllipseBuilder {
    fn build(&self) -> Mesh {
        // From https://docs.rs/bevy_mesh/0.19.0/src/bevy_mesh/primitives/dim2.rs.html#596
        let EllipseBuilder {
            half_size,
            resolution,
        } = *self;
        let resolution = resolution as usize;
        let mut indices = Vec::with_capacity((resolution - 2) * 3);
        let mut positions = Vec::with_capacity(resolution);
        let mut uvs = Vec::with_capacity(resolution);

        // Add pi/2 so that there is a vertex at the top (sin is 1.0 and cos is 0.0)
        let start_angle = core::f32::consts::FRAC_PI_2;
        let step = core::f32::consts::TAU / resolution as f32;

        for i in 0..resolution {
            // Compute vertex position at angle theta
            let theta = start_angle + i as f32 * step;
            let (sin, cos) = f32::sin_cos(theta);
            let x = cos * half_size.x;
            let y = sin * half_size.y;

            positions.push([x, y, 0.0]);
            uvs.push([0.5 * (cos + 1.0), 1.0 - 0.5 * (sin + 1.0)]);
        }

        for i in 1..(self.resolution - 1) {
            indices.extend_from_slice(&[0, i, i + 1]);
        }

        Mesh::new(positions, uvs, vec![], indices, PrimitiveTopology::TriangleList)
    }
}

impl Extrudable for EllipseBuilder {
    fn perimeter(&self) -> Vec<PerimeterSegment> {
        vec![PerimeterSegment::Smooth {
            indices: (0..self.resolution).chain([0]).collect(),
        }]
    }
}
