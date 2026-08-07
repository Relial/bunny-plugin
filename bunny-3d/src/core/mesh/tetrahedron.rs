use glam::{Mat3, Vec3};

use crate::core::mesh::Mesh;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TetrahedronMesh {
    pub vertices: [Vec3; 4],
}

// From https://docs.rs/bevy_math/0.19.0/src/bevy_math/primitives/dim3.rs.html#1440

impl Default for TetrahedronMesh {
    fn default() -> Self {
        Self {
            vertices: [
                Vec3::new(0.5, 0.5, 0.5),
                Vec3::new(-0.5, 0.5, -0.5),
                Vec3::new(-0.5, -0.5, 0.5),
                Vec3::new(0.5, -0.5, -0.5),
            ],
        }
    }
}

impl TetrahedronMesh {
    #[inline]
    pub const fn new(a: Vec3, b: Vec3, c: Vec3, d: Vec3) -> Self {
        Self {
            vertices: [a, b, c, d],
        }
    }

    #[inline]
    pub fn signed_volume(&self) -> f32 {
        let [a, b, c, d] = self.vertices;
        let ab = b - a;
        let ac = c - a;
        let ad = d - a;
        Mat3::from_cols(ab, ac, ad).determinant() / 6.0
    }

    #[inline]
    fn faces(&self) -> [Triangle3d; 4] {
        let [a, b, c, d] = self.vertices;
        [
            Triangle3d::new(b, c, d),
            Triangle3d::new(a, c, d).reversed(),
            Triangle3d::new(a, b, d),
            Triangle3d::new(a, b, c).reversed(),
        ]
    }
}

impl From<TetrahedronMesh> for Mesh {
    fn from(value: TetrahedronMesh) -> Self {
        // From https://docs.rs/bevy_mesh/0.19.0/src/bevy_mesh/primitives/dim3/tetrahedron.rs.html#15
        let mut faces: Vec<_> = value.faces().into();

        // If the tetrahedron has negative orientation, reverse all the triangles so that
        // they still face outward.
        if value.signed_volume().is_sign_negative() {
            faces.iter_mut().for_each(Triangle3d::reverse);
        }

        let mut positions = vec![];
        let mut uvs = vec![];

        // Each face is meshed as a `Triangle3d`, and we just shove the data into the
        // vertex attributes sequentially.
        for face in faces {
            positions.extend(face.vertices.into_iter().map(|v| v.to_array()));

            let face_uvs = uv_coords(&face);
            uvs.extend(face_uvs);
        }

        // There are four faces and none of them share vertices.
        let indices = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];

        Self {
            positions,
            uvs,
            indices,
        }
    }
}

struct Triangle3d {
    vertices: [Vec3; 3],
}

impl Triangle3d {
    #[inline]
    const fn new(a: Vec3, b: Vec3, c: Vec3) -> Self {
        Self {
            vertices: [a, b, c],
        }
    }

    #[inline]
    fn reverse(&mut self) {
        self.vertices.swap(0, 2);
    }

    #[inline]
    pub fn reversed(mut self) -> Self {
        self.reverse();
        self
    }
}

#[inline]
fn uv_coords(triangle: &Triangle3d) -> [[f32; 2]; 3] {
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
