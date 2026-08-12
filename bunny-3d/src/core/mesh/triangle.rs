use glam::Vec3;

use crate::core::{
    draw_list::PrimitiveTopology,
    mesh::{Mesh, MeshBuilder},
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Triangle3d {
    pub vertices: [Vec3; 3],
}

impl Triangle3d {
    #[inline]
    pub const fn new(a: Vec3, b: Vec3, c: Vec3) -> Self {
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

impl MeshBuilder for Triangle3d {
    fn build(&self) -> Mesh {
        let positions = self.vertices.iter().map(|v| v.to_array()).collect();
        let uvs = uv_coords(self).into();
        let indices = vec![0, 1, 2];
        Mesh::new(positions, uvs, indices, PrimitiveTopology::TriangleList)
    }
}

impl From<Triangle3d> for Mesh {
    fn from(value: Triangle3d) -> Self {
        value.build()
    }
}

#[inline]
pub(crate) fn uv_coords(triangle: &Triangle3d) -> [[f32; 2]; 3] {
    // From https://docs.rs/bevy_mesh/0.19.0/src/bevy_mesh/primitives/dim3/triangle3d.rs.html#51
    let [a, b, c] = triangle.vertices;

    let main_length = a.distance(b);
    let Some(x) = (b - a).try_normalize() else {
        return [[0., 0.], [1., 0.], [0., 1.]];
    };
    let y = c - a;

    // `x` corresponds to one of the axes in uv-coordinates;
    // to uv-map the triangle without skewing, we use the orthogonalization
    // of `y` with respect to `x` as the second direction and construct a rectangle that
    // contains `triangle`.
    let y_proj = y.project_onto_normalized(x);

    // `offset` represents the x-coordinate of the point `c`; note that x has been shrunk by a
    // factor of `main_length`, so `offset` follows it.
    let offset = y_proj.dot(x) / main_length;

    // Obtuse triangle leaning to the left => x direction extends to the left, shifting a from 0.
    if offset < 0. {
        let total_length = 1. - offset;
        let a_uv = [offset.abs() / total_length, 0.];
        let b_uv = [1., 0.];
        let c_uv = [0., 1.];

        [a_uv, b_uv, c_uv]
    }
    // Obtuse triangle leaning to the right => x direction extends to the right, shifting b from 1.
    else if offset > 1. {
        let a_uv = [0., 0.];
        let b_uv = [1. / offset, 0.];
        let c_uv = [1., 1.];

        [a_uv, b_uv, c_uv]
    }
    // Acute triangle => no extending necessary; a remains at 0 and b remains at 1.
    else {
        let a_uv = [0., 0.];
        let b_uv = [1., 0.];
        let c_uv = [offset, 1.];

        [a_uv, b_uv, c_uv]
    }
}
