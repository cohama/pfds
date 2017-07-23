pub mod list;
pub use self::list::{Stack, List, ListIter};

mod set;
pub use self::set::{Set};

pub mod binsearchtree;

pub mod leftistheap;

mod heap;
pub use self::heap::{Heap};

pub mod binominalheap;
pub mod binominalheap2;

pub mod explicitminheap;

pub fn run() {
    binominalheap();
}

#[derive(Eq, PartialEq)]
struct Tv {
    num: i32
}

use std::cmp::{Ordering};

impl PartialOrd for Tv {
    fn partial_cmp(&self, other: &Tv) -> Option<Ordering> {
        println!("compared {} and {} (p)", self.num, other.num);
        self.num.partial_cmp(&other.num)
    }
}

impl Ord for Tv {
    fn cmp(&self, other: &Tv) -> ::std::cmp::Ordering {
        println!("compared {} and {}", self.num, other.num);
        self.num.cmp(&other.num)
    }
}

use std::fmt;
impl fmt::Debug for Tv {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.num)
    }
}

impl fmt::Display for Tv {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.num)
    }
}

fn binominalheap() {
    use self::binominalheap2::*;
    let h1 = <BHeap<_> as Heap<_>>::empty()
        .insert(Tv{num: 1})
        .insert(Tv{num: 2})
        .insert(Tv{num: 2})
        .insert(Tv{num: 4})
    // let h1 = h0
        .insert(Tv{num: 3})
    // let h2 = <BHeap<_> as Heap<_>>::empty()
        .insert(Tv{num: 5})
        .insert(Tv{num: 6});
        // .insert(Tv{num: 7});

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
