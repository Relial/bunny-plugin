use glam::Vec3;

use crate::{
    draw_list::PrimitiveTopology,
    mesh::{Mesh, MeshBuilder},
};

#[derive(Clone, Copy, Debug)]
pub struct CuboidBuilder {
    pub half_size: Vec3,
}

impl Default for CuboidBuilder {
    fn default() -> Self {
        Self {
            half_size: Vec3::splat(0.5),
        }
    }
}

impl CuboidBuilder {
    #[inline]
    pub const fn new(x_length: f32, y_length: f32, z_length: f32) -> Self {
        Self::from_size(Vec3::new(x_length, y_length, z_length))
    }

    #[inline]
    pub const fn from_size(size: Vec3) -> Self {
        Self {
            half_size: Vec3::new(size.x / 2.0, size.y / 2.0, size.z / 2.0),
        }
    }

    #[inline]
    pub fn from_corners(point1: Vec3, point2: Vec3) -> Self {
        Self {
            half_size: (point2 - point1).abs() / 2.0,
        }
    }

    #[inline]
    pub const fn from_length(length: f32) -> Self {
        Self {
            half_size: Vec3::splat(length / 2.0),
        }
    }

    #[inline]
    pub fn size(&self) -> Vec3 {
        2.0 * self.half_size
    }
}

impl MeshBuilder for CuboidBuilder {
    fn build(&self) -> Mesh {
        // From https://docs.rs/bevy_mesh/0.19.0/src/bevy_mesh/primitives/dim3/cuboid.rs.html#23
        let min = -self.half_size;
        let max = self.half_size;

        // Suppose Y-up right hand, and camera look from +Z to -Z
        let vertices = &[
            // Front
            ([min.x, min.y, max.z], [0.0, 0.0]),
            ([max.x, min.y, max.z], [1.0, 0.0]),
            ([max.x, max.y, max.z], [1.0, 1.0]),
            ([min.x, max.y, max.z], [0.0, 1.0]),
            // Back
            ([min.x, max.y, min.z], [1.0, 0.0]),
            ([max.x, max.y, min.z], [0.0, 0.0]),
            ([max.x, min.y, min.z], [0.0, 1.0]),
            ([min.x, min.y, min.z], [1.0, 1.0]),
            // Right
            ([max.x, min.y, min.z], [0.0, 0.0]),
            ([max.x, max.y, min.z], [1.0, 0.0]),
            ([max.x, max.y, max.z], [1.0, 1.0]),
            ([max.x, min.y, max.z], [0.0, 1.0]),
            // Left
            ([min.x, min.y, max.z], [1.0, 0.0]),
            ([min.x, max.y, max.z], [0.0, 0.0]),
            ([min.x, max.y, min.z], [0.0, 1.0]),
            ([min.x, min.y, min.z], [1.0, 1.0]),
            // Top
            ([max.x, max.y, min.z], [1.0, 0.0]),
            ([min.x, max.y, min.z], [0.0, 0.0]),
            ([min.x, max.y, max.z], [0.0, 1.0]),
            ([max.x, max.y, max.z], [1.0, 1.0]),
            // Bottom
            ([max.x, min.y, max.z], [0.0, 0.0]),
            ([min.x, min.y, max.z], [1.0, 0.0]),
            ([min.x, min.y, min.z], [1.0, 1.0]),
            ([max.x, min.y, min.z], [0.0, 1.0]),
        ];

        let positions: Vec<_> = vertices.iter().map(|(p, _)| *p).collect();
        let uvs: Vec<_> = vertices.iter().map(|(_, uv)| *uv).collect();

        let indices = vec![
            0, 1, 2, 2, 3, 0, // front
            4, 5, 6, 6, 7, 4, // back
            8, 9, 10, 10, 11, 8, // right
            12, 13, 14, 14, 15, 12, // left
            16, 17, 18, 18, 19, 16, // top
            20, 21, 22, 22, 23, 20, // bottom
        ];

        Mesh::new(positions, uvs, indices, PrimitiveTopology::TriangleList)
    }
}
