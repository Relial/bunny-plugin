use egui::{Rect, Vec2};
use std::{mem::ManuallyDrop, ptr::NonNull, sync::Arc};

/// This is an FFI safe wrapper around Arc<egui::Galley>, so cloning is cheap.
#[derive(Clone)]
#[repr(C)]
pub struct BunnyGalley {
    inner: GalleyInner,
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
            inner: GalleyInner::new(galley),
            rect,
            num_vertices,
            num_indices,
            pixels_per_point,
        }
    }

    #[inline]
    pub fn into_inner(self) -> Arc<egui::Galley> {
        self.inner.into_inner()
    }
}

impl BunnyGalley {
    #[inline]
    pub fn size(&self) -> Vec2 {
        self.rect.size()
    }
}

type ArcGalleyClone = unsafe extern "C" fn(&GalleyInner);
type ArcGalleyDrop = unsafe extern "C" fn(&mut GalleyInner);

#[repr(C)]
struct GalleyInner {
    arc_ptr: NonNull<u8>,
    clone: ArcGalleyClone,
    drop: ArcGalleyDrop,
}

impl GalleyInner {
    #[inline]
    fn new(galley: Arc<egui::Galley>) -> Self {
        unsafe extern "C" fn clone(galley: &GalleyInner) {
            let ptr = galley.arc_ptr.cast::<egui::Galley>().as_ptr();
            unsafe {
                Arc::increment_strong_count(ptr);
            }
        }
        unsafe extern "C" fn drop(galley: &mut GalleyInner) {
            let ptr = galley.arc_ptr.cast::<egui::Galley>().as_ptr();
            let _arc = unsafe { Arc::from_raw(ptr) };
        }

        let arc_ptr = unsafe { NonNull::new_unchecked(Arc::into_raw(galley) as *mut u8) };
        Self {
            arc_ptr,
            clone,
            drop,
        }
    }

    #[inline]
    fn into_inner(self) -> Arc<egui::Galley> {
        let this = ManuallyDrop::new(self);
        let ptr = this.arc_ptr.cast::<egui::Galley>().as_ptr();
        unsafe { Arc::from_raw(ptr) }
    }
}

impl Clone for GalleyInner {
    fn clone(&self) -> Self {
        unsafe { (self.clone)(self) };
        Self {
            arc_ptr: self.arc_ptr,
            clone: self.clone,
            drop: self.drop,
        }
    }
}

impl Drop for GalleyInner {
    fn drop(&mut self) {
        unsafe { (self.drop)(self) };
    }
}
