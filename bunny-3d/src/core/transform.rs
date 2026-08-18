use std::ops::Mul;

use glam::{Mat4, Quat, Vec3};
use windows_numerics::Matrix4x4;

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Transform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Default for Transform {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Transform {
    pub const IDENTITY: Self = Transform {
        translation: Vec3::ZERO,
        rotation: Quat::IDENTITY,
        scale: Vec3::ONE,
    };

    #[inline]
    pub const fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        Self::from_translation(Vec3::new(x, y, z))
    }

    #[inline]
    pub fn from_matrix(world_from_local: Mat4) -> Self {
        let (scale, rotation, translation) = world_from_local.to_scale_rotation_translation();
        Self {
            translation,
            rotation,
            scale,
        }
    }

    #[inline]
    pub const fn from_translation(translation: Vec3) -> Self {
        Self {
            translation,
            ..Self::IDENTITY
        }
    }

    #[inline]
    pub const fn from_rotation(rotation: Quat) -> Self {
        Self {
            rotation,
            ..Self::IDENTITY
        }
    }

    #[inline]
    pub const fn from_scale(scale: Vec3) -> Self {
        Self {
            scale,
            ..Self::IDENTITY
        }
    }

    #[inline]
    pub const fn with_translation(mut self, translation: Vec3) -> Self {
        self.translation = translation;
        self
    }

    #[inline]
    pub const fn with_rotation(mut self, rotation: Quat) -> Self {
        self.rotation = rotation;
        self
    }

    #[inline]
    pub const fn with_scale(mut self, scale: Vec3) -> Self {
        self.scale = scale;
        self
    }

    #[inline]
    pub fn to_matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.translation)
    }

    #[inline]
    pub fn to_matrix_d3d(&self) -> Matrix4x4 {
        let mat = self.to_matrix();
        let cols = mat.to_cols_array();
        Matrix4x4 {
            M11: cols[0],
            M12: cols[1],
            M13: cols[2],
            M14: cols[3],
            M21: cols[4],
            M22: cols[5],
            M23: cols[6],
            M24: cols[7],
            M31: cols[8],
            M32: cols[9],
            M33: cols[10],
            M34: cols[11],
            M41: cols[12],
            M42: cols[13],
            M43: cols[14],
            M44: cols[15],
        }
    }

    #[inline]
    pub fn rotate(&mut self, rotation: Quat) {
        self.rotation = rotation * self.rotation
    }

    #[inline]
    pub fn local_x(&self) -> Vec3 {
        self.rotation * Vec3::X
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
        self.rotation * Vec3::Y
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
        self.rotation * Vec3::Z
    }

    #[inline]
    pub fn forward(&self) -> Vec3 {
        -self.local_z()
    }

    #[inline]
    pub fn back(&self) -> Vec3 {
        self.local_z()
    }

    /// Axis must be a normalized vector
    #[inline]
    pub fn rotate_axis(&mut self, axis: Vec3, angle: f32) {
        self.rotate(Quat::from_axis_angle(axis, angle));
    }

    #[inline]
    pub fn rotate_x(&mut self, angle: f32) {
        self.rotate(Quat::from_rotation_x(angle));
    }

    #[inline]
    pub fn rotate_y(&mut self, angle: f32) {
        self.rotate(Quat::from_rotation_y(angle));
    }

    #[inline]
    pub fn rotate_z(&mut self, angle: f32) {
        self.rotate(Quat::from_rotation_z(angle));
    }

    #[inline]
    pub fn mul_transform(&self, transform: Transform) -> Self {
        let translation = self.transform_point(transform.translation);
        let rotation = self.rotation * transform.rotation;
        let scale = self.scale * transform.scale;
        Self {
            translation,
            rotation,
            scale,
        }
    }

    #[inline]
    pub fn transform_point(&self, mut point: Vec3) -> Vec3 {
        point = self.scale * point;
        point = self.rotation * point;
        point += self.translation;
        point
    }

    #[inline]
    pub fn is_finite(&self) -> bool {
        self.translation.is_finite() && self.rotation.is_finite() && self.scale.is_finite()
    }
}

impl Mul<Transform> for Transform {
    type Output = Transform;

    fn mul(self, transform: Transform) -> Self::Output {
        self.mul_transform(transform)
    }
}

impl Mul<Vec3> for Transform {
    type Output = Vec3;

    fn mul(self, value: Vec3) -> Self::Output {
        self.transform_point(value)
    }
}
