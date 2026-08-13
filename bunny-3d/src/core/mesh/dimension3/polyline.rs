use glam::Vec3;

use crate::{
    draw_list::PrimitiveTopology,
    mesh::{Mesh, MeshBuilder},
};

#[derive(Clone, Debug)]
pub struct PolyLine {
    pub vertices: Vec<Vec3>,
}

impl FromIterator<Vec3> for PolyLine {
    fn from_iter<T: IntoIterator<Item = Vec3>>(iter: T) -> Self {
        Self {
            vertices: iter.into_iter().collect(),
        }
    }
}

impl Default for PolyLine {
    fn default() -> Self {
        Self {
            vertices: vec![Vec3::new(-0.5, 0.0, 0.0), Vec3::new(0.5, 0.0, 0.0)],
        }
    }
}

impl PolyLine {
    #[inline]
    pub fn new(vertices: impl IntoIterator<Item = Vec3>) -> Self {
        Self::from_iter(vertices)
    }

    pub fn with_subdivisions(start: Vec3, end: Vec3, subdivisions: usize) -> Self {
        let total_vertices = subdivisions + 2;
        let mut vertices = Vec::with_capacity(total_vertices);
        let step = (end - start) / (subdivisions + 1) as f32;
        for i in 0..total_vertices {
            vertices.push(start + step * i as f32);
        }
        Self { vertices }
    }
}

impl MeshBuilder for PolyLine {
    fn build(&self) -> Mesh {
        let positions = self.vertices.iter().map(|v| v.to_array()).collect();
        let indices = (0..self.vertices.len() as u32 - 1)
            .flat_map(|i| [i, i + 1])
            .collect();
        Mesh::new(positions, vec![], indices, PrimitiveTopology::LineList)
    }
}

impl From<PolyLine> for Mesh {
    fn from(value: PolyLine) -> Self {
        value.build()
    }
}
