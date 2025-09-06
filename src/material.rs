use crate::math::Vec3;

/// Material “Lambert-Phong”: albedo (difuso), ka (ambiente), ks (especular), shininess
#[derive(Clone, Copy)]
pub struct Material {
    pub albedo: Vec3,   // color base (difuso)
    pub ks: f32,        // fuerza especular
    pub shininess: f32, // exponente n
    pub ka: f32,        // ambiente
}

pub mod presets {
    use super::Material;
    use crate::math::Vec3;

    pub fn light_gray() -> Material {
        Material { albedo: Vec3::new(0.75, 0.75, 0.75), ks: 0.2, shininess: 8.0, ka: 0.06 }
    }
    pub fn rubber_red() -> Material {
        Material { albedo: Vec3::new(0.6, 0.1, 0.1), ks: 0.1, shininess: 6.0, ka: 0.07 }
    }
    pub fn ivory() -> Material {
        Material { albedo: Vec3::new(0.9, 0.85, 0.75), ks: 0.3, shininess: 18.0, ka: 0.07 }
    }
    pub fn chrome() -> Material {
        Material { albedo: Vec3::new(0.7, 0.7, 0.75), ks: 0.9, shininess: 64.0, ka: 0.03 }
    }
}
