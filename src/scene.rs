use std::ops::Index;
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
use crate::Const::Vec::UP;
use crate::traits::{Intersect, IntersectResult};

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

    pub fn intersect_ray(&self, point: Vec3, direction: Vec3) -> Option<(Intersect, &dyn Render)> {
        self.objects.iter()
            .filter_map(| object |
                match object.intersect(point, direction) {
                    IntersectResult::Intersected(intersect) => Some((intersect, object.as_ref())),
                    IntersectResult::NoneIntersect => None
                })
            .filter(| (Intersect { entry, ..}, _) | *entry > 0.0)
            .min_by(
                | (Intersect { entry: t1, exit: t3, .. }, _),
                  (Intersect { entry: t2, exit: t4, .. }, _) |
                t1.total_cmp(t2).then(t3.total_cmp(t4))
            )
    }

    pub fn compute_light(&self, point: Vec3, dir: Vec3, normal: Vec3) -> f32 {
        self.light.iter()
            .filter( | light | {
                light.is_point() ||
                    self
                        .intersect_ray(point, light.get_direction(point, normal))
                        .filter(| (Intersect { entry, ..}, _ ) | *entry > 0.0).is_none()
            })
            .map(| light | {
                light.get_intensity()
            })
            .filter(| intensive | *intensive > 0.0)
            .sum()
    }

    pub fn render_pixel(&self, cord: Vec2) -> Rgb<u8> {
        let origin = RIGHT * 0.0 + UP * 0.0;
        let direction = Vec3 {
            x: cord.x * (1.0 / 1000.0),
            y: cord.y * (1.0 / 1000.0),
            ..FORWARD
        }.normalize();

        self.intersect_ray(origin, direction)
            .map(| ( Intersect {entry, normal, ..}, object) | {
                let dir = (direction - origin).normalize();
                let point = dir * entry;

                let light_intensive = self.compute_light(point + normal * 0.00001, dir, normal).clamp(0.0, 150.0) as u8; // All this temporary
                let object_color = object.get_color().0;

                Rgb::from([object_color[0] + light_intensive, object_color[1] + light_intensive, object_color[2] + light_intensive])
                //object_color
            })
            .unwrap_or(BLACK)
    }
}
