use smallvec::SmallVec;
use crate::math::AxisAlignedBox;
use crate::RayIntersection;
use crate::types::FVec3;

pub trait OctTreeElement {
    fn recursive_ray_intersection(&self, pos: FVec3, extents: FVec3, origin: FVec3, direction: FVec3) -> Option<(f32, f32)>;
}

pub struct OctTree<T: OctTreeElement> {
    childs: [Option<T>; 8]
}

impl<T: OctTreeElement> OctTree<T> {
    pub fn new_from_childs(childs: [Option<T>; 8]) -> Self {
        Self {childs}
    }
}

impl<T: OctTreeElement> OctTreeElement for OctTree<T> {
    fn recursive_ray_intersection(&self, pos: FVec3, extents: FVec3, origin: FVec3, direction: FVec3) -> Option<(f32, f32)> {
        const OFFSETS: &[FVec3] = &[
            FVec3::new(-1.0, -1.0, -1.0),
            FVec3::new(1.0, -1.0, -1.0),
            FVec3::new(-1.0, -1.0, 1.0),
            FVec3::new(1.0, -1.0, 1.0),

            FVec3::new(-1.0, 1.0, -1.0),
            FVec3::new(1.0, 1.0, -1.0),
            FVec3::new(-1.0, 1.0, 1.0),
            FVec3::new(1.0, 1.0, 1.0)
        ];

        { // Self check
            let _ = AxisAlignedBox::from_position_and_extents(
                pos,
                extents
            ).intersect(origin, direction)?;
        }

        let child_extents = extents / 2.0;

        let mut intersections: SmallVec<[(f32, f32); 8]> = OFFSETS.iter()
            .copied()
            .map(| off | pos + off.component_mul(&child_extents))
            .zip(self.childs.iter().map(| child | child.as_ref()))
            .filter_map(| (pos, child) | Some((pos, child?)))
            .filter_map(| (pos, child) |
                child.recursive_ray_intersection(
                    pos,
                    child_extents,
                    origin,
                    direction
                )
            ).collect();

        intersections.sort_by(| a, b | a.0.total_cmp(&b.0));
        intersections.get(0).copied()
    }
}

impl<T: OctTreeElement> RayIntersection for OctTree<T> {
    #[inline(always)]
    fn intersect(&self, origin: FVec3, direction: FVec3) -> Option<(f32, f32)> {
        const POS: FVec3 = FVec3::new(0.0, 0.0, 0.0);
        const EXTENTS: FVec3 = FVec3::new(0.5, 0.5, 0.5);

        self.recursive_ray_intersection(
            POS,
            EXTENTS,
            origin,
            direction
        )
    }
}