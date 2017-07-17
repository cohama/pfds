use std::rc::Rc;

use super::Heap;

pub struct LHeapNode<T> {
  pub rank: i32,
  pub node: T,
  pub left: LHeap<T>,
  pub right: LHeap<T>
}

#[derive(Clone)]
pub enum LHeap<T> {
    Empty,
    Tree(Rc<LHeapNode<T>>)
}
use self::LHeap::*;

impl <T> LHeap<T> {
    pub fn rank(&self) -> i32 {
        match *self {
            Empty => 0,
            Tree(ref lhn) => lhn.rank
        }
    }

    pub fn make(node: T, left: LHeap<T>, right: LHeap<T>) -> LHeap<T> {
        let lr = left.rank();
        let rr = right.rank();
        if lr >= rr {
            Tree(Rc::new(LHeapNode {rank: rr + 1, node, left, right}))
        } else {
            Tree(Rc::new(LHeapNode {rank: lr + 1, node, right, left}))
        }
    }
}

impl <T> Heap<T> for LHeap<T>
    where T: Ord + Clone
{
    fn empty() -> LHeap<T> {LHeap::Empty}
    fn is_empty(&self) -> bool {
        match *self {
            Empty => true,
            _ => false
        }
    }

    fn merge(&self, other: &LHeap<T>) -> LHeap<T> {
        match (self, other) {
            (&Empty, h) => {h.clone()},
            (h, &Empty) => {h.clone()},
            (&Tree(ref rh1), &Tree(ref rh2)) => {
                if rh1.node <= rh2.node {
                    LHeap::make(rh1.node.clone(), rh1.left.clone(), rh1.right.merge(other))
                } else {
                    LHeap::make(rh2.node.clone(), rh2.left.clone(), self.merge(&rh2.right))
                }
            }
        }
    }

    fn insert(&self, node: T) -> LHeap<T> {
        Tree(Rc::new(LHeapNode {rank: 1, node, left: Empty, right: Empty})).merge(self)
    }

    fn find_min(&self) -> &T {
        match self {
            &Empty => panic!("empty heap"),
            &Tree(ref rh) => &rh.node
        }
    }

    fn delete_min(&self) -> LHeap<T> {
        match self {
            &Empty => panic!("empty heap"),
            &Tree(ref rh) => rh.left.merge(&rh.right)
        }
    }

}

pub fn run() {
    println!("hello, world")
}
