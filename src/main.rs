use raylib::prelude::*;

mod math;
mod camera;
mod light;
mod material;
mod object;
mod render;
mod objects;

use camera::Camera;
use light::PointLight;
use material::Material;
use math::{Vec3, clamp01};
use object::Hittable;
use objects::{cube::Cube, plane::Plane};
use render::trace_pixel;

fn main() {
    // Resolución "virtual" pequeña + escalado para dibujar rápido
    const WIDTH: i32 = 240;
    const HEIGHT: i32 = 160;
    const SCALE: i32 = 4;

    let (mut rl, thread) = raylib::init()
        .size(WIDTH * SCALE, HEIGHT * SCALE)
        .title("Ray Tracing (CPU+raylib) - Cube + Shadows + Phong")
        .build();

    rl.set_target_fps(60);

    // Cámara
    let eye    = Vec3::new(0.0, 1.0, 4.0);
    let center = Vec3::new(0.0, 1.0, 0.0);
    let up     = Vec3::new(0.0, 1.0, 0.0);
    let fov_y_deg = 60.0;
    let aspect = WIDTH as f32 / HEIGHT as f32;

    let camera = Camera::new(eye, center, up, fov_y_deg, aspect);

    // Luz puntual
    let light = PointLight {
        position: Vec3::new(3.0, 5.0, 2.0),
        color: Vec3::splat(1.0),
        intensity: 1.5,
    };

    // Materiales simples (albedo + ks + shininess + ka)
    let mat_cube = Material {
        albedo: Vec3::new(0.80, 0.15, 0.15), // rojo
        ks: 0.5,
        shininess: 32.0,
        ka: 0.08, // ambiente
    };

    let mat_floor = Material {
        albedo: Vec3::new(0.75, 0.75, 0.75), // gris claro
        ks: 0.2,
        shininess: 8.0,
        ka: 0.06,
    };

    // Escena: cubo y plano
    let cube = Cube::new(Vec3::new(0.0, 1.0, 0.0), Vec3::splat(0.75), mat_cube);
    let floor = Plane::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), mat_floor);

    // Guardamos como trait objects
    let mut world: Vec<Box<dyn Hittable + Send + Sync>> = Vec::new();
    world.push(Box::new(cube));
    world.push(Box::new(floor));

    // Render por CPU cada frame (rápido por la res + escala)
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let color = trace_pixel(x, y, WIDTH, HEIGHT, &camera, &world, &light);
                // convertir [0,1] a u8
                let r = (clamp01(color.x) * 255.0) as u8;
                let g = (clamp01(color.y) * 255.0) as u8;
                let b = (clamp01(color.z) * 255.0) as u8;
                d.draw_rectangle(x * SCALE, y * SCALE, SCALE, SCALE, Color::new(r, g, b, 255));
            }
        }

        // info
        d.draw_text("Cubo, luz puntual, sombras duras, Phong", 10, 10, 20, Color::WHITE);
        d.draw_fps(10, 34);
    }
}
