use crate::material::Material;
use crate::math::{Ray, Vec3};

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub t: f32,
    pub point: Vec3,
    pub normal: Vec3,   // normalizada, saliendo del objeto
    pub material: Material,
}

pub trait Hittable {
    /// Intersección del rayo en [t_min, t_max]. Devuelve el hit más cercano.
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
