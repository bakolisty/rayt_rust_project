extern crate minifb;

use minifb::{Key, WindowOptions, Window};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new("Test - ESC to exit",
                                 WIDTH,
                                 HEIGHT,
                                 WindowOptions::default()).unwrap_or_else(|e| {
        panic!("{}", e);
    });


    for i in 0..WIDTH*HEIGHT {
        let r = i as f64 / WIDTH as f64;
        let g = i as f64 / HEIGHT as f64;
        let b = 0.2 as f64;
        let ir = (255.99*r.sqrt()) as u32;
        let ig = (255.99*g.sqrt()) as u32;
        let ib = (255.99*b.sqrt()) as u32;
        buffer[i] = to_bgra(ir, ig, ib);
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(buffer.as_slice()).unwrap();
    }
}


fn to_bgra(r: u32, g: u32, b: u32) -> u32 {
    255 << 24 | r << 16 | g << 8 | b
}