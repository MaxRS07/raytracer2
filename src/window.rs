use minifb::{Key, Window, WindowOptions};
use crate::scene::*;


pub fn run_window(scene: Scene, update_buffer: fn(scene: Scene)->Vec<u32>) {
    let (width, height) = (scene.width, scene.height);
    let mut buffer: Vec<u32> = vec![0; width * height];
    
    let mut window = Window::new(
        "Test - ESC to exit",
        width,
        height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        buffer = update_buffer(scene);

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, width, height)
            .unwrap();
    }
}