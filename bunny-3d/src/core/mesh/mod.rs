use anyhow::{Result, anyhow};
use bytemuck::cast_slice;
use glam::{Quat, Vec3};

use crate::{GpuColor, Transform, core::draw_list::PrimitiveTopology};

mod dimension2;
mod dimension3;
mod extrusion;

pub use dimension2::*;
pub use dimension3::*;
pub use extrusion::*;

pub trait MeshBuilder {
    fn build(&self) -> Mesh;
}

impl<T: MeshBuilder> From<T> for Mesh {
    fn from(value: T) -> Self {
        value.build()
    }
}

#[derive(Clone, Debug)]
pub struct Mesh {
    pub positions: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    pub colors: Vec<GpuColor>,
    pub indices: Vec<u32>,
    pub primitive_topology: PrimitiveTopology,
}

impl Mesh {
    pub fn new(
        positions: Vec<[f32; 3]>,
        uvs: Vec<[f32; 2]>,
        colors: Vec<GpuColor>,
        indices: Vec<u32>,
        primitive_topology: PrimitiveTopology,
    ) -> Self {
        Self {
            positions,
            uvs,
            colors,
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

    pub fn transformed_by(mut self, transform: Transform) -> Self {
        self.transform_by(transform);
        self
    }

    pub fn transform_by(&mut self, transform: Transform) {
        self.positions
            .iter_mut()
            .for_each(|pos| *pos = transform.transform_point(Vec3::from_slice(pos)).to_array());
    }

    pub fn translated_by(mut self, translation: Vec3) -> Self {
        self.translate_by(translation);
        self
    }

    pub fn translate_by(&mut self, translation: Vec3) {
        self.positions
            .iter_mut()
            .for_each(|pos| *pos = (Vec3::from_slice(pos) + translation).to_array());
    }

    pub fn rotated_by(mut self, rotation: Quat) -> Self {
        self.rotate_by(rotation);
        self
    }

    pub fn rotate_by(&mut self, rotation: Quat) {
        self.positions
            .iter_mut()
            .for_each(|pos| *pos = (rotation * Vec3::from_slice(pos)).to_array());
    }

    pub fn scaled_by(mut self, scale: Vec3) -> Self {
        self.scale_by(scale);
        self
    }

    pub fn scale_by(&mut self, scale: Vec3) {
        self.positions
            .iter_mut()
            .for_each(|pos| *pos = (scale * Vec3::from_slice(pos)).to_array());
    }

    pub fn merge(&mut self, other: &Mesh) -> Result<()> {
        if self.primitive_topology != other.primitive_topology {
            return Err(anyhow!(
                "Can't merge meshes with different primitive topologies"
            ));
        }
        let index_offset = self.vertex_count();
        self.positions.extend(&other.positions);
        self.uvs.extend(&other.uvs);
        self.indices
            .extend(other.indices.iter().map(|i| i + index_offset as u32));
        Ok(())
    }
}

impl Mesh {
    pub(crate) fn update_vertex_buffer(
        &self,
        vertex_buffer: &mut [u8],
        vertex_size: usize,
        override_color: Option<GpuColor>,
    ) {
        const POSITION_SIZE: usize = std::mem::size_of::<[f32; 3]>();
        const COLOR_SIZE: usize = std::mem::size_of::<GpuColor>();
        const UV_SIZE: usize = std::mem::size_of::<[f32; 2]>();

        let vertex_count = self.positions.len();

        let positions: &[u8] = cast_slice(&self.positions);
        for (vertex_index, position_bytes) in positions
            .as_chunks::<POSITION_SIZE>()
            .0
            .iter()
            .take(vertex_count)
            .enumerate()
        {
            let offset = vertex_index * vertex_size;
            vertex_buffer[offset..offset + POSITION_SIZE].copy_from_slice(position_bytes);
        }

        if let Some(color) = override_color {
            let color_bytes = color.as_bytes();
            for vertex_index in 0..vertex_count {
                let offset = vertex_index * vertex_size + POSITION_SIZE;
                vertex_buffer[offset..offset + COLOR_SIZE].copy_from_slice(color_bytes);
            }
        } else if !self.colors.is_empty() {
            let colors: &[u8] = cast_slice(&self.colors);
            for (vertex_index, color_bytes) in colors
                .as_chunks::<COLOR_SIZE>()
                .0
                .iter()
                .take(vertex_count)
                .enumerate()
            {
                let offset = vertex_index * vertex_size + POSITION_SIZE;
                vertex_buffer[offset..offset + COLOR_SIZE].copy_from_slice(color_bytes);
            }
        } else {
            let color_bytes = GpuColor::WHITE.as_bytes();
            for vertex_index in 0..vertex_count {
                let offset = vertex_index * vertex_size + POSITION_SIZE;
                vertex_buffer[offset..offset + COLOR_SIZE].copy_from_slice(color_bytes);
            }
        }

        let uvs: &[u8] = cast_slice(&self.uvs);
        for (vertex_index, uv_bytes) in uvs
            .as_chunks::<UV_SIZE>()
            .0
            .iter()
            .take(vertex_count)
            .enumerate()
        {
            let offset = vertex_index * vertex_size + POSITION_SIZE + COLOR_SIZE;
            vertex_buffer[offset..offset + UV_SIZE].copy_from_slice(uv_bytes);
        }
    }

    pub(crate) fn index_buffer_bytes(&self) -> &[u8] {
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
            })
            .unwrap_or_default();
        let colors = value
            .remove_attribute(bevy_mesh::Mesh::ATTRIBUTE_COLOR)
            .map(|v| match v {
                VertexAttributeValues::Float32x4(colors) => colors
                    .into_iter()
                    .map(|rgba| GpuColor::from_rgba_float(rgba[0], rgba[1], rgba[2], rgba[3]))
                    .collect(),
                _ => vec![],
            })
            .unwrap_or_default();
        let indices = value
            .remove_indices()
            .ok_or(anyhow!("Bevy mesh missing indices"))?;
        let indices = match indices {
            Indices::U16(items) => items.into_iter().map(|i| i as u32).collect(),
            Indices::U32(items) => items,
        };
        Ok(Mesh {
            positions: vertices,
            uvs,
            colors,
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
        let colors: Vec<GpuColor> = mesh
            .vertex_color
            .as_chunks::<3>()
            .0
            .iter()
            .map(|rgb| GpuColor::from_rgba_float(rgb[0], rgb[1], rgb[2], 1.0))
            .collect();
        if uv_origin == UvOrigin::BottomLeft {
            uvs.iter_mut().for_each(|uv| uv[1] = 1.0 - uv[1]);
        }
        let indices = mesh.indices;
        Ok(Self::new(
            vertices,
            uvs,
            colors,
            indices,
            PrimitiveTopology::TriangleList,
        ))
    }
}
