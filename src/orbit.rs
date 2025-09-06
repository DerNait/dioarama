use raylib::prelude::RaylibHandle;
use crate::camera::Camera;
use crate::math::Vec3;

/// Orbit Camera (yaw, pitch, radius) con entrada de mouse
pub struct OrbitCamera {
    pub center: Vec3,
    pub radius: f32,
    pub yaw: f32,   // rad
    pub pitch: f32, // rad
    sens: f32,
    zoom_speed: f32,
}

impl OrbitCamera {
    pub fn new(center: Vec3, radius: f32, yaw: f32, pitch: f32) -> Self {
        Self { center, radius, yaw, pitch, sens: 0.008, zoom_speed: 0.15 }
    }

    pub fn update_from_input(&mut self, rl: &RaylibHandle) {
        // Arrastrar con botón izquierdo: orbit  (enum correcto)
        if rl.is_mouse_button_down(raylib::consts::MouseButton::MOUSE_BUTTON_LEFT) {
            let delta = rl.get_mouse_delta();
            self.yaw   -= delta.x as f32 * self.sens;
            self.pitch -= delta.y as f32 * self.sens;
        }
        // Clamp pitch (-89°..+89°)
        let lim = 89.0_f32.to_radians();
        if self.pitch >  lim { self.pitch =  lim; }
        if self.pitch < -lim { self.pitch = -lim; }

        // Rueda para zoom (log-smooth)
        let wheel = rl.get_mouse_wheel_move();
        if wheel.abs() > 0.0 {
            let factor = 1.0 - wheel as f32 * self.zoom_speed;
            self.radius = (self.radius * factor).clamp(0.8, 50.0);
        }
    }

    pub fn to_camera(&self, fov_y_deg: f32, aspect: f32) -> Camera {
        // Esféricas → Cartesianas (r, yaw, pitch)
        let r = self.radius;
        let cx = self.center.x;
        let cy = self.center.y;
        let cz = self.center.z;

        let eye = Vec3::new(
            cx + r * self.pitch.cos() * self.yaw.cos(),
            cy + r * self.pitch.sin(),
            cz + r * self.pitch.cos() * self.yaw.sin(),
        );
        Camera::new(eye, self.center, Vec3::new(0.0, 1.0, 0.0), fov_y_deg, aspect)
    }
}
