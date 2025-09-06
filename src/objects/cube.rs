use crate::material::Material;
use crate::math::{Vec3, Ray, EPS, INF};
use crate::object::{Hittable, HitRecord};

/// Cubo AABB con centro y semiejes (half extents)
pub struct Cube {
    pub center: Vec3,
    pub half: Vec3,
    pub material: Material,
}

impl Cube {
    pub fn new(center: Vec3, half_extents: Vec3, material: Material) -> Self {
        Self { center, half: half_extents, material }
    }
}

impl Hittable for Cube {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        // límites
        let min_b = self.center - self.half;
        let max_b = self.center + self.half;

        // Método de slabs por eje
        let mut tmin = t_min;
        let mut tmax = t_max;

        // X
        let inv_dx = if ray.dir.x.abs() < EPS { INF } else { 1.0 / ray.dir.x };
        let mut tx1 = (min_b.x - ray.origin.x) * inv_dx;
        let mut tx2 = (max_b.x - ray.origin.x) * inv_dx;
        if tx1 > tx2 { std::mem::swap(&mut tx1, &mut tx2); }
        tmin = tmin.max(tx1);
        tmax = tmax.min(tx2);
        if tmax < tmin { return None; }

        // Y
        let inv_dy = if ray.dir.y.abs() < EPS { INF } else { 1.0 / ray.dir.y };
        let mut ty1 = (min_b.y - ray.origin.y) * inv_dy;
        let mut ty2 = (max_b.y - ray.origin.y) * inv_dy;
        if ty1 > ty2 { std::mem::swap(&mut ty1, &mut ty2); }
        tmin = tmin.max(ty1);
        tmax = tmax.min(ty2);
        if tmax < tmin { return None; }

        // Z
        let inv_dz = if ray.dir.z.abs() < EPS { INF } else { 1.0 / ray.dir.z };
        let mut tz1 = (min_b.z - ray.origin.z) * inv_dz;
        let mut tz2 = (max_b.z - ray.origin.z) * inv_dz;
        if tz1 > tz2 { std::mem::swap(&mut tz1, &mut tz2); }
        tmin = tmin.max(tz1);
        tmax = tmax.min(tz2);
        if tmax < tmin { return None; }

        // Tomamos el primer cruce válido
        let t_hit = if tmin > t_min { tmin } else { tmax };
        if t_hit < t_min || t_hit > t_max { return None; }

        let p = ray.at(t_hit);

        // Normal según la cara tocada
        let mut n = Vec3::new(0.0, 0.0, 0.0);
        let bias = 1e-3;
        if (p.x - max_b.x).abs() < bias { n = Vec3::new( 1.0, 0.0, 0.0); }
        else if (p.x - min_b.x).abs() < bias { n = Vec3::new(-1.0, 0.0, 0.0); }
        else if (p.y - max_b.y).abs() < bias { n = Vec3::new(0.0,  1.0, 0.0); }
        else if (p.y - min_b.y).abs() < bias { n = Vec3::new(0.0, -1.0, 0.0); }
        else if (p.z - max_b.z).abs() < bias { n = Vec3::new(0.0, 0.0,  1.0); }
        else if (p.z - min_b.z).abs() < bias { n = Vec3::new(0.0, 0.0, -1.0); }

        Some(HitRecord { t: t_hit, point: p, normal: n, material: self.material })
    }
}
