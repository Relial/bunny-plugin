use abi_stable::std_types::{
    ROption::{self, RSome},
    RVec,
};
use anyhow::{Result, anyhow};
use egui::{
    Color32, Context, Pos2, Rect, TextureId, Vec2,
    emath::{Rot2, TSTransform},
    epaint::{Vertex, WHITE_UV},
};

use crate::image_source::ImageSource;

#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Mesh<'a> {
    pub texture_source: ROption<ImageSource<'a>>,
    pub indices: RVec<u32>,
    pub vertices: RVec<Vertex>,
}

impl<'a> Mesh<'a> {
    pub fn with_texture(texture_source: ImageSource<'a>) -> Self {
        Self {
            texture_source: RSome(texture_source),
            ..Default::default()
        }
    }

    pub fn clear(&mut self) {
        self.indices.clear();
        self.vertices.clear();
        self.vertices = Default::default();
    }

    pub fn is_valid(&self) -> bool {
        if let Ok(n) = u32::try_from(self.vertices.len()) {
            self.indices.iter().all(|&i| i < n)
        } else {
            false
        }
    }

    pub fn is_empty(&self) -> bool {
        self.indices.is_empty() && self.vertices.is_empty()
    }

    pub fn triangles(&self) -> impl Iterator<Item = [u32; 3]> {
        self.indices
            .as_chunks::<3>()
            .0
            .iter()
            .map(|chunk| [chunk[0], chunk[1], chunk[2]])
    }

    pub fn calc_bounds(&self) -> Rect {
        let mut bounds = Rect::NOTHING;
        for v in &self.vertices {
            bounds.extend_with(v.pos);
        }
        bounds
    }

    #[inline(always)]
    pub fn colored_vertex(&mut self, pos: Pos2, color: Color32) {
        debug_assert!(
            self.texture_source.is_none(),
            "Mesh has an assigned texture"
        );
        self.vertices.push(Vertex::untextured(pos, color));
    }

    #[inline(always)]
    pub fn add_triangle(&mut self, a: u32, b: u32, c: u32) {
        self.indices.extend_from_slice(&[a, b, c]);
    }

    #[inline(always)]
    pub fn reserve_triangles(&mut self, additional_triangles: usize) {
        self.indices.reserve(3 * additional_triangles);
    }

    #[inline(always)]
    pub fn reserve_vertices(&mut self, additional: usize) {
        self.vertices.reserve(additional);
    }

    #[inline(always)]
    pub fn add_rect_with_uv(&mut self, rect: Rect, uv: Rect, color: Color32) {
        #![expect(clippy::identity_op)]
        let idx = self.vertices.len() as u32;
        self.indices
            .extend_from_slice(&[idx + 0, idx + 1, idx + 2, idx + 2, idx + 1, idx + 3]);

        self.vertices.extend_from_slice(&[
            Vertex {
                pos: rect.left_top(),
                uv: uv.left_top(),
                color,
            },
            Vertex {
                pos: rect.right_top(),
                uv: uv.right_top(),
                color,
            },
            Vertex {
                pos: rect.left_bottom(),
                uv: uv.left_bottom(),
                color,
            },
            Vertex {
                pos: rect.right_bottom(),
                uv: uv.right_bottom(),
                color,
            },
        ]);
    }

    #[inline(always)]
    pub fn add_colored_rect(&mut self, rect: Rect, color: Color32) {
        debug_assert!(
            self.texture_source.is_none(),
            "Mesh has an assigned texture"
        );
        self.add_rect_with_uv(rect, [WHITE_UV, WHITE_UV].into(), color);
    }

    pub fn translate(&mut self, delta: Vec2) {
        for v in &mut self.vertices {
            v.pos += delta;
        }
    }

    pub fn transform(&mut self, transform: TSTransform) {
        for v in &mut self.vertices {
            v.pos = transform * v.pos
        }
    }

    pub fn rotate(&mut self, rot: Rot2, origin: Pos2) {
        for v in &mut self.vertices {
            v.pos = origin + rot * (v.pos - origin);
        }
    }
}

impl<'a> Mesh<'a> {
    pub fn to_egui(self, ctx: &Context) -> Result<egui::Mesh> {
        let texture_id = if let RSome(texture_loader) = self.texture_source {
            let texture_poll = texture_loader.convert_to_texture(ctx)?;
            match texture_poll {
                egui::load::TexturePoll::Pending { size: _ } => {
                    return Err(anyhow!("Texture is loading"));
                }
                egui::load::TexturePoll::Ready { texture } => texture.id,
            }
        } else {
            TextureId::Managed(0)
        };
        Ok(egui::Mesh {
            indices: self.indices.into(),
            vertices: self.vertices.into(),
            texture_id,
        })
    }
}
