use image::Rgb;
use glam::{Vec2, Vec3};
use crate::{
    Const::{
        Colors::BLACK,
        Vec::{
            FORWARD,
            RIGHT,
            ZERO
        }
    },
    traits::{
        Render,
        Light
    }
};

pub struct Scene {
    objects: Vec<Box<dyn Render>>,
    light: Vec<Box<dyn Light>>
}

impl Scene {
    pub fn new() -> Self {
        Self {
            objects: vec![],
            light: vec![]
        }
    }

    pub fn add_object(&mut self, object: Box<dyn Render>) {
        self.objects.push(object);
    }
    pub fn add_light(&mut self, light: Box<dyn Light>) {
        self.light.push(light)
    }

    pub fn intersect_ray(&self, point: Vec3, direction: Vec3) -> Option<(f32, &dyn Render)> {
        self.objects.iter()
            .filter_map(| object | {
                let (t1, _) = object.intersect(point, direction);

                if t1 != f32::INFINITY {
                    Some((t1, object.as_ref()))
                } else {
                    None
                }
            })
            .max_by(| (t1, _), (t2, _) | t1.total_cmp(t2))
    }

    pub fn compute_light(&self, point: Vec3, normal: Vec3) -> f32 {
        self.light.iter()
            .map(| light | {
                let dir = light.get_direction(point, normal);
                let dot = normal.dot(dir);

                (light.get_intensity() * dot / (normal.length() * dir.length()))
            })
            .filter(| intensive | *intensive > 0.0)
            .sum()
    }

    pub fn render_pixel(&self, cord: Vec2) -> Rgb<u8> {
        let origin = ZERO;
        let direction = Vec3 {
            x: cord.x * (1.0 / 1000.0),
            y: cord.y * (1.0 / 1000.0),
            ..FORWARD
        }.normalize();

        self.intersect_ray(origin, direction)
            .map(| (entry, object) | {
                let point = direction * entry;
                let normal = point - object.get_position();

                let light_intensive = self.compute_light(point, normal).clamp(0.0, 150.0) as u8; // All this temporary
                let object_color = object.get_color().0;

                Rgb::from([object_color[0] + light_intensive, object_color[1] + light_intensive, object_color[2] + light_intensive])
            })
            .unwrap_or(BLACK)
    }
}
