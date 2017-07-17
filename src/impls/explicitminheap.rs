use ::std::cmp;

use super::Heap;

enum ExplicitMinHeap<H, T>
    where H: Heap<T>
{
    Empty,
    Node(T, H)
}

impl <H, T> Clone for ExplicitMinHeap<H, T>
    where H: Heap<T> + Clone,
          T: Clone
{
    fn clone(&self) -> ExplicitMinHeap<H, T> {
        match self {
            &Empty => Empty,
            &Node(ref min, ref h) => Node(min.clone(), h.clone())
        }
    }
}

use self::ExplicitMinHeap::*;

impl <H, T> Heap<T> for ExplicitMinHeap<H, T>
    where H: Heap<T> + Clone,
          T: Ord + Clone
{
    fn empty() -> ExplicitMinHeap<H, T> {
        Empty
    }

    fn is_empty(&self) -> bool {
        match self {
            &Empty => true,
            &Node(_, _) => false
        }
    }

    fn insert(&self, node: T) -> ExplicitMinHeap<H, T> {
        match self {
            &Empty => {
                let h = H::empty().insert(node.clone());
                Node(node, h)
            },
            &Node(ref min, ref h) => {
                let min = cmp::min(min, &node).clone();
                let h = h.insert(node);
                Node(min, h)
            }
        }
    }

    fn merge(&self, other: &ExplicitMinHeap<H, T>) -> ExplicitMinHeap<H, T> {
        match (self, other) {
            (ts, &Empty) => ts.clone(),
            (&Empty, ts) => ts.clone(),
            (&Node(ref m1, ref h1), &Node(ref m2, ref h2)) => {
                let min = cmp::min(m1, m2).clone();
                let h = h1.merge(h2);
                Node(min, h)
            }
        }
    }

    fn find_min(&self) -> &T {
        match self {
            &Empty => panic!("find_min called for empty tree"),
            &Node(ref min, _) => min
        }
    }

    fn delete_min(&self) -> ExplicitMinHeap<H, T> {
        match self {
            &Empty => panic!("delete_min called for empty tree"),
            &Node(_, ref h) => {
                let min = h.find_min().clone();
                let h = h.delete_min();
                Node(min, h)
            }
        }
    }

}
