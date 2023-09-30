use crate::types::{FVec3, UVec2};

pub mod octree;
pub mod canvas;
pub mod types;
pub mod math;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IntersectionData<T> {
    pub min: f32,
    pub max: f32,

    pub in_normal: FVec3,
    pub out_normal: FVec3,
    pub meta: T
}

impl<T> IntersectionData<T> {
    pub fn new(min: f32, max: f32, in_normal: FVec3, out_normal: FVec3, meta: T) -> Self {
        Self {
            min,
            max,

            in_normal,
            out_normal,
            meta
        }
    }

    pub fn with_meta<U, F: FnOnce(T) -> U>(self, f: F) -> IntersectionData<U> {
        IntersectionData {
            min: self.min,
            max: self.max,

            in_normal: self.in_normal,
            out_normal: self.out_normal,

            meta: f(self.meta)
        }
    }
}

impl<T: std::fmt::Display> std::fmt::Display for IntersectionData<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Intersection {}-{}\nEnter normal: {}\nOut normal: {}",
            self.min,
            self.max,
            self.in_normal,
            self.out_normal
        )?;
        
        self.meta.fmt(f)
    }
}

impl<T: PartialEq> std::cmp::PartialOrd for IntersectionData<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.min.partial_cmp(&other.min)
    }
}

pub trait RayIntersection {
    type Meta;

    fn intersect(&self, origin: FVec3, direction: FVec3) -> Option<IntersectionData<Self::Meta>>;
}

pub trait Canvas {
    fn get_resolution(&self) -> UVec2;

    fn fill(&mut self, val: FVec3);

    fn put_pixel(&mut self, pos: UVec2, val: FVec3);
    fn read_pixel(&self, pos: UVec2) -> FVec3;
}