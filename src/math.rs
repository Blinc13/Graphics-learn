use crate::types::FVec3;
use crate::{RayIntersection, IntersectionData};

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
            dist: self.dist
        }
    }
}

impl RayIntersection for Plain {
    type Meta = ();

    #[inline(always)]
    fn intersect(&self, origin: FVec3, direction: FVec3) -> Option<IntersectionData<Self::Meta>> {
        let dot_dir = self.normal.dot(&direction);

        if dot_dir == 0.0 {
            return None
        }
        let dot = self.normal.dot(
            &(self.normal * self.dist - origin)
        );
        let res = dot / dot_dir;

        Some(
            IntersectionData::new(
                res,
                res,
                self.normal,
                -self.normal,
                ()
            )
        )
    }
}

#[derive(Debug, Copy, Clone)]
pub struct AxisAlignedBox {
    pub pos: FVec3,
    pub extents: FVec3
}

impl AxisAlignedBox {
    #[inline(always)]
    pub fn from_position_and_extents(pos: FVec3, extents: FVec3) -> Self {
        Self {
            pos,
            extents
        }
    }

    #[inline(always)]
    pub fn translate(&mut self, by: FVec3) {
        self.pos += by;
    }

    #[inline(always)]
    pub fn scale(&mut self, by: FVec3) {
        self.extents = self.extents.component_mul(&by);
    }

    #[inline(always)]
    pub fn contains(&self, point: FVec3) -> bool {
        self.contains_local(point - self.pos)
    }

    #[inline(always)]
    fn contains_local(&self, point: FVec3) -> bool {
        let point = point.abs();

        point.x <= self.extents.x && point.y <= self.extents.y && point.z <= self.extents.z
    }
}

impl RayIntersection for AxisAlignedBox {
    type Meta = ();

    fn intersect(&self, origin: FVec3, direction: FVec3) -> Option<IntersectionData<Self::Meta>> {
        const NORMALS: &[FVec3] = &[
            FVec3::new(-1.0, 0.0, 0.0),
            FVec3::new(0.0, -1.0, 0.0),
            FVec3::new(0.0, 0.0, -1.0),

            FVec3::new(1.0, 0.0, 0.0),
            FVec3::new(0.0, 1.0, 0.0),
            FVec3::new(0.0, 0.0, 1.0)
        ];

        // Make origin local to box center
        let origin = origin - self.pos;

        let components_iter = std::iter::repeat(
            self.extents.iter().copied()
        ).flatten();

        // Iterate over plains, intersect and check if intersection point inside box
        let mut ints = NORMALS
            .iter()
            .copied()
            .zip(components_iter)
            .filter_map(| (n, d) | Plain {normal: n, dist: d}.intersect(origin, direction))
            .filter(| d | self.contains_local((origin + direction * d.min) * 0.9999/*:)*/))
            .map(| d | d);

        // Must be 2 intersection points if ray intersects box
        let mut i1 = ints.next()?;
        let mut i2 = ints.next()?;

        if i1.min > i2.min {
            let i = i1;

            i1 = i2;
            i2 = i;
        }

        let i = IntersectionData::new(
            i1.min,
            i2.max,

            i1.in_normal,
            i2.in_normal,
            ()
        );

        Some(i)
    }
}