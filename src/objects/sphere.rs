use glam::Vec3;
use image::Rgb;
use crate::traits::Render;

pub struct Sphere {
    position: Vec3,
    size: Vec3,
    radius: f32,
    color: Rgb<u8>
}

impl Sphere {
    pub fn new(pos: Vec3, size: Vec3, r: f32, color: Rgb<u8>) -> Self {
        Self {
            position: pos,
            size,
            radius: r,
            color
        }
    }
}

impl Render for Sphere {
    //<CO + tD, CO + tD> = r*r
    fn intersect(&self, origin: Vec3, direction: Vec3) -> (f32, f32) {
        let CO = (origin - self.position) / self.size;
        let D = direction / self.size;

        let a = D.dot(D);
        let b = 2.0 * CO.dot(D);
        let c = CO.dot(CO) - self.radius*self.radius;

        let desc = (b*b) - (4.0 * a * c);

        if desc < 0.0 {
            return (f32::INFINITY, f32::INFINITY);
        }

        let desc = desc.sqrt();

        (
            (-b + desc) / (2.0 * a),
            (-b - desc) / (2.0 * a)
        )
    }

    fn get_color(&self) -> Rgb<u8> {
        self.color
    }
    fn get_position(&self) -> Vec3 {
        self.position
    }
}