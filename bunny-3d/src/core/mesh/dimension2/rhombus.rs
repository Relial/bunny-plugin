use std::f32::consts::FRAC_1_SQRT_2;

use glam::Vec2;

use crate::{
    draw_list::PrimitiveTopology,
    mesh::{Extrudable, Mesh, MeshBuilder, PerimeterSegment, Primitive2d},
};

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RhombusBuilder {
    pub half_diagonals: Vec2,
}

impl Default for RhombusBuilder {
    fn default() -> Self {
        Self {
            half_diagonals: Vec2::splat(0.5),
        }
    }
}

impl RhombusBuilder {
    #[inline]
    pub const fn new(horizontal_diagonal: f32, vertical_diagonal: f32) -> Self {
        Self {
            half_diagonals: Vec2::new(horizontal_diagonal / 2.0, vertical_diagonal / 2.0),
        }
    }

    #[inline]
    pub const fn from_side(side: f32) -> Self {
        Self {
            half_diagonals: Vec2::splat(side * FRAC_1_SQRT_2),
        }
    }

    #[inline]
    pub const fn from_inradius(inradius: f32) -> Self {
        let half_diagonal = inradius * 2.0 / core::f32::consts::SQRT_2;
        Self {
            half_diagonals: Vec2::new(half_diagonal, half_diagonal),
        }
    }
}

impl Primitive2d for RhombusBuilder {}

impl MeshBuilder for RhombusBuilder {
    fn build(&self) -> Mesh {
        // From https://docs.rs/bevy_mesh/0.19.1/src/bevy_mesh/primitives/dim2.rs.html#916

        let [hhd, vhd] = [self.half_diagonals.x, self.half_diagonals.y];
        let positions = vec![
            [hhd, 0.0, 0.0],
            [0.0, vhd, 0.0],
            [-hhd, 0.0, 0.0],
            [0.0, -vhd, 0.0],
        ];
        let uvs = vec![[1.0, 0.5], [0.5, 0.0], [0.0, 0.5], [0.5, 1.0]];
        let indices = vec![2, 0, 1, 2, 3, 0];
        Mesh::new(positions, uvs, vec![], indices, PrimitiveTopology::TriangleList)
    }
}

impl Extrudable for RhombusBuilder {
    fn perimeter(&self) -> Vec<PerimeterSegment> {
        vec![PerimeterSegment::Flat {
            indices: vec![0, 1, 2, 3, 0],
        }]
    }
}
