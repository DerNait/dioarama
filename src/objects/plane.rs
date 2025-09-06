use crate::material::Material;
use crate::math::{Vec3, Ray};
use crate::object::{Hittable, HitRecord};

/// Plano infinito (punto + normal)
pub struct Plane {
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

impl Plane {
    pub fn new(point: Vec3, normal: Vec3, material: Material) -> Self {
        Self { point, normal: normal.normalized(), material }
    }
}

impl Hittable for Plane {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let denom = self.normal.dot(ray.dir);
        if denom.abs() < 1e-6 { return None; } // paralelo

        let t = (self.point - ray.origin).dot(self.normal) / denom;
        if t < t_min || t > t_max { return None; }

        let p = ray.at(t);
        let n = if denom < 0.0 { self.normal } else { -self.normal };
        Some(HitRecord { t, point: p, normal: n, material: self.material })
    }
}
