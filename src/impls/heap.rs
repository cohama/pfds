pub trait Heap<T> {
    fn empty() -> Self;
    fn is_empty(&self) -> bool;
    fn insert(&self, node: T) -> Self;
    fn merge(&self, other: &Self) -> Self;
    fn find_min(&self) -> &T;
    fn delete_min(&self) -> Self;
}
