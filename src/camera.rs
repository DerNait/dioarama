use crate::math::{Vec3, Ray};

pub struct Camera {
    pub eye: Vec3,
    pub center: Vec3,
    pub up: Vec3,
    pub right: Vec3,
    pub forward: Vec3,
    pub true_up: Vec3,
    pub fov_y_deg: f32,
    pub aspect: f32,
}

impl Camera {
    pub fn new(eye: Vec3, center: Vec3, up: Vec3, fov_y_deg: f32, aspect: f32) -> Self {
        let forward = (center - eye).normalized();
        let right = forward.cross(up).normalized();    // regla mano derecha
        let true_up = right.cross(forward).normalized();
        Self { eye, center, up, right, forward, true_up, fov_y_deg, aspect }
    }

    pub fn ray_for_pixel(&self, px: i32, py: i32, w: i32, h: i32) -> Ray {
        // NDC -> [-1,1]
        let x = ((px as f32 + 0.5) / w as f32) * 2.0 - 1.0;
        let y = 1.0 - ((py as f32 + 0.5) / h as f32) * 2.0;

        let fov_scale = (self.fov_y_deg.to_radians() * 0.5).tan();
        let sx = x * self.aspect * fov_scale;
        let sy = y * fov_scale;

        let dir = (self.right * sx + self.true_up * sy + self.forward).normalized();
        Ray::new(self.eye, dir)
    }
}
