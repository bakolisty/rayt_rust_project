extern crate minifb;

mod vec3;
mod ray;
mod hitable;
mod sphere;

use minifb::{Key, WindowOptions, Window};
use vec3::Vec3;
use ray::Ray;
use hitable::HitableList;
use hitable::Hitable;

const WIDTH: usize = 640;
const HEIGHT: usize = 320;

fn to_buffer_index(i: usize, j: usize, width: usize, height: usize) -> usize {
    ((height - 1 - i) * width) + j
}

fn to_bgra(r: u32, g: u32, b: u32) -> u32 {
    255 << 24 | r << 16 | g << 8 | b
}

/*
fn hit_sphere(center: Vec3, radius: f32, r: &Ray) -> f32 {
    let oc = r.origin - center;
    let a = r.direction.dot(r.direction);
    let b = 2.0 * oc.dot(r.direction);
    let c = oc.dot(oc) - (radius*radius);
    let discriminant = b*b - (4.0*a*c);
    if discriminant < 0.0 {
        return -1.0
    }
    else {
        (-b - discriminant.sqrt()) / (2.0*a)
    }
}
*/

fn color(r: &Ray, world: &HitableList) -> Vec3 {
    if let Some(hit) = world.hit(&r, 0.0, 10000.0) {
        return Vec3::new(hit.normal.x+1.0, hit.normal.y+1.0, hit.normal.z+1.0)*0.5
    }
    else {
        let unit_dir = vec3::unit_vec3(r.direction);
        let t = 0.5*(unit_dir.y+1.0);
        Vec3::new(1.0, 1.0, 1.0)*(1.0-t) + Vec3::new(0.5, 0.7, 1.0)*t
    }
    /*
    let t = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, &r);
    if t > 0.0 {
        let n = vec3::unit_vec3(r.point_at_parameter(t) - Vec3::new(0.0, 0.0, -1.0));
        return Vec3::new(n.x+1.0, n.y+1.0, n.z+1.0)*0.5
    }
    let unit_dir = vec3::unit_vec3(r.direction);
    let t = 0.5*(unit_dir.y+1.0);
    Vec3::new(1.0, 1.0, 1.0)*(1.0-t) + Vec3::new(0.5, 0.7, 1.0)*t
    */
}

fn main() {

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new("Rayt_Rust - ESC to exit",
                                 WIDTH,
                                 HEIGHT,
                                 WindowOptions::default()).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let lower_left_corner = Vec3::new(-1.6, -0.8, -1.0);
    let horizontal = Vec3::new(3.2, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 1.6, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let spheres = vec![
        sphere::Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5),
        sphere::Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0),
    ];
    let list = hitable::HitableList::new(spheres);

    for j in 0..WIDTH {
        for i in 0..HEIGHT {
            //let col = Vec3::new(j as f32/WIDTH as f32, i as f32/HEIGHT as f32, 0.2 as f32);
            let u = j as f32 / WIDTH as f32;
            let v = i as f32 / HEIGHT as f32;
            let r = Ray::new(origin, lower_left_corner + (horizontal*u) + (vertical*v));
            let col = color(&r, &list);
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

