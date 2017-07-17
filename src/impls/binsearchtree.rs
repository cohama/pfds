/// 2.2 二分探索木
use std::rc::Rc;
use std::fmt;

pub trait Set<T> {
    fn empty() -> Self;
    fn member(&self, elem: &T) -> bool;
    fn insert(&self, elem: T) -> Self;
}

#[derive(Clone)]
pub enum Tree<T> {
    Empty,
    Tree(Rc<(Tree<T>, T, Tree<T>)>)
}
use self::Tree::*;

impl <T> Set<T> for Tree<T>
    where T: Ord + Clone
{
    fn empty() -> Tree<T> {Tree::Empty}
    fn member(&self, elem: &T) -> bool {
        match self {
            &Empty => false,
            &Tree(ref t) => {
                let (ref left, ref node, ref right) = **t;
                if elem < node {
                    left.member(elem)
                } else if elem > node {
                    right.member(elem)
                } else {
                    true
                }
            }
        }
    }
    fn insert(&self, elem: T) -> Tree<T> {
        match self {
            &Empty => Tree(Rc::new((Empty, elem, Empty))),
            &Tree(ref t) => {
                let (ref left, ref node, ref right) = **t;
                if &elem < node {
                    Tree(Rc::new((left.insert(elem), node.clone(), right.clone())))
                } else if &elem > node {
                    Tree(Rc::new((left.clone(), node.clone(), right.insert(elem))))
                } else {
                    self.clone()
                }
            }
        }
    }
}

impl <T> fmt::Debug for Tree<T>
    where T: fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Empty => write!(f, "E"),
            &Tree(ref t) => {
                let (ref left, ref node, ref right) = **t;
                write!(f, "T({})[{:?}, {:?}, {:?}]", Rc::strong_count(&t), node, left, right)
            }
        }
    }
}
