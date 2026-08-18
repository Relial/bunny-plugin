use crate::{
    draw_list::PrimitiveTopology,
    mesh::{Mesh, MeshBuilder},
};

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ConeBuilder {
    pub radius: f32,
    pub height: f32,
    pub resolution: u32,
    pub anchor: ConeAnchor,
}

impl Default for ConeBuilder {
    fn default() -> Self {
        Self {
            radius: 0.5,
            height: 1.0,
            resolution: 24,
            anchor: Default::default(),
        }
    }
}

impl ConeBuilder {
    #[inline]
    pub const fn new(radius: f32, height: f32) -> Self {
        Self {
            radius,
            height,
            resolution: 24,
            anchor: ConeAnchor::MidPoint,
        }
    }

    #[inline]
    pub const fn resolution(mut self, resolution: u32) -> Self {
        self.resolution = resolution;
        self
    }

    #[inline]
    pub const fn anchor(mut self, anchor: ConeAnchor) -> Self {
        self.anchor = anchor;
        self
    }
}

impl MeshBuilder for ConeBuilder {
    fn build(&self) -> Mesh {
        // From https://docs.rs/bevy_mesh/0.19.1/src/bevy_mesh/primitives/dim3/cone.rs.html#72

        let ConeBuilder {
            radius,
            height,
            resolution,
            anchor,
        } = *self;

        let half_height = height / 2.0;

        // `resolution` vertices for the base, `resolution` vertices for the bottom of the lateral surface,
        // and one vertex for the tip.
        let num_vertices = resolution as usize * 2 + 1;
        let num_indices = resolution as usize * 6 - 6;

        let mut positions = Vec::with_capacity(num_vertices);
        let mut uvs = Vec::with_capacity(num_vertices);
        let mut indices = Vec::with_capacity(num_indices);

        // Tip
        positions.push([0.0, half_height, 0.0]);

        // The UVs of the cone are in polar coordinates, so it's like projecting a circle texture from above.
        // The center of the texture is at the center of the lateral surface, at the tip of the cone.
        uvs.push([0.5, 0.5]);

        // Now we build the lateral surface, the side of the cone.

        // How much the angle changes at each step
        let step_theta = core::f32::consts::TAU / resolution as f32;

        // Add vertices for the bottom of the lateral surface.
        for segment in 0..resolution {
            let theta = segment as f32 * step_theta;
            let (sin, cos) = f32::sin_cos(theta);

            positions.push([radius * cos, -half_height, radius * sin]);
            uvs.push([0.5 + cos * 0.5, 0.5 + sin * 0.5]);
        }

        // Add indices for the lateral surface. Each triangle is formed by the tip
        // and two vertices at the base.
        for j in 1..resolution {
            indices.extend_from_slice(&[0, j + 1, j]);
        }

        // Close the surface with a triangle between the tip, first base vertex, and last base vertex.
        indices.extend_from_slice(&[0, 1, resolution]);

        // Now we build the actual base of the cone.

        let index_offset = positions.len() as u32;

        // Add base vertices.
        for i in 0..resolution {
            let theta = i as f32 * step_theta;
            let (sin, cos) = f32::sin_cos(theta);

            positions.push([cos * radius, -half_height, sin * radius]);
            uvs.push([0.5 * (cos + 1.0), 1.0 - 0.5 * (sin + 1.0)]);
        }

        // Add base indices.
        for i in 1..(resolution - 1) {
            indices.extend_from_slice(&[index_offset, index_offset + i, index_offset + i + 1]);
        }

        // Offset the vertex positions Y axis to match the anchor
        match anchor {
            ConeAnchor::Tip => positions.iter_mut().for_each(|p| p[1] -= half_height),
            ConeAnchor::Base => positions.iter_mut().for_each(|p| p[1] += half_height),
            ConeAnchor::MidPoint => (),
        };

        Mesh::new(positions, uvs, vec![], indices, PrimitiveTopology::TriangleList)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ConeAnchor {
    #[default]
    MidPoint,
    Tip,
    Base,
}
