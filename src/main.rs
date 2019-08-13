extern crate minifb;
extern crate rand;

mod vec3;
mod ray;
mod hitable;
mod sphere;
mod camera;

use minifb::{Key, WindowOptions, Window};
use rand::Rng;
use vec3::Vec3;
use ray::Ray;
use hitable::{HitableList, Hitable};
use camera::Camera;

const WIDTH: usize = 640;
const HEIGHT: usize = 320;
const NS: usize = 32;

fn to_buffer_index(i: usize, j: usize, width: usize, height: usize) -> usize {
    ((height - 1 - i) * width) + j
}

fn to_bgra(r: u32, g: u32, b: u32) -> u32 {
    255 << 24 | r << 16 | g << 8 | b
}

fn rand_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = Vec3::new(rng.gen(), rng.gen(), rng.gen())*2.0 - Vec3::new(0.5, 0.5, 0.5);
        if p.squared_len() >= 1.0 {
            break p;
        } 
    }
}

fn color(r: &Ray, world: &HitableList) -> Vec3 {
    if let Some(hit) = world.hit(&r, 0.001, 1000000.0) {
        let target = hit.p + hit.normal + rand_in_unit_sphere();
        return color(&Ray::new(hit.p, target-hit.p), &world)*0.5
    }
    else {
        let unit_dir = vec3::unit_vec3(r.direction);
        let t = 0.5*(unit_dir.y+1.0);
        Vec3::new(1.0, 1.0, 1.0)*(1.0-t) + Vec3::new(0.5, 0.7, 1.0)*t
    }
}

fn main() {

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new("Rayt_Rust - ESC to exit",
                                 WIDTH,
                                 HEIGHT,
                                 WindowOptions::default()).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let cam = Camera::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(-1.6, -0.8, -1.0), 
        Vec3::new(3.2, 0.0, 0.0), 
        Vec3::new(0.0, 1.6, 0.0),
        );

    let spheres = vec![
        sphere::Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.25),
        sphere::Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0),
    ];

    let list = hitable::HitableList::new(spheres);
    let mut rng = rand::thread_rng();

    for j in 0..WIDTH {
        for i in 0..HEIGHT {

            let mut col = Vec3::new(0.0, 0.0, 0.0);

            for _ in 0..NS {
                let u = (j as f32 + rng.gen_range(0.0, 1.0)) / WIDTH as f32;
                let v = (i as f32 + rng.gen_range(0.0, 1.0)) / HEIGHT as f32;
                let r = cam.get_ray(u, v);
                col += color(&r, &list);
            }

            col /= NS as f32;
            col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());
            let ir = (255.99*col.x) as u32;
            let ig = (255.99*col.y) as u32;
            let ib = (255.99*col.z) as u32;
            buffer[to_buffer_index(i, j, WIDTH, HEIGHT)] = to_bgra(ir, ig, ib);
        }
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(buffer.as_slice()).unwrap();
    }
}