use smallvec::SmallVec;
use crate::types::FVec3;
use crate::RayIntersection;

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
    #[inline(always)]
    fn intersect(&self, origin: FVec3, direction: FVec3) -> Option<(f32, f32)> {
        let dot_dir = self.normal.dot(&direction);

        if dot_dir == 0.0 {
            return None
        }
        let dot = self.normal.dot(
            &(self.normal * self.dist - origin)
        );
        let res = dot / dot_dir;

        Some((res, res))
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
        self.contains_local(self.pos - point)
    }

    #[inline(always)]
    fn contains_local(&self, point: FVec3) -> bool {
        let point = point.abs();

        point.x <= self.extents.x && point.y <= self.extents.y && point.z <= self.extents.z
    }
}

impl RayIntersection for AxisAlignedBox {
    fn intersect(&self, origin: FVec3, direction: FVec3) -> Option<(f32, f32)> {
        let origin = origin - self.pos;

        const NORMALS: &[FVec3] = &[
            FVec3::new(-1.0, 0.0, 0.0),
            FVec3::new(0.0, -1.0, 0.0),
            FVec3::new(0.0, 0.0, -1.0),

            FVec3::new(1.0, 0.0, 0.0),
            FVec3::new(0.0, 1.0, 0.0),
            FVec3::new(0.0, 0.0, 1.0)
        ];

        let components_iter = self.extents.iter()
            .copied()
            .chain(self.extents.iter().copied());

        let mut ints: SmallVec<[f32; 2]> = NORMALS
            .iter()
            .copied()
            .zip(components_iter)
            .filter_map(| (n, d) | Plain {normal: n, dist: d}.intersect(origin, direction))
            .map(| (d, _) | (d, origin + direction * d))
            .filter(| (_, p) | self.contains_local(*p * 0.99))
            .map(| (d, _) | d).collect();

        if ints.len() < 2 {
            None
        } else {
            ints.sort_by(| a, b | a.total_cmp(b));

            Some((ints[0], ints[1]))
        }
    }
}