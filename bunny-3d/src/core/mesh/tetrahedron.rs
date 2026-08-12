use glam::{Mat3, Vec3};

use crate::core::mesh::{
    Mesh, MeshBuilder,
    triangle::{Triangle3d, uv_coords},
};

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

impl MeshBuilder for TetrahedronMesh {
    fn build(&self) -> Mesh {
        // From https://docs.rs/bevy_mesh/0.19.0/src/bevy_mesh/primitives/dim3/tetrahedron.rs.html#15
        let mut faces: Vec<_> = self.faces().into();

        // If the tetrahedron has negative orientation, reverse all the triangles so that
        // they still face outward.
        if self.signed_volume().is_sign_negative() {
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

        Mesh {
            positions,
            uvs,
            indices,
        }
    }
}

impl From<TetrahedronMesh> for Mesh {
    fn from(value: TetrahedronMesh) -> Self {
        value.build()
    }
}
