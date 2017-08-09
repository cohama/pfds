use super::{Stack, List, Heap};
use super::List::*;

#[derive(Debug)]
pub struct BinominalTree<T>
    where T: Clone
{
    rank: i32,
    node: T,
    sub: List<BinominalTree<T>>
}

impl <T> Clone for BinominalTree<T>
    where T: Clone
{
    fn clone(&self) -> BinominalTree<T> {
        BinominalTree {
            rank: self.rank,
            node: self.node.clone(),
            sub: self.sub.clone()
        }
    }
}

impl <T> BinominalTree<T>
    where T: Ord + Clone
{
    pub fn link(&self, that: &BinominalTree<T>) -> BinominalTree<T> {
        assert_eq!(self.rank, that.rank);
        if self.node < that.node {
            BinominalTree {
                rank: self.rank + 1,
                node: self.node.clone(),
                sub: self.sub.snoc(that.clone())
            }
        } else {
            BinominalTree {
                rank: self.rank + 1,
                node: that.node.clone(),
                sub: that.sub.snoc(self.clone())
            }
        }
    }

}

pub type BHeap<T> = List<BinominalTree<T>>;

fn ins_tree<T>(t: BinominalTree<T>, ts: &List<BinominalTree<T>>) -> BHeap<T>
    where T: Ord + Clone
{
    match ts {
        &Nil => List::singleton(t.clone()),
        &Cons(ref rxs) => {
            let (x, xs) = (&rxs.0, &rxs.1);
            if t.rank < x.rank {
                ts.snoc(t)
            } else {
                ins_tree(t.link(x), xs)
            }
        }
    }
}

fn remove_min_tree<T>(ts: &BHeap<T>) -> (&BinominalTree<T>, BHeap<T>)
    where T: Ord + Clone
{
    if Stack::is_empty(ts) {
        panic!("remove from empty tree");
    } else {
        let (x, xs) = ts.decom();
        if Stack::is_empty(xs) {
            (x, Nil)
        } else {
            let (y, ys) = remove_min_tree(xs);
            if x.node < y.node {
                (x, xs.clone())
            } else {
                (y, ys.snoc(x.clone()))
            }
        }
    }
    // match ts {
    //     &Nil => panic!("remove from empty tree"),
    //     &Cons(ref rts) => {
    //         let (x, xs) = (&rts.0, &rts.1);
    //         match xs {
    //             &Nil => (x, Nil),
    //             &Cons(_) => {
    //                 let (y, ys) = remove_min_tree(xs);
    //                 if x.node < y.node {
    //                     (x, xs.clone())
    //                 } else {
    //                     (y, ys.snoc(x.clone()))
    //                 }
    //             }
    //         }
    //     }
    // }
}

impl <T> BHeap<T>
    where T: Ord + Clone
{
    #[allow(dead_code)]
    fn find_min_naive(&self) -> &T {
        &remove_min_tree(self).0.node
    }
}

impl <T> Heap<T> for BHeap<T>
    where T: Ord + Clone + ::std::fmt::Debug
{
    fn empty() -> BHeap<T> {
        Stack::empty()
    }
    fn is_empty(&self) -> bool {
        Stack::is_empty(self)
    }

    fn insert(&self, x: T) -> BHeap<T> {
        let t = BinominalTree {
            rank: 0,
            node: x,
            sub: List::Nil
        };
        ins_tree(t, &self)
    }

    fn merge(&self, other: &BHeap<T>) -> BHeap<T> {
        match (self, other) {
            (ts, &Nil) => ts.clone(),
            (&Nil, ts) => ts.clone(),
            (&Cons(ref rts1), &Cons(ref rts2)) => {
                let (t1, ts1) = (&rts1.0, &rts1.1);
                let (t2, ts2) = (&rts2.0, &rts2.1);
                if t1.rank < t2.rank {
                    ts1.merge(other).snoc(t1.clone())
                } else if t1.rank > t2.rank {
                    self.merge(ts2).snoc(t2.clone())
                } else {
                    let linked = t1.link(t2);
                    let merged = ts1.merge(ts2);
                    ins_tree(linked, &merged)
                }
            }
        }
    }

    fn find_min(&self) -> &T {
        if Stack::is_empty(self) {
            panic!("tree is empty!");
        } else {
            let (x, xs) = self.decom();
            if Stack::is_empty(xs) {
                &x.node
            } else {
                let y = xs.find_min();
                ::std::cmp::min(&x.node, y)
            }
        }
    }

    fn delete_min(&self) -> BHeap<T> {
        let (x, xs) = remove_min_tree(self);
        x.sub.reverse().merge(&xs)
    }

}
