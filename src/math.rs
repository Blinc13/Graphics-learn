use std::ops::{Add, Mul, Sub};
use crate::types::{Mat3, Mat4, FVec3, FVec2};

#[inline(always)]
pub fn interpolate<T: Add<<<T as Sub>::Output as Mul<f32>>::Output, Output = T> + Sub + Copy>(v1: T, v2: T, f: f32) -> T
    where <T as Sub>::Output: Mul<f32>,
{
    v1 + (v2 - v1) * f
}

#[inline(always)]
pub fn courtesan_to_barycentric(point: FVec2, points: [FVec2; 3]) -> FVec3 {
    let a = Mat3::new(
        points[0].x, points[1].x, points[2].x,
        points[0].y, points[1].y, points[2].y,
        1.0, 1.0, 1.0
    );
    let a1 = Mat3::new(
        point.x, points[1].x, points[2].x,
        point.y, points[1].y, points[2].y,
        1.0, 1.0, 1.0
    );
    let a2 = Mat3::new(
        points[0].x, point.x, points[2].x,
        points[0].y, point.y, points[2].y,
        1.0, 1.0, 1.0
    );
    let a3 = Mat3::new(
        points[0].x, points[1].x, point.x,
        points[0].y, points[1].y, point.y,
        1.0, 1.0, 1.0
    );

    let det = a.determinant();
    let det1 = a1.determinant();
    let det2 = a2.determinant();
    let det3 = a3.determinant();

    FVec3::new(
        det1 / det,
        det2 / det,
        det3 / det
    )
}

pub struct CamBasis {
    pub x: FVec3,
    pub y: FVec3,
    pub z: FVec3,

    pub pos: FVec3
}

#[inline(always)]
fn rotate_by_axis(axis: FVec3, angle: f32) -> Mat3 {
    let cos = angle.cos();
    let sin = angle.sin();
    let t = 1.0 - cos;

    Mat3::new(
        t * axis.x * axis.x + cos         , t * axis.x * axis.y - sin * axis.z, t * axis.x * axis.z + sin * axis.y,
        t * axis.x * axis.y + sin * axis.z, t * axis.y * axis.y + cos,          t * axis.y * axis.z - sin * axis.x,
        t * axis.x * axis.z - sin * axis.y, t * axis.y * axis.z + sin * axis.x,       t * axis.z * axis.z + cos
    )
}

impl CamBasis {
    pub fn new() -> Self {
        Self {
            x: FVec3::new(1.0, 0.0, 0.0),
            y: FVec3::new(0.0, 1.0, 0.0),
            z: FVec3::new(0.0, 0.0, 1.0),

            pos: FVec3::zeros()
        }
    }

    pub fn rotate(&mut self, axis: FVec3, angle: f32) {
        let mat = rotate_by_axis(axis, angle);

        self.x = mat * self.x;
        self.y = mat * self.y;
        self.z = mat * self.z;
    }

    pub fn translate(&mut self, by: FVec3) {
        self.pos += by;
    }

    pub fn as_basis_mat(&self) -> Mat3 {
        Mat3::new(
            self.x.x, self.x.y, self.x.z,
            self.y.x, self.y.y, self.y.z,
            self.z.x, self.z.y, self.z.z,
        ).transpose()
    }

    pub fn build_look_at_matrix(&self) -> Mat4 {
        Mat4::new(
            self.x.x, self.x.y, self.x.z, 0.0,
            self.y.x, self.y.y, self.y.z, 0.0,
            self.z.x, self.z.y, self.z.z, 0.0,
            0.0, 0.0, 0.0, 1.0
        ) *
            Mat4::new_translation(&-self.pos)
    }
}