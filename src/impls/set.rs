pub trait Set<T> {
    fn empty() -> Self;
    fn member(&self, elem: &T) -> bool;
    fn insert(&self, elem: T) -> Self;
}
