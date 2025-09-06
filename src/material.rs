use crate::math::Vec3;

#[derive(Clone, Copy)]
pub struct Material {
    pub albedo: Vec3,   // color base (difuso)
    pub ks: f32,        // especular
    pub shininess: f32, // exponente n
    pub ka: f32,        // ambiente
}
