use glam::Vec2;

use crate::mesh::{Extrudable, Mesh, MeshBuilder, PerimeterSegment, Primitive2d, TriangleMesh};

#[derive(Clone, Copy, Debug)]
pub struct Triangle2d {
    pub vertices: [Vec2; 3],
}

impl Default for Triangle2d {
    fn default() -> Self {
        Self {
            vertices: [Vec2::Y * 0.5, Vec2::new(-0.5, -0.5), Vec2::new(0.5, -0.5)],
        }
    }
}

impl Triangle2d {
    #[inline]
    pub const fn new(a: Vec2, b: Vec2, c: Vec2) -> Self {
        Self {
            vertices: [a, b, c],
        }
    }

    #[inline]
    pub fn reverse(&mut self) {
        self.vertices.swap(0, 2);
    }

    #[inline]
    pub fn reversed(mut self) -> Self {
        self.reverse();
        self
    }
}

impl Primitive2d for Triangle2d {}

impl MeshBuilder for Triangle2d {
    fn build(&self) -> Mesh {
        let triangle3d = TriangleMesh {
            vertices: self.vertices.map(|v| v.extend(0.0)),
        };
        triangle3d.build()
    }
}

impl Extrudable for Triangle2d {
    fn perimeter(&self) -> Vec<PerimeterSegment> {
        vec![PerimeterSegment::Flat {
            indices: vec![2, 1, 0, 2],
        }]
    }
}
