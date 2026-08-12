#[cfg(feature = "tobj")]
use anyhow::anyhow;
use bytemuck::cast_slice;

use crate::{backend::GpuColor, core::draw_list::PrimitiveTopology};

pub mod capsule;
pub mod circle;
pub mod cuboid;
pub mod ellipse;
pub mod rectangle;
pub mod sphere;
pub mod tetrahedron;
pub mod triangle;
pub mod polyline;

pub trait MeshBuilder {
    fn build(&self) -> Mesh;
}

#[derive(Clone, Debug)]
pub struct Mesh {
    pub positions: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    pub indices: Vec<u32>,
    pub primitive_topology: PrimitiveTopology,
}

impl Mesh {
    pub fn new(
        positions: Vec<[f32; 3]>,
        uvs: Vec<[f32; 2]>,
        indices: Vec<u32>,
        primitive_topology: PrimitiveTopology,
    ) -> Self {
        Self {
            positions,
            uvs,
            indices,
            primitive_topology,
        }
    }

    #[inline]
    pub fn vertex_count(&self) -> usize {
        self.positions.len()
    }

    #[inline]
    pub fn index_count(&self) -> usize {
        self.indices.len()
    }

    pub fn update_vertex_buffer(
        &self,
        vertex_buffer: &mut [u8],
        vertex_size: usize,
        color: GpuColor,
    ) {
        let position_size = std::mem::size_of::<[f32; 3]>();
        let vertex_count = self.positions.len();
        let positions: &[u8] = cast_slice(&self.positions);
        for (vertex_index, position_bytes) in positions
            .chunks_exact(position_size)
            .take(vertex_count)
            .enumerate()
        {
            let offset = vertex_index * vertex_size;
            vertex_buffer[offset..offset + position_size].copy_from_slice(position_bytes);
            let offset = offset + position_size;
            vertex_buffer[offset..offset + std::mem::size_of::<GpuColor>()]
                .copy_from_slice(color.as_bytes());
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

        let primitive_topology: PrimitiveTopology = value.primitive_topology().try_into()?;
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
            positions: vertices,
            uvs: uvs.unwrap_or_default(),
            indices,
            primitive_topology,
        })
    }
}

/// What is considered UV coordinate 0.0, 0.0
///
/// The internal representation is TopLeft. If you specify BottomLeft the y axis will be flipped during mesh conversion.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[repr(C)]
pub enum UvOrigin {
    /// Top left is 0.0, 0.0, bottom right is 1.0, 1.0
    ///
    /// e.g. DirectX, Unreal, glTF
    #[default]
    TopLeft,
    /// Bottom left is 0.0, 0.0, top right is 1.0, 1.0
    ///
    /// e.g. OpenGL, Blender, Maya, Unity
    BottomLeft,
}

#[cfg(feature = "tobj")]
pub struct TobjMesh {
    pub mesh: tobj::Mesh,
    pub uv_origin: UvOrigin,
}

#[cfg(feature = "tobj")]
impl TobjMesh {
    pub const fn new(mesh: tobj::Mesh, uv_origin: UvOrigin) -> Self {
        Self { mesh, uv_origin }
    }
}

#[cfg(feature = "tobj")]
impl TryFrom<TobjMesh> for Mesh {
    type Error = anyhow::Error;

    fn try_from(value: TobjMesh) -> std::prelude::v1::Result<Self, Self::Error> {
        use bytemuck::try_cast_slice;

        let TobjMesh { mesh, uv_origin } = value;

        let vertices: Vec<[f32; 3]> = try_cast_slice(&mesh.positions)
            .map_err(|e| anyhow!("Failed to cast positions to [f32; 3]: {e:#}"))?
            .to_vec();
        let mut uvs: Vec<[f32; 2]> = try_cast_slice(&mesh.texcoords)
            .map_err(|e| anyhow!("Failed to cast texcoords to [f32; 2]: {e:#}"))?
            .to_vec();
        if uv_origin == UvOrigin::BottomLeft {
            uvs.iter_mut().for_each(|uv| uv[1] = 1.0 - uv[1]);
        }
        let indices = mesh.indices;
        Ok(Self::new(
            vertices,
            uvs,
            indices,
            PrimitiveTopology::TriangleList,
        ))
    }
}
