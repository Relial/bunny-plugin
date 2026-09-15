use glam::{Mat4, Vec2, Vec3, Vec3A};
use mint::Vector2;

// https://docs.rs/bevy_camera/0.19.1/src/bevy_camera/camera.rs.html

/// Contains information about the game camera
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
    pub fn new(
        view: Mat4,
        projection: Mat4,
        position: Vec3,
        screen_size: impl Into<Vector2<f32>>,
    ) -> Self {
        let proj_view = projection * view;
        Self {
            view,
            projection,
            proj_view,
            position,
            screen_size: screen_size.into().into(),
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

    pub fn screen_to_world(
        &self,
        screen_position: impl Into<Vector2<f32>>,
        view: Mat4,
        proj: Mat4,
    ) -> Option<Ray> {
        let ndc_xy = self.screen_to_ndc(screen_position);
        let ndc_point_near = ndc_xy.extend(f32::EPSILON).into();
        let ndc_point_far = ndc_xy.extend(1.0).into();

        let view_from_clip = proj.inverse();
        let world_from_view = view.inverse();

        let view_point_near = view_from_clip.project_point3a(ndc_point_near);
        let view_point_far = view_from_clip.project_point3a(ndc_point_far);
        let view_dir = view_point_far - view_point_near;
        let origin = world_from_view.transform_point3a(view_point_near);
        let direction = world_from_view.transform_vector3a(view_dir);
        if !(direction.is_finite() && direction.length() > 0.0) {
            return None;
        }

        (!direction.is_nan()).then_some(Ray {
            origin,
            direction: direction.normalize(),
        })
    }

    pub fn screen_to_ndc(&self, screen_position: impl Into<Vector2<f32>>) -> Vec2 {
        let screen_position: Vec2 = screen_position.into().into();
        let relative = screen_position / self.screen_size;
        let mut ndc = relative * 2.0 - Vec2::ONE;
        ndc.y = -ndc.y;
        ndc
    }
}

#[derive(Debug)]
pub struct Ray {
    pub origin: Vec3A,
    pub direction: Vec3A,
}

impl Ray {
    #[inline]
    pub fn get_point(&self, distance: f32) -> Vec3 {
        (self.origin + self.direction * distance).into()
    }

    #[inline]
    pub fn intersect_plane(&self, plane_origin: Vec3, plane: InfinitePlane) -> Option<f32> {
        let denominator = plane.normal.dot(self.direction.to_vec3());
        if denominator.abs() > f32::EPSILON {
            let distance = (plane_origin - self.origin.to_vec3()).dot(plane.normal) / denominator;
            if distance > f32::EPSILON {
                return Some(distance);
            }
        }
        None
    }

    #[inline]
    pub fn plane_intersection_point(
        &self,
        plane_origin: Vec3,
        plane: InfinitePlane,
    ) -> Option<Vec3> {
        self.intersect_plane(plane_origin, plane)
            .map(|distance| self.get_point(distance))
    }

    pub fn sphere_intersection_at(
        &self,
        sphere: &BoundingSphere,
        max_distance: f32,
    ) -> Option<f32> {
        let offset = self.origin - sphere.center;
        let projected = offset.dot(self.direction);
        let closest_point = offset - projected * self.direction;
        let distance_squared = sphere.radius * sphere.radius - closest_point.length_squared();
        if distance_squared < 0.0
            || (projected * projected).copysign(-projected) < -distance_squared
        {
            None
        } else {
            let toi = -projected - distance_squared.sqrt();
            if toi > max_distance {
                None
            } else {
                Some(toi.max(0.0))
            }
        }
    }
}

pub struct BoundingSphere {
    pub center: Vec3A,
    pub radius: f32,
}

impl BoundingSphere {
    pub fn new(center: impl Into<Vec3A>, radius: f32) -> Self {
        Self {
            center: center.into(),
            radius,
        }
    }
}

pub struct InfinitePlane {
    pub normal: Vec3,
}

impl InfinitePlane {
    #[inline]
    pub fn new<T: Into<Vec3>>(normal: T) -> Self {
        Self {
            normal: normal.into().normalize(),
        }
    }

    #[inline]
    pub fn from_points(a: Vec3, b: Vec3, c: Vec3) -> (Self, Vec3) {
        let normal = (b - a).cross(c - a).normalize();
        let translation = (a + b + c) / 3.0;
        (Self { normal }, translation)
    }
}
