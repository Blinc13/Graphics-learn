use std::ops::Range;
use nalgebra::RawStorage;
use crate::RayIntersection;
use crate::types::{FVec2, FVec3};

#[derive(Debug, Copy, Clone)]
pub struct Plain {
    pub normal: FVec3,
    // Distance from cord origin
    pub dist: f32
}

impl Plain {
    #[inline(always)]
    pub fn invert(self) -> Self {
        Self {
            normal: -self.normal,
            dist: -self.dist
        }
    }
}

impl RayIntersection for Plain {
    #[inline(always)]
    fn intersect(&self, origin: FVec3, direction: FVec3) -> Option<(f32, f32)> {
        let dot_dir = self.normal.dot(&direction);

        if dot_dir == 0.0 {
            return None
        }
        let dot = self.normal.dot(
            &(origin - self.normal * self.dist)
        );
        let res = dot / dot_dir;

        Some((res, res))
    }
}

#[derive(Debug, Copy, Clone)]
pub enum AABFace {
    XPos,
    XNeg,
    YPos,
    YNeg,
    ZPos,
    ZNeg
}

impl Into<FVec3> for AABFace {
    #[inline(always)]
    fn into(self) -> FVec3 {
        match self {
            AABFace::XPos => FVec3::new(1.0, 0.0, 0.0),
            AABFace::XNeg => FVec3::new(-1.0, 0.0, 0.0),
            AABFace::YPos => FVec3::new(0.0, 1.0, 0.0),
            AABFace::YNeg => FVec3::new(0.0, -1.0, 0.0),
            AABFace::ZPos => FVec3::new(0.0, 0.0, 1.0),
            AABFace::ZNeg => FVec3::new(0.0, 0.0, -1.0)
        }
    }
}

pub struct AxisAlignedBox {
    pub pos: FVec3,
    pub extents: FVec3
}

impl AxisAlignedBox {
    // Assumes what int was divided by extents
    #[inline(always)]
    fn check_face_intersection(face: AABFace, int: FVec3) -> bool {
        const RANGE: Range<f32> = -1.0..1.0;

        let check_edges = match face {
            AABFace::XPos => FVec2::new(int.y, int.z),
            AABFace::XNeg => FVec2::new(int.y, int.z),
            AABFace::YPos => FVec2::new(int.x, int.z),
            AABFace::YNeg => FVec2::new(int.x, int.z),
            AABFace::ZPos => FVec2::new(int.x, int.y),
            AABFace::ZNeg => FVec2::new(int.x, int.y)
        };

        let b = RANGE.contains(&check_edges.x) && RANGE.contains(&check_edges.y);

        if b {
            println!("Intersects!");
        } else {
            println!("Not intersect!");
        }

        b
    }
}

impl RayIntersection for AxisAlignedBox {
    fn intersect(&self, origin: FVec3, direction: FVec3) -> Option<(f32, f32)> {
        let origin = origin - self.pos;

        let plain_types = [
            AABFace::XPos,
            AABFace::XNeg,
            AABFace::YPos,
            AABFace::YNeg,
            AABFace::ZPos,
            AABFace::ZNeg
        ];
        let plains = [
            Plain {
                normal: FVec3::new(1.0, 0.0, 0.0),
                dist: self.extents.x
            },
            Plain {
                normal: FVec3::new(-1.0, 0.0, 0.0),
                dist: -self.extents.x
            },
            Plain {
                normal: FVec3::new(0.0, 1.0, 0.0),
                dist: self.extents.y
            },
            Plain {
                normal: FVec3::new(0.0, -1.0, 0.0),
                dist: -self.extents.y
            },
            Plain {
                normal: FVec3::new(0.0, 0.0, 1.0),
                dist: self.extents.z
            },
            Plain {
                normal: FVec3::new(0.0, 0.0, -1.0),
                dist: -self.extents.z
            }
        ];

        let mut iter = plains.iter().copied()
            .zip(plain_types.iter().copied())
            .map(| (plain, plain_type) | (plain.intersect(origin, direction).map(| r | r.0), plain, plain_type))
            .filter_map(| (int, plain, plaint_type) | int.and_then(| int | Some((int, plain, plaint_type))))
            .map(| (int, plain, plain_type) | (origin + direction * int, int, plain, plain_type))
            .filter(| (int, _, _, plain_type) | AxisAlignedBox::check_face_intersection(*plain_type, *int))
            .map(| (_, int, _, _) | int);

        Some (
            (
                iter.next()?,
                iter.next()?,
                )
        )
    }
}