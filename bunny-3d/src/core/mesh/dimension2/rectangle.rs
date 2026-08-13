use glam::Vec2;

use crate::{
    draw_list::PrimitiveTopology, mesh::{Mesh, MeshBuilder, Primitive2d},
};

#[derive(Clone, Copy, Debug)]
pub struct RectangleMesh {
    pub half_size: Vec2,
}

impl Default for RectangleMesh {
    fn default() -> Self {
        Self {
            half_size: Vec2::splat(0.5),
        }
    }
}

impl RectangleMesh {
    #[inline]
    pub const fn new(width: f32, height: f32) -> Self {
        Self::from_size(Vec2::new(width, height))
    }

    #[inline]
    pub const fn from_size(size: Vec2) -> Self {
        Self {
            half_size: Vec2::new(size.x / 2.0, size.y / 2.0),
        }
    }

    #[inline]
    pub fn from_corners(point1: Vec2, point2: Vec2) -> Self {
        Self {
            half_size: (point2 - point1).abs() / 2.0,
        }
    }

    #[inline]
    pub const fn from_length(length: f32) -> Self {
        Self {
            half_size: Vec2::splat(length / 2.0),
        }
    }
}

impl Primitive2d for RectangleMesh {}

impl MeshBuilder for RectangleMesh {
    fn build(&self) -> Mesh {
        // From https://docs.rs/bevy_mesh/0.19.0/src/bevy_mesh/primitives/dim2.rs.html#1072
        let [hw, hh] = [self.half_size.x, self.half_size.y];
        let positions = vec![
            [hw, hh, 0.0],
            [-hw, hh, 0.0],
            [-hw, -hh, 0.0],
            [hw, -hh, 0.0],
        ];
        let uvs = vec![[1.0, 0.0], [0.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
        let indices = vec![0, 1, 2, 0, 2, 3];
        Mesh::new(positions, uvs, indices, PrimitiveTopology::TriangleList)
    }
}
