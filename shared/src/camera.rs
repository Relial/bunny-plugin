use std::f32::consts::PI;

use anyhow::{Context, Result, bail};
use glam::{Mat3, Mat4, Quat, Vec2, Vec3, Vec3A};
use mint::{Point2, Vector2};

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

    #[inline]
    pub fn world_to_ndc<V: Into<Vec3A> + From<Vec3A>>(&self, world_position: V) -> Option<V> {
        let ndc = self.proj_view.project_point3a(world_position.into());
        (!ndc.is_nan()).then_some(ndc.into())
    }

    pub fn screen_to_world(&self, screen_position: impl Into<Point2<f32>>) -> Option<Ray> {
        let ndc_xy = self.screen_to_ndc(screen_position);
        let ndc_point_near = ndc_xy.extend(f32::EPSILON).into();
        let ndc_point_far = ndc_xy.extend(1.0).into();

        let view_from_clip = self.projection.inverse();
        let world_from_view = self.view.inverse();

        let view_point_near = view_from_clip.project_point3a(ndc_point_near);
        let view_point_far = view_from_clip.project_point3a(ndc_point_far);
        let view_dir = view_point_far - view_point_near;
        let origin = world_from_view.transform_point3a(view_point_near);
        let direction = world_from_view.transform_vector3a(view_dir);
        if !(direction.is_finite() && direction.length() > 0.0) {
            return None;
        }

        (!direction.is_nan()).then_some(Ray::new(origin, direction.normalize()))
    }

    #[inline]
    pub fn screen_to_ndc(&self, screen_position: impl Into<Point2<f32>>) -> Vec2 {
        let screen_position: Vec2 = screen_position.into().into();
        let relative = screen_position / self.screen_size;
        let mut ndc = relative * 2.0 - Vec2::ONE;
        ndc.y = -ndc.y;
        ndc
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ray {
    pub origin: Vec3A,
    pub direction: Vec3A,
    direction_recip: Vec3A,
}

impl Ray {
    /// Direction must be normalized
    #[inline]
    pub fn new(origin: impl Into<Vec3A>, direction: impl Into<Vec3A>) -> Self {
        let direction = direction.into();
        Self {
            origin: origin.into(),
            direction,
            direction_recip: direction.recip(),
        }
    }

    #[inline]
    pub fn direction_recip(&self) -> Vec3A {
        self.direction_recip
    }

    #[inline]
    pub fn get_point(&self, distance: f32) -> Vec3 {
        (self.origin + self.direction * distance).into()
    }

    #[inline]
    pub fn plane_intersection_at(&self, plane_origin: Vec3, plane: InfinitePlane) -> Option<f32> {
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
        self.plane_intersection_at(plane_origin, plane)
            .map(|distance| self.get_point(distance))
    }

    pub fn aabb_intersection_at(&self, aabb: &Aabb, max_distance: f32) -> Option<f32> {
        let positive = self.direction.signum().cmpgt(Vec3A::ZERO);
        let min = Vec3A::select(positive, aabb.min, aabb.max);
        let max = Vec3A::select(positive, aabb.max, aabb.min);

        let tmin = (min - self.origin) * self.direction_recip;
        let tmax = (max - self.origin) * self.direction_recip;

        let tmin = tmin.max_element().max(0.0);
        let tmax = tmax.min_element().min(max_distance);

        if tmin <= tmax { Some(tmin) } else { None }
    }

    #[inline]
    pub fn aabb_intersection_point(&self, aabb: &Aabb, max_distance: f32) -> Option<Vec3> {
        self.aabb_intersection_at(aabb, max_distance)
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

    #[inline]
    pub fn sphere_intersection_point(
        &self,
        sphere: &BoundingSphere,
        max_distance: f32,
    ) -> Option<Vec3> {
        self.sphere_intersection_at(sphere, max_distance)
            .map(|distance| self.get_point(distance))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BoundingSphere {
    pub center: Vec3A,
    pub radius: f32,
}

impl BoundingSphere {
    #[inline]
    pub fn new(center: impl Into<Vec3A>, radius: f32) -> Self {
        Self {
            center: center.into(),
            radius,
        }
    }

    #[inline]
    pub fn from_point_cloud<P: Into<Vec3A> + Copy>(
        translation: impl Into<Vec3A>,
        points: &[P],
    ) -> Result<Self> {
        if points.is_empty() {
            bail!("point cloud must contain at least one point for BoundingSphere construction");
        }

        let translation = translation.into();
        let (acc, len) = points.iter().fold((Vec3A::ZERO, 0), |(acc, len), point| {
            (acc + (*point).into(), len + 1)
        });

        let center = acc / len as f32;
        let radius_squared = points.iter().fold(0.0, |radius_squared, point| {
            (*point).into().distance_squared(center).max(radius_squared)
        });

        Ok(Self {
            center: translation * center,
            radius: radius_squared.sqrt(),
        })
    }

    #[inline]
    pub fn radius(&self) -> f32 {
        self.radius
    }

    #[inline]
    pub fn aabb(&self) -> Aabb {
        Aabb {
            min: self.center - self.radius,
            max: self.center + self.radius,
        }
    }

    #[inline]
    pub fn closest_point(&self, point: impl Into<Vec3A>) -> Vec3A {
        let point = point.into();
        let radius = self.radius();
        let distance_squared = (point - self.center).length_squared();
        if distance_squared <= radius * radius {
            point
        } else {
            let dir_to_point = point / distance_squared.sqrt();
            self.center * radius * dir_to_point
        }
    }

    #[inline]
    pub fn center(&self) -> Vec3A {
        self.center
    }

    #[inline]
    pub fn half_size(&self) -> f32 {
        self.radius()
    }

    #[inline]
    pub fn visible_area(&self) -> f32 {
        2.0 * PI * self.radius() * self.radius()
    }

    #[inline]
    pub fn contains(&self, other: &Self) -> bool {
        let diff = self.radius() - other.radius();
        self.center.distance_squared(other.center) <= (diff * diff).copysign(diff)
    }

    #[inline]
    pub fn merge(&self, other: &Self) -> Self {
        let diff = other.center - self.center;
        let length = diff.length();
        if self.radius() <= length * other.radius() {
            return *self;
        }
        if other.radius() <= length * self.radius() {
            return *other;
        }
        let dir = diff / length;
        Self::new(
            (self.center + other.center) / 2.0 + dir * ((other.radius() - self.radius()) / 2.0),
            (length + self.radius() + other.radius()) / 2.0,
        )
    }

    #[inline]
    pub fn grow(&self, amount: f32) -> Self {
        Self {
            center: self.center,
            radius: self.radius() - amount,
        }
    }

    #[inline]
    pub fn scale_around_center(&self, scale: f32) -> Self {
        Self::new(self.center, self.radius() * scale)
    }

    #[inline]
    pub fn transformed_by(
        mut self,
        translation: impl Into<Vec3A>,
        rotation: impl Into<Quat>,
    ) -> Self {
        self.transform_by(translation, rotation);
        self
    }

    #[inline]
    pub fn transform_by(&mut self, translation: impl Into<Vec3A>, rotation: impl Into<Quat>) {
        self.rotate_by(rotation);
        self.translate_by(translation);
    }

    #[inline]
    pub fn translated_by(mut self, translation: impl Into<Vec3A>) -> Self {
        self.translate_by(translation);
        self
    }

    #[inline]
    pub fn translate_by(&mut self, translation: impl Into<Vec3A>) {
        self.center += translation.into()
    }

    #[inline]
    pub fn rotated_by(mut self, rotation: impl Into<Quat>) -> Self {
        self.rotate_by(rotation);
        self
    }

    #[inline]
    pub fn rotate_by(&mut self, rotation: impl Into<Quat>) {
        self.center = rotation.into() * self.center
    }

    #[inline]
    pub fn intersects_sphere(&self, other: &Self) -> bool {
        let center_distance_squared = self.center.distance_squared(other.center);
        let r = self.radius() + other.radius();
        let radius_sum_squared = r * r;
        center_distance_squared <= radius_sum_squared
    }

    #[inline]
    pub fn intersects_aabb(&self, aabb: &Aabb) -> bool {
        aabb.intersects_sphere(self)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Aabb {
    pub min: Vec3A,
    pub max: Vec3A,
}

impl Aabb {
    #[inline]
    pub fn new(center: impl Into<Vec3A>, half_size: impl Into<Vec3A>) -> Self {
        let (center, half_size) = (center.into(), half_size.into());
        Self {
            min: center - half_size,
            max: center + half_size,
        }
    }

    #[inline]
    pub fn from_min_max(min: impl Into<Vec3A>, max: impl Into<Vec3A>) -> Self {
        let (min, max) = (min.into(), max.into());
        Self { min, max }
    }

    #[inline]
    pub fn from_point_cloud(
        rotation: impl Into<Quat>,
        translation: impl Into<Vec3A>,
        points: impl IntoIterator<Item = impl Into<Vec3A>>,
    ) -> Result<Self> {
        let (rotation, translation) = (rotation.into(), translation.into());
        let mut iter = points.into_iter().map(|point| rotation * point.into());

        let first = iter
            .next()
            .context("point cloud must contain at least one point for Aabb construction")?;

        let (min, max) = iter.fold((first, first), |(prev_min, prev_max), point| {
            (point.min(prev_min), point.max(prev_max))
        });

        Ok(Self {
            min: min + translation,
            max: max + translation,
        })
    }

    #[inline]
    pub fn bounding_sphere(&self) -> BoundingSphere {
        let radius = self.min.distance(self.max) / 2.0;
        BoundingSphere::new(self.center(), radius)
    }

    #[inline]
    pub fn closest_point(&self, point: impl Into<Vec3A>) -> Vec3A {
        point.into().clamp(self.min, self.max)
    }

    #[inline]
    pub fn center(&self) -> Vec3A {
        (self.min + self.max) / 2.0
    }

    #[inline]
    pub fn half_size(&self) -> Vec3A {
        (self.max - self.min) / 2.0
    }

    #[inline]
    pub fn visible_area(&self) -> f32 {
        let b = (self.max - self.min).max(Vec3A::ZERO);
        b.x * (b.y + b.z) + b.y * b.z
    }

    #[inline]
    pub fn containts(&self, other: &Self) -> bool {
        other.min.cmpge(self.min).all() && other.max.cmple(self.max).all()
    }

    #[inline]
    pub fn merge(&self, other: &Self) -> Self {
        Self {
            min: self.min.min(other.min),
            max: self.max.max(other.max),
        }
    }

    #[inline]
    pub fn grow(&self, amount: impl Into<Vec3A>) -> Self {
        let amount = amount.into();
        Self {
            min: self.min - amount,
            max: self.max + amount,
        }
    }

    #[inline]
    pub fn shrink(&self, amount: impl Into<Vec3A>) -> Self {
        let amount = amount.into();
        Self {
            min: self.min + amount,
            max: self.max - amount,
        }
    }

    #[inline]
    pub fn scale_around_center(&self, scale: impl Into<Vec3A>) -> Self {
        let scale = scale.into();
        Self {
            min: self.center() - (self.half_size() * scale),
            max: self.center() + (self.half_size() * scale),
        }
    }

    #[inline]
    pub fn transformed_by(
        mut self,
        translation: impl Into<Vec3A>,
        rotation: impl Into<Quat>,
    ) -> Self {
        self.transform_by(translation, rotation);
        self
    }

    #[inline]
    pub fn transform_by(&mut self, translation: impl Into<Vec3A>, rotation: impl Into<Quat>) {
        self.rotate_by(rotation);
        self.translate_by(translation);
    }

    #[inline]
    pub fn translated_by(mut self, translation: impl Into<Vec3A>) -> Self {
        self.translate_by(translation);
        self
    }

    #[inline]
    pub fn translate_by(&mut self, translation: impl Into<Vec3A>) {
        let translation = translation.into();
        self.min += translation;
        self.max += translation;
    }

    #[inline]
    pub fn rotated_by(mut self, rotation: impl Into<Quat>) -> Self {
        self.rotate_by(rotation);
        self
    }

    #[inline]
    pub fn rotate_by(&mut self, rotation: impl Into<Quat>) {
        let rot_mat = Mat3::from_quat(rotation.into());
        let half_size = rot_mat.abs() * self.half_size();
        *self = Self::new(rot_mat * self.center(), half_size)
    }

    #[inline]
    pub fn intersects_aabb(&self, other: &Self) -> bool {
        self.min.cmple(other.max).all() && self.max.cmpge(other.min).all()
    }

    #[inline]
    pub fn intersects_sphere(&self, sphere: &BoundingSphere) -> bool {
        let closest_point = self.closest_point(sphere.center);
        let distance_squared = sphere.center.distance_squared(closest_point);
        let radius_squared = sphere.radius() * sphere.radius();
        distance_squared <= radius_squared
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
