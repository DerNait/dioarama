use crate::math::Vec3;

pub struct PointLight {
    pub position: Vec3,
    pub color: Vec3,     // normalmente blanco
    pub intensity: f32,  // factor multiplicador
}
