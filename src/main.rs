pub mod scene;
use scene::*;

pub mod window;
use vek::Vec3;
use window::run_window;

mod tracing;
use tracing::*;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 360;



fn main() {
    let pos = Vec3::new(0.0, 0.0, 0.0);
    let camera = Camera::new(pos, 0.0,0.0,0.0);
    let scene = Scene::new(WIDTH, HEIGHT, camera);
    run_window(scene, update_buffer);
}