use crate::types::{FVec3, UVec2};

pub mod octree;
pub mod canvas;
pub mod types;
pub mod math;

pub trait RayIntersection {
    fn intersect(&self, origin: FVec3, direction: FVec3) -> Option<(f32, f32)>;
}

pub trait Canvas {
    fn get_resolution(&self) -> UVec2;

    fn fill(&mut self, val: FVec3);

    fn put_pixel(&mut self, pos: UVec2, val: FVec3);
    fn read_pixel(&self, pos: UVec2) -> FVec3;
}