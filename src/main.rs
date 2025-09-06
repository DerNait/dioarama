use raylib::prelude::*;

mod math;
mod camera;
mod orbit;
mod light;
mod material;
mod object;
mod render;
mod objects;

use camera::Camera;
use light::PointLight;
use material::{Material, presets};
use math::{Vec3, clamp01};
use object::Hittable;
use objects::{cube::Cube, plane::Plane};
use orbit::OrbitCamera;
use render::trace_pixel;

fn main() {
    const WIDTH: i32 = 500;
    const HEIGHT: i32 = 400;
    const SCALE: i32 = 2;

    let (mut rl, thread) = raylib::init()
        .size(WIDTH * SCALE, HEIGHT * SCALE)
        .title("Ray Tracing (CPU+raylib) - Cube • Materials • ZBuffer • OrbitCam")
        .build();

    rl.set_target_fps(30);

    // ------- Orbit Camera -------
    let mut orbit = OrbitCamera::new(
        Vec3::new(0.0, 1.0, 0.0), // center
        4.0,                      // radius
        0.0_f32.to_radians(),     // yaw
        12.0_f32.to_radians(),    // pitch
    );
    let fov_y_deg = 60.0;
    let aspect = WIDTH as f32 / HEIGHT as f32;

    // ------- Luz -------
    let light = PointLight {
        position: Vec3::new(3.0, 5.0, 2.0),
        color: Vec3::splat(1.0),
        intensity: 1.5,
    };

    // ------- Materiales -------
    let mat_cube = Material {
        albedo: Vec3::new(0.82, 0.18, 0.18),
        ks: 0.45,
        shininess: 32.0,
        ka: 0.08,
    };
    let mat_floor = presets::light_gray();

    // ------- Objetos -------
    let cube = Cube::new(Vec3::new(0.0, 1.0, 0.0), Vec3::splat(0.75), mat_cube);
    let floor = Plane::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), mat_floor);

    let mut world: Vec<Box<dyn Hittable + Send + Sync>> = Vec::new();
    world.push(Box::new(cube));
    world.push(Box::new(floor));

    // ------- ZBuffer -------
    let mut zbuffer: Vec<f32> = vec![math::INF; (WIDTH * HEIGHT) as usize];

    while !rl.window_should_close() {
        // Input orbit
        orbit.update_from_input(&rl);

        // ✅ Generar cámara para el frame (línea corregida)
        let camera: Camera = orbit.to_camera(fov_y_deg, aspect);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        // Limpia zbuffer
        for z in &mut zbuffer { *z = math::INF; }

        // Ray cast por píxel
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let idx = (y * WIDTH + x) as usize;
                let color_lin = trace_pixel(
                    x, y, WIDTH, HEIGHT,
                    &camera, &world, &light,
                    &mut zbuffer, idx
                );

                // gamma-correct (sRGB aprox): out = pow(color, 1/2.2)
                let gamma = 1.0 / 2.2;
                let color = Vec3::new(
                    color_lin.x.powf(gamma),
                    color_lin.y.powf(gamma),
                    color_lin.z.powf(gamma),
                );

                let r = (clamp01(color.x) * 255.0) as u8;
                let g = (clamp01(color.y) * 255.0) as u8;
                let b = (clamp01(color.z) * 255.0) as u8;
                d.draw_rectangle(x * SCALE, y * SCALE, SCALE, SCALE, Color::new(r, g, b, 255));
            }
        }

        // HUD
        d.draw_text("Orbit: arrastra con el mouse • Zoom: rueda", 10, 10, 18, Color::WHITE);
        d.draw_text(
            &format!("yaw {:.1}°  pitch {:.1}°  radius {:.2}",
                orbit.yaw.to_degrees(), orbit.pitch.to_degrees(), orbit.radius),
            10, 32, 18, Color::WHITE,
        );
        d.draw_fps(10, 54);
    }
}
