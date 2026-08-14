use core::ops::RangeInclusive;

use glam::Vec3;

use crate::{
    draw_list::PrimitiveTopology,
    mesh::{Mesh, MeshBuilder},
};

#[derive(Clone, Debug)]
pub struct TorusBuilder {
    pub minor_radius: f32,
    pub major_radius: f32,
    pub minor_resolution: usize,
    pub major_resolution: usize,
    pub angle_range: RangeInclusive<f32>,
}

impl Default for TorusBuilder {
    fn default() -> Self {
        Self {
            minor_radius: 0.25,
            major_radius: 0.75,
            minor_resolution: 18,
            major_resolution: 24,
            angle_range: (0.0..=2.0 * core::f32::consts::PI),
        }
    }
}

impl TorusBuilder {
    #[inline]
    pub const fn new(inner_radius: f32, outer_radius: f32) -> Self {
        let minor_radius = (outer_radius - inner_radius) / 2.0;
        let major_radius = outer_radius - minor_radius;
        Self {
            minor_radius,
            major_radius,
            minor_resolution: 18,
            major_resolution: 24,
            angle_range: (0.0..=2.0 * core::f32::consts::PI),
        }
    }

    #[inline]
    pub const fn minor_resolution(mut self, resolution: usize) -> Self {
        self.minor_resolution = resolution;
        self
    }

    #[inline]
    pub const fn major_resolution(mut self, resolution: usize) -> Self {
        self.major_resolution = resolution;
        self
    }

    #[inline]
    pub const fn angle_range(mut self, range: RangeInclusive<f32>) -> Self {
        self.angle_range = range;
        self
    }
}

impl MeshBuilder for TorusBuilder {
    fn build(&self) -> Mesh {
        // From https://docs.rs/bevy_mesh/0.19.1/src/bevy_mesh/primitives/dim3/torus.rs.html#80

        // code adapted from http://apparat-engine.blogspot.com/2013/04/procedural-meshes-torus.html

        let n_vertices = (self.major_resolution + 1) * (self.minor_resolution + 1);
        let mut positions: Vec<[f32; 3]> = Vec::with_capacity(n_vertices);
        let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(n_vertices);

        let start_angle = self.angle_range.start();
        let end_angle = self.angle_range.end();

        let segment_stride = (end_angle - start_angle) / self.major_resolution as f32;
        let side_stride = 2.0 * core::f32::consts::PI / self.minor_resolution as f32;

        for segment in 0..=self.major_resolution {
            let theta = start_angle + segment_stride * segment as f32;

            for side in 0..=self.minor_resolution {
                let phi = side_stride * side as f32;
                let (sin_theta, cos_theta) = f32::sin_cos(theta);
                let (sin_phi, cos_phi) = f32::sin_cos(phi);
                let radius = self.major_radius + self.minor_radius * cos_phi;

                let position = Vec3::new(
                    cos_theta * radius,
                    self.minor_radius * sin_phi,
                    sin_theta * radius,
                );

                positions.push(position.into());
                uvs.push([
                    segment as f32 / self.major_resolution as f32,
                    side as f32 / self.minor_resolution as f32,
                ]);
            }
        }

        let n_faces = (self.major_resolution) * (self.minor_resolution);
        let n_triangles = n_faces * 2;
        let n_indices = n_triangles * 3;

        let mut indices: Vec<u32> = Vec::with_capacity(n_indices);

        let n_vertices_per_row = self.minor_resolution + 1;
        for segment in 0..self.major_resolution {
            for side in 0..self.minor_resolution {
                let lt = side + segment * n_vertices_per_row;
                let rt = (side + 1) + segment * n_vertices_per_row;

                let lb = side + (segment + 1) * n_vertices_per_row;
                let rb = (side + 1) + (segment + 1) * n_vertices_per_row;

                indices.push(lt as u32);
                indices.push(rt as u32);
                indices.push(lb as u32);

                indices.push(rt as u32);
                indices.push(rb as u32);
                indices.push(lb as u32);
            }
        }

        Mesh::new(positions, uvs, indices, PrimitiveTopology::TriangleList)
    }
}
