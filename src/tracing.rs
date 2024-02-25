use crate::scene::*;

use vek::*;

use nalgebra::*;


trait ColorHelper {
    fn u32color(&self) -> u32;
}
impl ColorHelper for Rgb<u8> {
    fn u32color(&self) -> u32 {
        let (r, g, b) = (self.r as u32, self.g as u32, self.b as u32);
        return (r << 16) | (g << 8) | b;
    }
}

pub fn update_buffer(scene: Scene) -> Vec<u32> {
    let mut buffer = Vec::new();
    for i in cast_rays(scene) {
        buffer.push(i.u32color())
    }
    return buffer;
}
fn cast_rays(scene: Scene) -> Vec<Rgb<u8>> {
    let (w, h) = (scene.width as u32,scene.height as u32);
    let mut buffer = Vec::new();
    for x in 0..w {
        for y in 0..h {
            let origin = 0;
            
        }
    }
    return buffer
}