use std::ops::{Add, Sub, Mul, Div, Neg};

pub const EPS: f32 = 1e-4;
pub const INF: f32 = 1e30;

#[derive(Copy, Clone, Debug, Default)]
pub struct Vec3 {
    pub x: f32, pub y: f32, pub z: f32
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self { Self { x, y, z } }
    pub fn splat(v: f32) -> Self { Self { x: v, y: v, z: v } }

    pub fn dot(self, o: Self) -> f32 { self.x * o.x + self.y * o.y + self.z * o.z }
    pub fn cross(self, o: Self) -> Self {
        Self::new(
            self.y * o.z - self.z * o.y,
            self.z * o.x - self.x * o.z,
            self.x * o.y - self.y * o.x,
        )
    }
    pub fn length(self) -> f32 { self.dot(self).sqrt() }
    pub fn normalized(self) -> Self {
        let len = self.length();
        if len < EPS { self } else { self / len }
    }
    pub fn hadamard(self, o: Self) -> Self { Self::new(self.x*o.x, self.y*o.y, self.z*o.z) }
    pub fn reflect(i: Self, n: Self) -> Self { i - n * (2.0 * i.dot(n)) }
    pub fn min(self, o: Self) -> Self { Self::new(self.x.min(o.x), self.y.min(o.y), self.z.min(o.z)) }
    pub fn max(self, o: Self) -> Self { Self::new(self.x.max(o.x), self.y.max(o.y), self.z.max(o.z)) }
    pub fn abs(self) -> Self { Self::new(self.x.abs(), self.y.abs(), self.z.abs()) }
}

impl Add for Vec3 { type Output = Self; fn add(self, o: Self) -> Self { Self::new(self.x+o.x, self.y+o.y, self.z+o.z) } }
impl Sub for Vec3 { type Output = Self; fn sub(self, o: Self) -> Self { Self::new(self.x-o.x, self.y-o.y, self.z-o.z) } }
impl Mul<f32> for Vec3 { type Output = Self; fn mul(self, s: f32) -> Self { Self::new(self.x*s, self.y*s, self.z*s) } }
impl Div<f32> for Vec3 { type Output = Self; fn div(self, s: f32) -> Self { Self::new(self.x/s, self.y/s, self.z/s) } }
impl Neg for Vec3 { type Output = Self; fn neg(self) -> Self { Self::new(-self.x, -self.y, -self.z) } }

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3, // normalizado
}
impl Ray {
    pub fn new(origin: Vec3, dir: Vec3) -> Self { Self { origin, dir: dir.normalized() } }
    pub fn at(&self, t: f32) -> Vec3 { self.origin + self.dir * t }
}

pub fn clamp01(v: f32) -> f32 { if v < 0.0 { 0.0 } else if v > 1.0 { 1.0 } else { v } }
