use image::Rgb;
use glam::{Vec2, Vec3};
use crate::{
    Const::{
        Colors::BLACK,
        Vec::{
            FORWARD,
            ZERO
        }
    },
    traits::Render
};

pub struct Scene {
    objects: Vec<Box<dyn Render>>
}

impl Scene {
    pub fn new() -> Self {
        Self {
            objects: vec![]
        }
    }

    pub fn add_object(&mut self, object: Box<dyn Render>) {
        self.objects.push(object);
    }

    pub fn render_pixel(&self, cord: Vec2) -> Rgb<u8> {
        let origin = ZERO;
        let direction = Vec3 {
            x: cord.x * (1.0 / 1000.0),
            y: cord.y * (1.0 / 1000.0),
            ..FORWARD
        };

        self.objects.iter()
            .filter_map(| object | {
                let (t1, _) = object.intersect(origin, direction);

                if t1 != f32::INFINITY {
                    Some((t1, object))
                } else {
                    None
                }
            })
            .max_by(| (t1, _), (t2, _) | t1.total_cmp(t2))
            .map(| (_, object) | object.get_color())
            .unwrap_or(BLACK)
    }
}

impl From<Vec<Box<dyn Render>>> for Scene {
    fn from(objects: Vec<Box<dyn Render>>) -> Self {
        Self {
            objects
        }
    }
}

impl Into<Vec<Box<dyn Render>>> for Scene {
    fn into(self) -> Vec<Box<dyn Render>> {
        self.objects
    }
}