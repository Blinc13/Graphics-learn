use glam::Vec3;
use image::Rgb;
use crate::traits::{IntersectResult, Render};

pub struct Square {
    pos: Vec3,
    size: f32,
    color: Rgb<u8>
}

impl Render for Square {
    fn get_color(&self) -> Rgb<u8> {
        todo!()
    }

    fn get_position(&self) -> Vec3 {
        self.pos
    }

    fn intersect(&self, origin: Vec3, ray_dir: Vec3) -> IntersectResult {
        
    }
}