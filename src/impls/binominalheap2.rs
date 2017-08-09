use super::{Stack, List, Heap};
use super::List::*;

#[derive(Debug)]
pub struct BinominalTree<T>
    where T: Clone
{
    node: T,
    sub: List<BinominalTree<T>>
}

// Clone を自動導出にすると不必要な T: Clone が要求されてしまうので
// 手動で実装する
// http://qnighy.hatenablog.com/entry/2017/06/01/070000
// #26925
impl <T> Clone for BinominalTree<T>
    where T: Clone
{
    fn clone(&self) -> BinominalTree<T> {
        BinominalTree {
            node: self.node.clone(),
            sub: self.sub.clone()
        }
    }
}

impl <T> BinominalTree<T>
    where T: Ord + Clone
{
    pub fn link(&self, that: &BinominalTree<T>) -> BinominalTree<T> {
        if self.node < that.node {
            BinominalTree {
                node: self.node.clone(),
                sub: self.sub.snoc(that.clone())
            }
        } else {
            BinominalTree {
                node: that.node.clone(),
                sub: that.sub.snoc(self.clone())
            }
        }
    }

}

pub type BHeap<T> = (List<(i32, BinominalTree<T>)>);

fn ins_tree<T>(rank: i32, t: BinominalTree<T>, ts: &BHeap<T>) -> BHeap<T>
    where T: Ord + Clone
{
    match ts {
        &Nil => List::singleton((rank, t.clone())),
        &Cons(ref rxs) => {
            let (&(rank_, ref x), xs) = (&rxs.0, &rxs.1);
            if rank < rank_ {
                ts.snoc((rank, t))
            } else {
                ins_tree(rank + 1, t.link(x), xs)
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
        let (&(rank, ref x), xs) = ts.decom();
        if Stack::is_empty(xs) {
            (x, Nil)
        } else {
            let (y, ys) = remove_min_tree(xs);
            if x.node < y.node {
                (x, xs.clone())
            } else {
                (y, ys.snoc((rank, x.clone())))
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

impl <T> Heap<T> for BHeap<T>
    where T: Ord + Clone
{
    fn empty() -> BHeap<T> {
        Stack::empty()
    }
    fn is_empty(&self) -> bool {
        Stack::is_empty(self)
    }

    fn insert(&self, x: T) -> BHeap<T> {
        let t = BinominalTree {
            node: x,
            sub: List::Nil
        };
        ins_tree(0, t, &self)
    }

    fn merge(&self, other: &BHeap<T>) -> BHeap<T> {
        match (self, other) {
            (ts, &Nil) => ts.clone(),
            (&Nil, ts) => ts.clone(),
            (&Cons(ref rts1), &Cons(ref rts2)) => {
                let (&(rank1, ref t1), ts1) = (&rts1.0, &rts1.1);
                let (&(rank2, ref t2), ts2) = (&rts2.0, &rts2.1);
                if rank1 < rank2 {
                    ts1.merge(other).snoc((rank1, t1.clone()))
                } else if rank1 > rank2 {
                    self.merge(ts2).snoc((rank2, t2.clone()))
                } else {
                    let linked = t1.link(t2);
                    let merged = ts1.merge(ts2);
                    ins_tree(rank1 + 1, linked, &merged)
                }
            }
        }
    }

    fn find_min(&self) -> &T {
        if Stack::is_empty(self) {
            panic!("tree is empty!");
        } else {
            let (&(_, ref x), xs) = self.decom();
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
        x.sub.iter()
            .enumerate()
            .map(|(i, x)| (i as i32, x.clone()))
            .collect::<BHeap<_>>()
            .reverse()
            .merge(&xs)
    }

}
