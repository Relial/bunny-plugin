use egui::Rect;

use crate::image_source::ImageSource;

#[derive(Clone, Debug, PartialEq)]
#[repr(C)]
pub struct Brush<'a> {
    pub fill_texture_source: ImageSource<'a>,
    pub uv: Rect,
}

#[cfg(feature = "manager")]
impl<'a> Brush<'a> {
    pub fn to_egui(self, ctx: &egui::Context) -> anyhow::Result<egui::epaint::Brush> {
        let Brush {
            fill_texture_source,
            uv,
        } = self;
        let texture_poll = fill_texture_source.get_texture(ctx)?;
        match texture_poll {
            egui::load::TexturePoll::Pending { size: _ } => {
                Err(anyhow::anyhow!("Texture is loading"))
            }
            egui::load::TexturePoll::Ready { texture } => {
                let brush = egui::epaint::Brush {
                    fill_texture_id: texture.id,
                    uv,
                };
                Ok(brush)
            }
        }
    }
}
