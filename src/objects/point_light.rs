use glam::Vec3;
use crate::traits::Light;

pub struct PointLight {
    intensive: f32,
    position: Vec3
}

impl PointLight {
    pub fn new(position: Vec3, intensive: f32) -> Self {
        Self {
            intensive,
            position
        }
    }
}

impl Light for PointLight {
    fn get_intensity(&self) -> f32 {
        self.intensive
    }

    fn get_direction(&self, point: Vec3, _: Vec3) -> Vec3 {
        self.position - point
    }
    fn is_point(&self) -> bool {
        false
    }
}