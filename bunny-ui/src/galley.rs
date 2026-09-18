use egui::{Rect, Vec2};
use std::{ffi::c_void, sync::Arc};

#[repr(C)]
pub struct BunnyGalley {
    inner: *const c_void,
    pub rect: Rect,
    pub num_vertices: usize,
    pub num_indices: usize,
    pub pixels_per_point: f32,
}

#[cfg(feature = "manager")]
impl BunnyGalley {
    #[inline]
    pub fn new(galley: Arc<egui::Galley>) -> Self {
        let rect = galley.rect;
        let num_vertices = galley.num_vertices;
        let num_indices = galley.num_indices;
        let pixels_per_point = galley.pixels_per_point;
        Self {
            inner: Arc::into_raw(galley) as *const c_void,
            rect,
            num_vertices,
            num_indices,
            pixels_per_point,
        }
    }

    #[inline]
    pub fn into_inner(self) -> Arc<egui::Galley> {
        unsafe { Arc::from_raw(self.inner as *const egui::Galley) }
    }
}

impl BunnyGalley {
    #[inline]
    pub fn size(&self) -> Vec2 {
        self.rect.size()
    }
}
