use crate::{
    draw_list::PrimitiveTopology,
    mesh::{Extrudable, Mesh, MeshBuilder, PerimeterSegment, Primitive2d},
};

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AnnulusBuilder {
    pub inner_radius: f32,
    pub outer_radius: f32,
    pub resolution: u32,
}

impl Default for AnnulusBuilder {
    fn default() -> Self {
        Self {
            inner_radius: 0.5,
            outer_radius: 1.0,
            resolution: 24,
        }
    }
}

impl AnnulusBuilder {
    #[inline]
    pub const fn new(inner_radius: f32, outer_radius: f32) -> Self {
        Self {
            inner_radius,
            outer_radius,
            resolution: 24,
        }
    }

    #[inline]
    pub const fn resolution(mut self, resolution: u32) -> Self {
        self.resolution = resolution;
        self
    }
}

impl Primitive2d for AnnulusBuilder {}

impl MeshBuilder for AnnulusBuilder {
    fn build(&self) -> Mesh {
        // From https://docs.rs/bevy_mesh/0.19.1/src/bevy_mesh/primitives/dim2.rs.html#784
        let AnnulusBuilder {
            inner_radius,
            outer_radius,
            resolution,
        } = *self;

        let num_vertices = (resolution as usize + 1) * 2;
        let mut indices = Vec::with_capacity(resolution as usize * 6);
        let mut positions = Vec::with_capacity(num_vertices);
        let mut uvs = Vec::with_capacity(num_vertices);
        // We have one more set of vertices than might be naïvely expected;
        // the vertices at `start_angle` are duplicated for the purposes of UV
        // mapping. Here, each iteration places a pair of vertices at a fixed
        // angle from the center of the annulus.
        let start_angle = core::f32::consts::FRAC_PI_2;
        let step = core::f32::consts::TAU / resolution as f32;
        for i in 0..=resolution {
            let theta = start_angle + (i % resolution) as f32 * step;
            let (sin, cos) = f32::sin_cos(theta);
            let inner_pos = [cos * inner_radius, sin * inner_radius, 0.];
            let outer_pos = [cos * outer_radius, sin * outer_radius, 0.];
            positions.push(inner_pos);
            positions.push(outer_pos);
            // The first UV direction is radial and the second is angular;
            // i.e., a single UV rectangle is stretched around the annulus, with
            // its top and bottom meeting as the circle closes. Lines of constant
            // U map to circles, and lines of constant V map to radial line segments.
            let inner_uv = [0., i as f32 / resolution as f32];
            let outer_uv = [1., i as f32 / resolution as f32];
            uvs.push(inner_uv);
            uvs.push(outer_uv);
        }
        // Adjacent pairs of vertices form two triangles with each other; here,
        // we are just making sure that they both have the right orientation,
        // which is the CCW order of
        // `inner_vertex` -> `outer_vertex` -> `next_outer` -> `next_inner`
        for i in 0..resolution {
            let inner_vertex = 2 * i;
            let outer_vertex = 2 * i + 1;
            let next_inner = inner_vertex + 2;
            let next_outer = outer_vertex + 2;
            indices.extend_from_slice(&[inner_vertex, outer_vertex, next_outer]);
            indices.extend_from_slice(&[next_outer, next_inner, inner_vertex]);
        }

        Mesh::new(positions, uvs, vec![], indices, PrimitiveTopology::TriangleList)
    }
}

impl Extrudable for AnnulusBuilder {
    fn perimeter(&self) -> Vec<PerimeterSegment> {
        let vert_count = 2 * self.resolution;
        vec![
            PerimeterSegment::Smooth {
                indices: (0..vert_count).step_by(2).chain([0]).rev().collect(),
            },
            PerimeterSegment::Smooth {
                indices: (1..vert_count).step_by(2).chain([1]).collect(),
            },
        ]
    }
}
