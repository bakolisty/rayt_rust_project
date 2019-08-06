extern crate minifb;

use minifb::{Key, WindowOptions, Window};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn to_buffer_index(i: usize, j: usize, width: usize, height: usize) -> usize {
    ((height - 1 - i) * width) + j
}

fn to_bgra(r: u32, g: u32, b: u32) -> u32 {
    255 << 24 | r << 16 | g << 8 | b
}

fn main() {

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new("Rayt_Rust - ESC to exit",
                                 WIDTH,
                                 HEIGHT,
                                 WindowOptions::default()).unwrap_or_else(|e| {
        panic!("{}", e);
    });


    for j in 0..WIDTH {
        for i in 0..HEIGHT {
            let r = j as f64 / WIDTH as f64;
            let g = i as f64 / HEIGHT as f64;
            let b = 0.2 as f64;
            let ir = (255.99*r) as u32;
            let ig = (255.99*g) as u32;
            let ib = (255.99*b) as u32;
            buffer[to_buffer_index(i, j, WIDTH, HEIGHT)] = to_bgra(ir, ig, ib);
        }
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(buffer.as_slice()).unwrap();
    }
}

