use glam::{Mat4, Vec2, Vec3, Vec3A};

/// Contains information about the game camera and lets you calculate screen positions from world positions
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Camera {
    view: Mat4,
    projection: Mat4,
    proj_view: Mat4,
    position: Vec3,
    screen_size: Vec2,
}

impl Camera {
    pub fn new(view: Mat4, projection: Mat4, position: Vec3, screen_size: Vec2) -> Self {
        let proj_view = projection * view;
        Self {
            view,
            projection,
            proj_view,
            position,
            screen_size,
        }
    }

    #[inline]
    pub fn position(&self) -> Vec3 {
        self.position
    }

    #[inline]
    pub fn local_x(&self) -> Vec3 {
        let view = &self.view;
        Vec3::new(view.x_axis.x, view.y_axis.x, view.z_axis.x)
    }

    #[inline]
    pub fn left(&self) -> Vec3 {
        -self.local_x()
    }

    #[inline]
    pub fn right(&self) -> Vec3 {
        self.local_x()
    }

    #[inline]
    pub fn local_y(&self) -> Vec3 {
        let view = &self.view;
        Vec3::new(view.x_axis.y, view.y_axis.y, view.z_axis.y)
    }

    #[inline]
    pub fn up(&self) -> Vec3 {
        self.local_y()
    }

    #[inline]
    pub fn down(&self) -> Vec3 {
        -self.local_y()
    }

    #[inline]
    pub fn local_z(&self) -> Vec3 {
        let view = &self.view;
        Vec3::new(view.x_axis.z, view.y_axis.z, view.z_axis.z)
    }

    #[inline]
    pub fn forward(&self) -> Vec3 {
        -self.local_z()
    }

    #[inline]
    pub fn back(&self) -> Vec3 {
        self.local_z()
    }

    #[inline]
    pub fn view_matrix(&self) -> Mat4 {
        self.view
    }

    #[inline]
    pub fn projection_matrix(&self) -> Mat4 {
        self.projection
    }

    /// Projection matrix * view matrix
    #[inline]
    pub fn proj_view(&self) -> Mat4 {
        self.proj_view
    }

    /// Calculate a screen position and depth value from a given world position
    pub fn world_to_screen(&self, world_position: Vec3) -> Option<(Vec2, f32)> {
        let mut ndc = self.world_to_ndc(world_position)?;
        if !(0.0..=1.0).contains(&ndc.z) {
            // Past near or far plane
            return None;
        }
        let depth = ndc.z;

        ndc.y = -ndc.y;

        let screen_position = (ndc.truncate() + Vec2::ONE) / 2.0 * self.screen_size;
        Some((screen_position.round(), depth))
    }

    pub fn world_to_ndc<V: Into<Vec3A> + From<Vec3A>>(&self, world_position: V) -> Option<V> {
        let ndc = self.proj_view.project_point3a(world_position.into());
        (!ndc.is_nan()).then_some(ndc.into())
    }
}
