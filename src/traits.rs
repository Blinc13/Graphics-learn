use glam::Vec3;
use image::Rgb;

pub trait Render {
    fn intersect(&self, origin: Vec3, ray_dir: Vec3) -> IntersectResult;

    fn get_color(&self) -> Rgb<u8>;
    fn get_position(&self) -> Vec3;
}

pub trait Light {
    fn get_intensity(&self) -> f32;

    fn get_direction(&self, point: Vec3, normal: Vec3) -> Vec3;
}


pub struct Intersect {
    pub entry: f32,
    pub exit: f32,
    pub normal: Vec3
}

pub enum IntersectResult {
    Intersected(Intersect),
    NoneIntersect
}

impl Intersect {
    pub fn new(entry: f32, exit: f32, normal: Vec3) -> Self {
        Self {
            entry,
            exit,
            normal
        }
    }
}