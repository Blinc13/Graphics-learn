use glam::Vec3;
use image::Rgb;

pub trait Render {
    fn intersect(&self, origin: Vec3, ray_dir: Vec3) -> (f32, f32, Vec3);

    fn get_color(&self) -> Rgb<u8>;
}

pub trait Light {
    fn get_intensity(&self) -> f32;

    fn get_direction(&self, point: Vec3, normal: Vec3) -> Vec3;
}