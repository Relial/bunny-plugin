use crate::{
    draw_list::PrimitiveTopology,
    mesh::{Mesh, MeshBuilder},
};

#[derive(Clone, Copy, Debug)]
pub struct CylinderBuilder {
    pub radius: f32,
    pub half_height: f32,
    pub resolution: u32,
    pub segments: u32,
    pub caps: bool,
    pub anchor: CylinderAnchor,
}

impl Default for CylinderBuilder {
    fn default() -> Self {
        Self {
            radius: 0.5,
            half_height: 0.5,
            resolution: 24,
            segments: 1,
            caps: true,
            anchor: Default::default(),
        }
    }
}

impl CylinderBuilder {
    #[inline]
    pub const fn new(radius: f32, height: f32) -> Self {
        Self {
            radius,
            half_height: height / 2.0,
            resolution: 24,
            segments: 1,
            caps: true,
            anchor: CylinderAnchor::MidPoint,
        }
    }

    #[inline]
    pub const fn resolution(mut self, resolution: u32) -> Self {
        self.resolution = resolution;
        self
    }

    #[inline]
    pub const fn segments(mut self, segments: u32) -> Self {
        self.segments = segments;
        self
    }

    #[inline]
    pub const fn without_caps(mut self) -> Self {
        self.caps = false;
        self
    }

    #[inline]
    pub const fn anchor(mut self, anchor: CylinderAnchor) -> Self {
        self.anchor = anchor;
        self
    }
}

impl MeshBuilder for CylinderBuilder {
    fn build(&self) -> Mesh {
        // From https://docs.rs/bevy_mesh/0.19.1/src/bevy_mesh/primitives/dim3/cylinder.rs.html#97
        let resolution = self.resolution;
        let segments = self.segments;

        let num_rings = segments + 1;
        let num_vertices = resolution * 2 + num_rings * (resolution + 1);
        let num_faces = resolution * (num_rings - 2);
        let num_indices = (2 * num_faces + 2 * (resolution - 1) * 2) * 3;

        let mut positions = Vec::with_capacity(num_vertices as usize);
        let mut uvs = Vec::with_capacity(num_vertices as usize);
        let mut indices = Vec::with_capacity(num_indices as usize);

        let step_theta = core::f32::consts::TAU / resolution as f32;
        let step_y = 2.0 * self.half_height / segments as f32;

        // rings

        for ring in 0..num_rings {
            let y = -self.half_height + ring as f32 * step_y;

            for segment in 0..=resolution {
                let theta = segment as f32 * step_theta;
                let (sin, cos) = f32::sin_cos(theta);

                positions.push([self.radius * cos, y, self.radius * sin]);
                uvs.push([
                    segment as f32 / resolution as f32,
                    ring as f32 / segments as f32,
                ]);
            }
        }

        // barrel skin

        for i in 0..segments {
            let ring = i * (resolution + 1);
            let next_ring = (i + 1) * (resolution + 1);

            for j in 0..resolution {
                indices.extend_from_slice(&[
                    ring + j,
                    next_ring + j,
                    ring + j + 1,
                    next_ring + j,
                    next_ring + j + 1,
                    ring + j + 1,
                ]);
            }
        }

        // caps
        if self.caps {
            let mut build_cap = |top: bool| {
                let offset = positions.len() as u32;
                let (y, winding) = if top {
                    (self.half_height, (1, 0))
                } else {
                    (-self.half_height, (0, 1))
                };

                for i in 0..self.resolution {
                    let theta = i as f32 * step_theta;
                    let (sin, cos) = f32::sin_cos(theta);

                    positions.push([cos * self.radius, y, sin * self.radius]);
                    uvs.push([0.5 * (cos + 1.0), 1.0 - 0.5 * (sin + 1.0)]);
                }

                for i in 1..(self.resolution - 1) {
                    indices.extend_from_slice(&[
                        offset,
                        offset + i + winding.0,
                        offset + i + winding.1,
                    ]);
                }
            };

            build_cap(true);
            build_cap(false);
        }

        // Offset the vertex positions Y axis to match the anchor
        match self.anchor {
            CylinderAnchor::Top => positions.iter_mut().for_each(|p| p[1] -= self.half_height),
            CylinderAnchor::Bottom => positions.iter_mut().for_each(|p| p[1] += self.half_height),
            CylinderAnchor::MidPoint => (),
        };

        Mesh::new(positions, uvs, indices, PrimitiveTopology::TriangleList)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum CylinderAnchor {
    #[default]
    MidPoint,
    Top,
    Bottom,
}
