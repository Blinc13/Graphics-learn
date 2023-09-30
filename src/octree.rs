use std::cmp::Ordering;
use std::ops::Deref;

use crate::math::AxisAlignedBox;
use crate::{RayIntersection, IntersectionData};
use crate::types::FVec3;

use smallvec::SmallVec;

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

pub trait OctTreeElement {
    type Meta;
    type RayIter<'a, F>: Iterator<Item = IntersectionData<Self::Meta>>
        where Self: 'a,
              F: Fn(IntersectionData<()>) -> bool + Copy;
    const SIZE: FVec3;

    fn intersect_in_place(
        &self,
        pos: FVec3,
        extents: FVec3,
        origin: FVec3,
        direction: FVec3
    ) -> Option<IntersectionData<Self::Meta>>;

    fn build_ray_iterator<'a, F: Fn(IntersectionData<()>) -> bool + Copy>(
        &'a self,
        pos: FVec3,
        extents: FVec3,
        origin: FVec3,
        direction: FVec3,
        filter: F
    ) -> Self::RayIter<'a, F>;
}

pub struct OctTree<T> {
    childs: [Option<T>; 8]
}

impl<T: OctTreeElement> OctTree<T> {
    pub fn new_from_childs(childs: [Option<T>; 8]) -> Self {
        Self {childs}
    }
}

impl<T: OctTreeElement> OctTreeElement for OctTree<T>
    where T::Meta: Copy
{
    type Meta = T::Meta;
    type RayIter<'a, F: Fn(IntersectionData<()>) -> bool + Copy> = RayIterator<'a, T, F> where T: 'a;
    const SIZE: FVec3 = FVec3::new(0.0, 0.0, 0.0);

    fn intersect_in_place(
        &self,
        pos: FVec3,
        extents: FVec3,
        origin: FVec3,
        direction: FVec3
    ) -> Option<IntersectionData<Self::Meta>> {
        let child_extents = extents / 2.0;

        let mut ints: SmallVec<[_; 4]> = OFFSETS.iter()
            .copied()
            .map(| off | pos + off.component_mul(&child_extents))
            .zip(self.childs.iter().map(| child | child.as_ref()))
            .filter_map(| (pos, child) | Some((pos, child?)))
            .filter_map(| (pos, child) | Some((pos, child, check_aab(pos, child_extents, origin, direction)?)))
            .collect();

        ints.sort_by(| (_, _, a), (_, _, b) | a.min.total_cmp(&b.min));

        ints.into_iter()
            .filter_map(| (pos, child, _) | child.intersect_in_place(pos, child_extents, origin, direction))
            .next()
    }

    fn build_ray_iterator<'a, F: Fn(IntersectionData<()>) -> bool + Copy>(
        &'a self,
        pos: FVec3,
        extents: FVec3,
        origin: FVec3,
        direction: FVec3,
        filter: F
    ) -> Self::RayIter<'a, F> {
        let child_extents = extents / 2.0;

        let iter = OFFSETS.iter()
            .copied()
            .map(| off | pos + off.component_mul(&child_extents))
            .zip(self.childs.iter().map(| c | c.as_ref()))
            .filter_map(| (pos, child) | Some((pos, child?)))
            .filter_map(| (pos, child) | Some((pos, child, check_aab(pos, child_extents, origin, direction)?)))
            .filter(| (_, _, int) | filter(*int));
        
        RayIterator {
            extents: child_extents,

            origin,
            direction,

            filter,

            iter: SortIterator::new(iter, | (_, _, a), (_, _, b) | a.min.total_cmp(&b.min)),
            t_iter: None
        }
    }
}

impl<T: OctTreeElement> OctTreeElement for Box<OctTree<T>>
    where T::Meta: Copy
{
    type Meta = T::Meta;
    type RayIter<'a, F: Fn(IntersectionData<()>) -> bool + Copy> = RayIterator<'a, T, F> where T: 'a;
    const SIZE: FVec3 = FVec3::new(0.0, 0.0, 0.0);

    fn intersect_in_place(
        &self,
        pos: FVec3,
        extents: FVec3,
        origin: FVec3,
        direction: FVec3
    ) -> Option<IntersectionData<Self::Meta>> {
        self.deref().intersect_in_place(pos, extents, origin, direction)
    }

    fn build_ray_iterator<'a, F: Fn(IntersectionData<()>) -> bool + Copy>(
        &'a self,
        pos: FVec3,
        extents: FVec3,
        origin: FVec3,
        direction: FVec3,
        filter: F
    ) -> Self::RayIter<'a, F> {
        self.deref().build_ray_iterator(pos, extents, origin, direction, filter)
    }
}

impl<T: OctTreeElement> RayIntersection for OctTree<T>
    where T::Meta: Copy
{
    type Meta = T::Meta;

    #[inline(always)]
    fn intersect(&self, origin: FVec3, direction: FVec3) -> Option<IntersectionData<Self::Meta>> {
        const POS: FVec3 = FVec3::new(0.0, 0.0, 0.0);
        const EXTENTS: FVec3 = FVec3::new(0.5, 0.5, 0.5);

        self.intersect_in_place(
            POS,
            EXTENTS,
            origin,
            direction
        )
    }
}

use std::collections::VecDeque;

struct SortIterator<T> {
    data: VecDeque<T>
}

impl<T> SortIterator<T> {
    fn new(iter: impl Iterator<Item = T>, sort: impl FnMut(&T, &T) -> Ordering) -> Self {
        let mut data = VecDeque::from_iter(iter);
        
        data.make_contiguous().sort_by(sort);

        Self {data}
    }
}

impl<T> Iterator for SortIterator<T> {
    type Item = T;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.data.pop_front()
    }
}

pub struct RayIterator<'a, T: OctTreeElement, F: Fn(IntersectionData<()>) -> bool + Copy> {
    extents: FVec3,

    origin: FVec3,
    direction: FVec3,

    filter: F,

    iter: SortIterator<(FVec3, &'a T, IntersectionData<()>)>,
    t_iter: Option<T::RayIter<'a, F>>
}

impl<'a, T: OctTreeElement, F: Fn(IntersectionData<()>) -> bool + Copy> Iterator for RayIterator<'a, T, F> {
    type Item = IntersectionData<T::Meta>;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(t_iter) = self.t_iter.as_mut() {
            let ne = t_iter.next();

            if let Some(i) = ne {
                Some(i)
            } else {
                self.t_iter = Some(self.iter.next().map(| (pos, c, _) | c.build_ray_iterator(
                    pos,
                    self.extents,
                    self.origin,
                    self.direction,
                    self.filter
                ))?);
                self.next()
            }
        } else {
            self.t_iter = Some(self.iter.next().map(| (pos, c, _) | c.build_ray_iterator(
                pos,
                self.extents,
                self.origin,
                self.direction,
                self.filter
            ))?);
            self.next()
        }
    }
}


fn check_aab(pos: FVec3, extents: FVec3, origin: FVec3, direction: FVec3) -> Option<IntersectionData<()>> {
    AxisAlignedBox::from_position_and_extents(
        pos,
        extents
    ).intersect(origin, direction)
}