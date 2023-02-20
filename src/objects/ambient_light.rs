use glam::Vec3;
use crate::traits::Light;

pub struct AmbientLight {
    intensive: f32
}

impl AmbientLight {
    pub fn new(intensive: f32) -> Self {
        Self {
            intensive
        }
    }
}

impl Light for AmbientLight {
    fn get_intensity(&self) -> f32 {
        self.intensive
    }

    fn get_direction(&self, point: Vec3, normal: Vec3) -> Vec3 {
        normal
    }
    fn is_point(&self) -> bool {
        false
    }
}