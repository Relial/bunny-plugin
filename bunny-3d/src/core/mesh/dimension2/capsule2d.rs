use crate::{
    draw_list::PrimitiveTopology, mesh::{Mesh, MeshBuilder, Primitive2d},
};

#[derive(Clone, Copy, Debug)]
pub struct Capsule2dMesh {
    pub radius: f32,
    pub half_length: f32,
    pub resolution: u32,
}

impl Default for Capsule2dMesh {
    fn default() -> Self {
        Self {
            radius: 0.5,
            half_length: 0.5,
            resolution: 16,
        }
    }
}

impl Capsule2dMesh {
    #[inline]
    pub const fn new(radius: f32, length: f32) -> Self {
        Self {
            radius,
            half_length: length / 2.0,
            resolution: 16,
        }
    }

    #[inline]
    pub const fn resolution(mut self, resolution: u32) -> Self {
        self.resolution = resolution;
        self
    }
}

impl Primitive2d for Capsule2dMesh {}

impl MeshBuilder for Capsule2dMesh {
    fn build(&self) -> Mesh {
        // From https://docs.rs/bevy_mesh/0.19.1/src/bevy_mesh/primitives/dim2.rs.html#1163

        let Capsule2dMesh {
            radius,
            half_length,
            resolution,
        } = *self;

        // The resolution is the number of vertices for one semicircle
        let vertex_count = 2 * resolution;

        // Six extra indices for the two triangles between the semicircles
        let mut indices = Vec::with_capacity((resolution as usize - 2) * 2 * 3 + 6);
        let mut positions = Vec::with_capacity(vertex_count as usize);
        let mut uvs = Vec::with_capacity(vertex_count as usize);

        let step = core::f32::consts::TAU / vertex_count as f32;

        // If the vertex count is even, offset starting angle of top semicircle by half a step
        // to position the vertices evenly.
        let start_angle = if vertex_count.is_multiple_of(2) {
            step / 2.0
        } else {
            0.0
        };

        // How much the hemicircle radius is of the total half-height of the capsule.
        // This is used to prevent the UVs from stretching between the semicircles.
        let radius_frac = radius / (half_length + radius);

        // Create top semicircle
        for i in 0..resolution {
            // Compute vertex position at angle theta
            let theta = start_angle + i as f32 * step;
            let (sin, cos) = f32::sin_cos(theta);
            let (x, y) = (cos * radius, sin * radius + half_length);
            positions.push([x, y, 0.0]);
            uvs.push([0.5 * (cos + 1.0), radius_frac * (1.0 - 0.5 * (sin + 1.0))]);
        }

        // Add top semicircle indices
        for i in 1..resolution - 1 {
            indices.extend_from_slice(&[0, i, i + 1]);
        }

        // Add indices for top left triangle of the part between the semicircles
        indices.extend_from_slice(&[0, resolution - 1, resolution]);

        // Create bottom semicircle
        for i in resolution..vertex_count {
            // Compute vertex position at angle theta
            let theta = start_angle + i as f32 * step;
            let (sin, cos) = f32::sin_cos(theta);
            let (x, y) = (cos * radius, sin * radius - half_length);
            positions.push([x, y, 0.0]);
            uvs.push([0.5 * (cos + 1.0), 1.0 - radius_frac * 0.5 * (sin + 1.0)]);
        }

        // Add bottom semicircle indices
        for i in 1..resolution - 1 {
            indices.extend_from_slice(&[resolution, resolution + i, resolution + i + 1]);
        }

        // Add indices for bottom right triangle of the part between the semicircles
        indices.extend_from_slice(&[resolution, vertex_count - 1, 0]);

        Mesh::new(positions, uvs, indices, PrimitiveTopology::TriangleList)
    }
}
