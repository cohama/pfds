extern crate stopwatch;

pub mod list;
pub use self::list::{Stack, List, Iter, IntoIter};

mod set;
pub use self::set::{Set};

pub mod binsearchtree;

pub mod leftistheap;

mod heap;
pub use self::heap::{Heap};

pub mod binominalheap;
pub mod binominalheap2;

pub mod explicitminheap;

pub mod redblacktree;

// use self::stopwatch::Stopwatch;

pub fn run() {
    redblacktree();
}

use std::cell::RefCell;

#[derive(Debug)]
struct TvBuilder {
    cmp_counter: RefCell<i32>
}

#[derive(Clone)]
struct Tv<'a> {
    num: i32,
    _builder: &'a TvBuilder
}

impl TvBuilder {
    fn new() -> TvBuilder {
        TvBuilder {cmp_counter: RefCell::new(0)}
    }
    fn v(&self, num: i32) -> Tv {
        Tv {num, _builder: self}
    }
    fn inc_counter(&self) {
        *self.cmp_counter.borrow_mut() += 1;
    }
}

impl <'a> PartialEq for Tv<'a> {
    fn eq(&self, other: &Tv) -> bool {
        self.num == other.num
    }
}

impl <'a> Eq for Tv<'a> {}

use std::cmp::{Ordering};

impl <'a> PartialOrd for Tv<'a> {
    fn partial_cmp(&self, other: &Tv) -> Option<Ordering> {
        self._builder.inc_counter();
        self.num.partial_cmp(&other.num)
    }
}

impl <'a> Ord for Tv<'a> {
    fn cmp(&self, other: &Tv) -> ::std::cmp::Ordering {
        self._builder.inc_counter();
        self.num.cmp(&other.num)
    }
}

use std::fmt;
impl <'a> fmt::Debug for Tv<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.num)
    }
}

impl <'a> fmt::Display for Tv<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.num)
    }
}

#[allow(dead_code)]
fn binominalheap() {
    use self::binominalheap2::*;
    let tb = TvBuilder::new();
    let h1 = <BHeap<_> as Heap<_>>::empty()
        .insert(tb.v(1))
        .insert(tb.v(2))
        .insert(tb.v(2))
        .insert(tb.v(4))
    // let h1 = h0
        .insert(tb.v(3))
    // let h2 = <BHeap<_> as Heap<_>>::empty()
        .insert(tb.v(5))
        .insert(tb.v(6));
        // .insert(Tv::new(7));

    // println!("{:?}", h0);
    println!("{:?}", h1);
    let h2 = h1.delete_min();
    println!("{:?}", h2);
    // println!("{:?}", h1.merge(&h2));
    // println!("{}", h1.find_min());
    // println!();
    // println!("{:?}", h2);
    // println!("{}", h2.find_min());

}

fn redblacktree() {
    #![allow(unused_imports)]
    use self::redblacktree::*;
    use self::redblacktree::RedBlackTree::*;
    use self::redblacktree::Color::*;
    use ::std::rc::Rc;
    // let tb = TvBuilder::new();

    // let t = Node(
    //     Black,
    //     Rc::new(Node(Red, Rc::new(Empty), 10, Rc::new(Empty))),
    //     20,
    //     Rc::new(Node(Red, Rc::new(Node( Red, Rc::new(Empty), 30, Rc::new(Empty))), 22, Rc::new(Empty)))
    // );
    // println!("balanced?: {:?}", t.self_check());

    let tt = RedBlackTree::empty()
        .insert(1)
        .insert(2)
        .insert(3);
        // .insert(4)
        // .insert(5)
        // .insert(6)
        // .insert(7)
        // .insert(8)
        // .insert(9)
        // .insert(10)
        // .insert(11);
    println!("{:?}", tt);
    println!("balanced?: {:?}", tt.self_check());

    println!("  n   |   cmp   | elapsed ms");
    // for j in 1..101 {
    //     tb.reset_counter();
    //     // let n = 100 * (j - 1);
    //     let n = j;
    //     let mut xs = List::Nil;
    //     print!("list constructing..  ");
    //     for i in (1..n*100).rev() {
    //         xs = xs.snoc(i);
    //     }
    //     print!("ok  ");
    //     let mut sw = Stopwatch::start_new();
    //     let t1 = RedBlackTree::from_sorted_list(xs.clone());
    //     sw.stop();
    //     let cmp = *tb.cmp_counter.borrow();
    //     let cmpf = cmp as f64;
    //     println!("{:>5} | {:>7} | {}", n, cmpf, sw.elapsed_ms());

    //     if let Err(s) = t1.self_check() {
    //         println!("unbalanced!: {:?}", s);
    //         println!("  list: {:?}", xs);
    //         println!("  tree: {:?}", t1);
    //         panic!();
    //     }
    // }
    // let t1 = RedBlackTree::<Tv>::empty()
    //     .insert(tb.v(1))
    //     .insert(tb.v(2))
    //     .insert(tb.v(3))
    //     .insert(tb.v(4))
    //     .insert(tb.v(5))
    //     .insert(tb.v(6));
    // let t2 = t1.insert(tb.v(7));
    // println!("{:?}", t1);
    // println!("{:?}", t2);
}
