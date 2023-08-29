use crate::types::{FVec2, FVec3, FVec4, UVec2};

pub trait Canvas {
    fn get_resolution(&self) -> UVec2;
    fn read_pixel(&self, pos: UVec2) -> FVec3;
    
    fn put_pixel(&mut self, pos: UVec2, val: FVec3);
    fn fill(&mut self, val: FVec3);
}

pub trait Painter {
    fn canvas_resolution(&self) -> UVec2;

    fn draw_line<F: Copy + Fn(FVec2, FVec3) -> Option<FVec3>>(&mut self, start: FVec2, end: FVec2, pixel_fill: F);
    fn draw_triangle_lines<F: Copy + Fn(FVec2, FVec3) -> Option<FVec3>>(&mut self, vert: [FVec2; 3], pixel_fill: F);
    fn fill_triangle<F: Copy + Fn(FVec2, FVec3) -> Option<FVec3>>(&mut self, vert: [FVec2; 3], pixel_fill: F);
}

pub trait Rasterizer {
    fn draw_triangle<V: Copy + Fn(FVec3) -> FVec4, F: Copy + Fn(FVec3, FVec3) -> FVec3>(&mut self, triangle: [FVec3; 3], vert: V, frag: F);
}

pub mod types {
    pub type Mat4 = nalgebra::Matrix4<f32>;
    pub type Mat3 = nalgebra::Matrix3<f32>;
    pub type Mat2 = nalgebra::Matrix2<f32>;

    pub type Vec4<T> = nalgebra::Vector4<T>;
    pub type IVec4 = Vec4<i32>;
    pub type UVec4 = Vec4<u32>;
    pub type FVec4 = Vec4<f32>;

    pub type Vec3<T> = nalgebra::Vector3<T>;
    pub type IVec3 = Vec3<i32>;
    pub type UVec3 = Vec3<u32>;
    pub type FVec3 = Vec3<f32>;

    pub type Vec2<T> = nalgebra::Vector2<T>;
    pub type IVec2 = Vec2<i32>;
    pub type UVec2 = Vec2<u32>;
    pub type FVec2 = Vec2<f32>;
}

pub mod math;
pub mod painter;
pub mod rasterizer;
pub mod ascii_canvas;