use smallvec::SmallVec;

pub struct OctTree<T> {
    childs: SmallVec<[T; 8]>
}