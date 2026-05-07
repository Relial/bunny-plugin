use egui::{Context, Rect, load::TexturePoll};

use anyhow::{Result, anyhow};

use crate::image_source::ImageLoader;

#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct Brush<'a> {
    pub fill_texture_loader: ImageLoader<'a>,
    pub uv: Rect,
}

impl<'a> Brush<'a> {
    pub fn to_egui(self, ctx: &Context) -> Result<egui::epaint::Brush> {
        let Brush {
            fill_texture_loader,
            uv,
        } = self;
        let texture_poll = fill_texture_loader.to_texture(ctx)?;
        match texture_poll {
            TexturePoll::Pending { size: _ } => Err(anyhow!("Texture is loading")),
            TexturePoll::Ready { texture } => {
                let brush = egui::epaint::Brush {
                    fill_texture_id: texture.id,
                    uv,
                };
                Ok(brush)
            }
        }
    }
}
