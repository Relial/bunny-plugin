use bytemuck::cast_slice;

use crate::backend::GpuColor;

pub mod capsule;
pub mod sphere;

#[derive(Clone, Debug)]
pub struct Mesh {
    pub vertices: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    pub color: GpuColor,
    pub indices: Vec<u32>,
}

impl Mesh {
    pub fn new(vertices: Vec<[f32; 3]>, indices: Vec<u32>) -> Self {
        Self {
            vertices,
            uvs: vec![],
            color: GpuColor::WHITE,
            indices,
        }
    }

    #[inline]
    pub fn uvs(mut self, uvs: Vec<[f32; 2]>) -> Self {
        self.uvs = uvs;
        self
    }

    #[inline]
    pub fn color(mut self, color: impl Into<GpuColor>) -> Self {
        self.color = color.into();
        self
    }

    #[inline]
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    #[inline]
    pub fn index_count(&self) -> usize {
        self.indices.len()
    }

    pub fn update_vertex_buffer(&self, vertex_buffer: &mut [u8], vertex_size: usize) {
        let position_size = std::mem::size_of::<[f32; 3]>();
        let vertex_count = self.vertices.len();
        let positions: &[u8] = cast_slice(&self.vertices);
        for (vertex_index, position_bytes) in positions
            .chunks_exact(position_size)
            .take(vertex_count)
            .enumerate()
        {
            let offset = vertex_index * vertex_size;
            vertex_buffer[offset..offset + position_size].copy_from_slice(position_bytes);
            let offset = offset + position_size;
            vertex_buffer[offset..offset + std::mem::size_of::<GpuColor>()]
                .copy_from_slice(self.color.as_bytes());
        }

        let uv_size = std::mem::size_of::<[f32; 2]>();
        let uvs: &[u8] = cast_slice(&self.uvs);
        for (vertex_index, uv_bytes) in uvs.chunks_exact(uv_size).take(vertex_count).enumerate() {
            let offset =
                vertex_index * vertex_size + position_size + std::mem::size_of::<GpuColor>();
            vertex_buffer[offset..offset + uv_size].copy_from_slice(uv_bytes);
        }
    }

    pub fn index_buffer_bytes(&self) -> &[u8] {
        cast_slice(&self.indices)
    }
}

#[cfg(feature = "bevy")]
impl TryFrom<bevy_mesh::Mesh> for Mesh {
    type Error = anyhow::Error;

    fn try_from(mut value: bevy_mesh::Mesh) -> Result<Self, Self::Error> {
        use anyhow::{anyhow, bail};
        use bevy_mesh::{Indices, VertexAttributeValues};

        let vertices = value
            .remove_attribute(bevy_mesh::Mesh::ATTRIBUTE_POSITION)
            .ok_or(anyhow!("Bevy mesh missing ATTRIBUTE_POSITION"))?;
        let VertexAttributeValues::Float32x3(vertices) = vertices else {
            bail!("ATTRIBUTE_POSITION must be Float32x3");
        };
        let uvs = value
            .remove_attribute(bevy_mesh::Mesh::ATTRIBUTE_UV_0)
            .map(|v| match v {
                VertexAttributeValues::Float32x2(uvs) => uvs,
                _ => vec![],
            });
        let indices = value
            .remove_indices()
            .ok_or(anyhow!("Bevy mesh missing indices"))?;
        let indices = match indices {
            Indices::U16(items) => items.into_iter().map(|i| i as u32).collect(),
            Indices::U32(items) => items,
        };
        Ok(Mesh {
            vertices,
            uvs: uvs.unwrap_or_default(),
            color: GpuColor::WHITE,
            indices,
        })
    }
}
