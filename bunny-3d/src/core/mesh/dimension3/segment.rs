use glam::Vec3;

use crate::{
    draw_list::PrimitiveTopology,
    mesh::{Mesh, MeshBuilder},
};

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SegmentBuilder {
    pub vertices: [Vec3; 2],
}

impl Default for SegmentBuilder {
    fn default() -> Self {
        Self {
            vertices: [Vec3::new(-0.5, 0.0, 0.0), Vec3::new(0.5, 0.0, 0.0)],
        }
    }
}

impl SegmentBuilder {
    #[inline]
    pub const fn new(point1: Vec3, point2: Vec3) -> Self {
        Self {
            vertices: [point1, point2],
        }
    }

    #[inline]
    pub fn from_direction_and_length(direction: Vec3, length: f32) -> Self {
        let endpoint = 0.5 * length * direction;
        Self {
            vertices: [-endpoint, endpoint],
        }
    }

    #[inline]
    pub fn from_scaled_direction(scaled_direction: Vec3) -> Self {
        let endpoint = 0.5 * scaled_direction;
        Self {
            vertices: [-endpoint, endpoint],
        }
    }
}

impl MeshBuilder for SegmentBuilder {
    fn build(&self) -> Mesh {
        let positions = self.vertices.map(|v| v.to_array()).into();
        let indices = vec![0, 1];
        Mesh::new(positions, vec![], vec![], indices, PrimitiveTopology::LineList)
    }
}
