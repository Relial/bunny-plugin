use glam::Vec2;

use crate::{
    draw_list::PrimitiveTopology,
    mesh::{Extrudable, Mesh, MeshBuilder, PerimeterSegment, Primitive2d},
};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ConvexPolygonBuilder {
    pub vertices: Vec<Vec2>,
}

impl Primitive2d for ConvexPolygonBuilder {}

impl MeshBuilder for ConvexPolygonBuilder {
    fn build(&self) -> Mesh {
        // From https://docs.rs/bevy_mesh/0.19.1/src/bevy_mesh/primitives/dim2.rs.html#430
        let len = self.vertices.len();
        let mut indices = Vec::with_capacity((len - 2) * 3);
        let mut positions = Vec::with_capacity(len);
        let mut uvs = Vec::with_capacity(len);

        let mut min = Vec2::splat(f32::INFINITY);
        let mut max = Vec2::splat(f32::NEG_INFINITY);

        for vertex in &self.vertices {
            min = min.min(*vertex);
            max = max.max(*vertex);
        }

        let size = (max - min).max(Vec2::splat(f32::EPSILON));

        for vertex in &self.vertices {
            positions.push([vertex.x, vertex.y, 0.0]);
            let uv = (*vertex - min) / size;
            // Map each axis independently into [0, 1] over the polygon's AABB.
            uvs.push([uv.x, uv.y]);
        }
        for i in 2..len as u32 {
            indices.extend_from_slice(&[0, i - 1, i]);
        }
        Mesh::new(positions, uvs, vec![], indices, PrimitiveTopology::TriangleList)
    }
}

impl Extrudable for ConvexPolygonBuilder {
    fn perimeter(&self) -> Vec<PerimeterSegment> {
        vec![PerimeterSegment::Flat {
            indices: (0..self.vertices.len() as u32).chain([0]).collect(),
        }]
    }
}
