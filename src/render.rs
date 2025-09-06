use crate::camera::Camera;
use crate::light::PointLight;
use crate::math::{Vec3, Ray, EPS, INF};
use crate::object::{Hittable, HitRecord};

fn background(dir: Vec3) -> Vec3 {
    // gradiente simple cielo
    let t = 0.5 * (dir.y + 1.0);
    let c1 = Vec3::new(0.6, 0.8, 1.0);
    let c2 = Vec3::new(0.05, 0.05, 0.08);
    c2 * (1.0 - t) + c1 * t
}

// Phong: ambiente + difuso + especular
fn shade(hit: &HitRecord, cam_pos: Vec3, light: &PointLight, world: &Vec<Box<dyn Hittable + Send + Sync>>) -> Vec3 {
    let ambient = hit.material.albedo * hit.material.ka;

    // dirección y distancia a la luz
    let to_light = (light.position - hit.point);
    let dist_to_light = to_light.length();
    let l_dir = to_light / dist_to_light;

    // Shadow ray (offset para evitar acne)
    let shadow_origin = hit.point + hit.normal * EPS * 8.0;
    let shadow_ray = Ray::new(shadow_origin, l_dir);
    let mut in_shadow = false;

    for obj in world.iter() {
        if let Some(h) = obj.intersect(&shadow_ray, EPS, dist_to_light - EPS) {
            if h.t > 0.0 { in_shadow = true; break; }
        }
    }

    if in_shadow {
        return ambient; // solo componente ambiente
    }

    // Difuso
    let ndotl = hit.normal.dot(l_dir).max(0.0);
    let diffuse = hit.material.albedo * ndotl * light.intensity;

    // Especular
    let v = (cam_pos - hit.point).normalized();
    let r = Vec3::reflect(-l_dir, hit.normal).normalized();
    let spec = r.dot(v).max(0.0).powf(hit.material.shininess);
    let specular = Vec3::splat(hit.material.ks * spec * light.intensity);

    // Luz (asumimos color blanco)
    (ambient + diffuse + specular).hadamard(light.color)
}

pub fn trace_pixel(
    x: i32, y: i32, w: i32, h: i32,
    camera: &Camera,
    world: &Vec<Box<dyn Hittable + Send + Sync>>,
    light: &PointLight
) -> Vec3 {
    let ray = camera.ray_for_pixel(x, y, w, h);
    let mut closest_t = INF;
    let mut closest_hit: Option<HitRecord> = None;

    for obj in world.iter() {
        if let Some(hit) = obj.intersect(&ray, 0.001, closest_t) {
            if hit.t < closest_t {
                closest_t = hit.t;
                closest_hit = Some(hit);
            }
        }
    }

    if let Some(hit) = closest_hit {
        return shade(&hit, camera.eye, light, world);
    }

    background(ray.dir)
}
