// From https://docs.rs/bevy_math/0.19.1/src/bevy_math/primitives/inset.rs.html

use glam::Vec2;

use crate::mesh::{Capsule2dMesh, CircleMesh, Primitive2d, RectangleMesh, RhombusMesh, Triangle2d};

pub trait Inset: Primitive2d {
    fn inset(self, distance: f32) -> Self;
}

impl Inset for CircleMesh {
    fn inset(mut self, distance: f32) -> Self {
        self.radius -= distance;
        self
    }
}

impl Inset for Triangle2d {
    fn inset(self, distance: f32) -> Self {
        fn find_inset_point(a: Vec2, b: Vec2, c: Vec2, distance: f32) -> Vec2 {
            let unit_vector_ab = (b - a).normalize();
            let unit_vector_ac = (c - a).normalize();
            let half_angle_bac = unit_vector_ab.angle_to(unit_vector_ac) / 2.0;
            let mean = (unit_vector_ab + unit_vector_ac) / 2.0;
            let direction = mean.normalize();
            let magnitude = distance / f32::sin(half_angle_bac);
            a + direction * magnitude
        }

        let [a, b, c] = self.vertices;

        let new_a = find_inset_point(a, b, c, distance);
        let new_b = find_inset_point(b, c, a, distance);
        let new_c = find_inset_point(c, a, b, distance);

        Self::new(new_a, new_b, new_c)
    }
}

impl Inset for RhombusMesh {
    fn inset(mut self, distance: f32) -> Self {
        let [half_width, half_height] = self.half_diagonals.into();
        let angle = f32::atan(half_height / half_width);
        let x_offset = distance / f32::sin(angle);
        let y_offset = distance / f32::cos(angle);
        self.half_diagonals -= Vec2::new(x_offset, y_offset);
        self
    }
}

impl Inset for Capsule2dMesh {
    fn inset(mut self, distance: f32) -> Self {
        self.radius -= distance;
        self
    }
}

impl Inset for RectangleMesh {
    fn inset(mut self, distance: f32) -> Self {
        self.half_size -= Vec2::splat(distance);
        self
    }
}
