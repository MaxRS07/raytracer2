use vek::*;


pub struct Scene {
    pub width: usize,
    pub height: usize,
    pub camera: Camera,    
}
impl Scene {
    pub fn new(
        width: usize,
        height: usize,
        camera: Camera
    ) -> Self {
        return Scene {
            width,
            height,
            camera,
        }
    }
}
#[derive(Copy, Clone, Debug)]
pub struct Camera {
    pos: Vec3<f32>,
    pitch: f32,
    yaw:  f32,
    fov: f32,
}
impl Camera {
    pub fn new(pos: Vec3<f32>, pitch: f32, yaw: f32, fov: f32) -> Self {
        return Camera { pos: pos, pitch: pitch, yaw: yaw, fov: fov }
    }
}
pub struct Ray {
    pub origin: Vec3<f32>,
    pub dir: Vec3<f32>,
}
impl Ray {
    pub fn new(origin: Vec3<f32>, dir: Vec3<f32>) -> Self {
        return Ray { origin, dir }
    }
}