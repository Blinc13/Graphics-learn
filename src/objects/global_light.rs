use glam::Vec3;
use image::imageops::dither;
use crate::traits::Light;

pub struct GlobalLight {
    direction: Vec3,
    intensive: f32
}

impl GlobalLight {
    pub fn new(direction: Vec3, intensive: f32) -> Self {
        Self {
            direction,
            intensive
        }
    }
}

impl Light for GlobalLight {
    fn get_intensity(&self) -> f32 {
        self.intensive
    }

    fn get_direction(&self, point: Vec3, normal: Vec3) -> Vec3 {
        self.direction
    }
}