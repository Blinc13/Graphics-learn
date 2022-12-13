use glam::Vec3;
use image::Rgb;

pub trait Render {
    fn intersect(&self, origin: Vec3, ray_dir: Vec3) -> (f32, f32);

    fn get_color(&self) -> Rgb<u8>;
}