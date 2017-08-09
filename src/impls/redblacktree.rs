use std::rc::Rc;
use std::fmt;
use std::cmp::Ordering::*;
use std::boxed::FnBox;

use super::{Set, Stack, List};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Color {
    Red,
    Black
}

#[derive(Clone)]
pub enum RedBlackTree<T> {
    Empty,
    Node(Color, Rc<RedBlackTree<T>>, T, Rc<RedBlackTree<T>>)
}

use self::Color::*;
use self::RedBlackTree::*;

impl <T> fmt::Debug for RedBlackTree<T>
    where T: fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Empty => write!(f, "E"),
            &Node(color, ref left, ref val, ref right) => {
                let c = if color == Red { "R" } else { "B" };
                write!(f, "T({}, {:?}, {:?}, {:?})", c, left, val, right)
            }
        }
    }
}

enum TailRec<'a, T> {
    Val(T),
    Func(Box<FnBox() -> TailRec<'a, T> + 'a>)
}

impl <'a, T> TailRec<'a, T> {
    fn run(self) -> T {
        match self {
            TailRec::Val(x) => x,
            TailRec::Func(f) => f().run()
        }
    }
}

impl <T> RedBlackTree<T>
    where T: Clone + Ord
{

    #[allow(dead_code)]
    fn balance(color: Color, left: &Rc<RedBlackTree<T>>, val: &T, right: &Rc<RedBlackTree<T>>) -> RedBlackTree<T> {
        RedBlackTree::balance_impl(color, left, val, right)
            .unwrap_or(Node(color, left.clone(), val.clone(), right.clone()))
    }

    #[allow(dead_code)]
    fn balance_impl(color: Color, left: &Rc<RedBlackTree<T>>, val: &T, right: &Rc<RedBlackTree<T>>) -> Option<RedBlackTree<T>> {
        if color == Red { return None }
        if let &Node(Red, ref ll, ref lv, ref lr) = left.as_ref() {
            if let &Node(Red, ref lll, ref llv, ref llr) = ll.as_ref() {
                Some(Node(Red,
                          Rc::new(Node(Black, lll.clone(), llv.clone(), llr.clone())),
                          lv.clone(),
                          Rc::new(Node(Black, lr.clone(), val.clone(), right.clone()))
                         ))
            } else if let &Node(Red, ref lrl, ref lrv, ref lrr) = lr.as_ref() {
                Some(Node(Red,
                          Rc::new(Node(Black, ll.clone(), lv.clone(), lrl.clone())),
                          lrv.clone(),
                          Rc::new(Node(Black, lrr.clone(), val.clone(), right.clone()))
                         ))
            } else {
                None
            }
        } else if let &Node(Red, ref rl, ref rv, ref rr) = right.as_ref() {
            if let &Node(Red, ref rll, ref rlv, ref rlr) = rl.as_ref() {
                Some(Node(Red,
                          Rc::new(Node(Black, left.clone(), val.clone(), rll.clone())),
                          rlv.clone(),
                          Rc::new(Node(Black, rlr.clone(), rv.clone(), rr.clone()))
                         ))
            } else if let &Node(Red, ref rrl, ref rrv, ref rrr) = rr.as_ref() {
                Some(Node(Red,
                          Rc::new(Node(Black, left.clone(), val.clone(), rl.clone())),
                          rv.clone(),
                          Rc::new(Node(Black, rrl.clone(), rrv.clone(), rrr.clone()))
                         ))
            } else {
                None
            }
        } else {
            None
        }
    }

    #[allow(dead_code)]
    fn lbalance(color: Color, left: &Rc<RedBlackTree<T>>, val: &T, right: &Rc<RedBlackTree<T>>) -> RedBlackTree<T> {
        if color == Red {
            if let &Node(Red, ref ll, ref lv, ref lr) = left.as_ref() {
                if let &Node(Red, ref lll, ref llv, ref llr) = ll.as_ref() {
                    return Node(Red,
                                Rc::new(Node(Black, lll.clone(), llv.clone(), llr.clone())),
                                lv.clone(),
                                Rc::new(Node(Black, lr.clone(), val.clone(), right.clone())))
                } else if let &Node(Red, ref lrl, ref lrv, ref lrr) = lr.as_ref() {
                    return Node(Red,
                                Rc::new(Node(Black, ll.clone(), lv.clone(), lrl.clone())),
                                lrv.clone(),
                                Rc::new(Node(Black, lrr.clone(), val.clone(), right.clone())))
                }
            }
        }
        Node(color, left.clone(), val.clone(), right.clone())
    }

    #[allow(dead_code)]
    fn rbalance(color: Color, left: &Rc<RedBlackTree<T>>, val: &T, right: &Rc<RedBlackTree<T>>) -> RedBlackTree<T> {
        if color == Red {
            if let &Node(Red, ref rl, ref rv, ref rr) = right.as_ref() {
                if let &Node(Red, ref rll, ref rlv, ref rlr) = rl.as_ref() {
                    return Node(Red,
                                Rc::new(Node(Black, left.clone(), val.clone(), rll.clone())),
                                rlv.clone(),
                                Rc::new(Node(Black, rlr.clone(), rv.clone(), rr.clone())))
                } else if let &Node(Red, ref rrl, ref rrv, ref rrr) = rr.as_ref() {
                    return Node(Red,
                                Rc::new(Node(Black, left.clone(), val.clone(), rl.clone())),
                                rv.clone(),
                                Rc::new(Node(Black, rrl.clone(), rrv.clone(), rrr.clone())))
                }
            }
        }
        Node(color, left.clone(), val.clone(), right.clone())
    }

    fn llbalance(color: Color, left: &Rc<RedBlackTree<T>>, val: &T, right: &Rc<RedBlackTree<T>>) -> RedBlackTree<T> {
        if color == Red {
            if let &Node(Red, ref ll, ref lv, ref lr) = left.as_ref() {
                if let &Node(Red, ref lll, ref llv, ref llr) = ll.as_ref() {
                    return Node(Red,
                                Rc::new(Node(Black, lll.clone(), llv.clone(), llr.clone())),
                                lv.clone(),
                                Rc::new(Node(Black, lr.clone(), val.clone(), right.clone())))
                }
            }
        }
        Node(color, left.clone(), val.clone(), right.clone())
    }

    fn lrbalance(color: Color, left: &Rc<RedBlackTree<T>>, val: &T, right: &Rc<RedBlackTree<T>>) -> RedBlackTree<T> {
        if color == Red {
            if let &Node(Red, ref ll, ref lv, ref lr) = left.as_ref() {
                if let &Node(Red, ref lrl, ref lrv, ref lrr) = lr.as_ref() {
                    return Node(Red,
                                Rc::new(Node(Black, ll.clone(), lv.clone(), lrl.clone())),
                                lrv.clone(),
                                Rc::new(Node(Black, lrr.clone(), val.clone(), right.clone())))
                }
            }
        }
        Node(color, left.clone(), val.clone(), right.clone())
    }

    fn rlbalance(color: Color, left: &Rc<RedBlackTree<T>>, val: &T, right: &Rc<RedBlackTree<T>>) -> RedBlackTree<T> {
        if color == Red {
            if let &Node(Red, ref rl, ref rv, ref rr) = right.as_ref() {
                if let &Node(Red, ref rll, ref rlv, ref rlr) = rl.as_ref() {
                    return Node(Red,
                                Rc::new(Node(Black, left.clone(), val.clone(), rll.clone())),
                                rlv.clone(),
                                Rc::new(Node(Black, rlr.clone(), rv.clone(), rr.clone())))
                }
            }
        }
        Node(color, left.clone(), val.clone(), right.clone())
    }

    fn rrbalance(color: Color, left: &Rc<RedBlackTree<T>>, val: &T, right: &Rc<RedBlackTree<T>>) -> RedBlackTree<T> {
        if color == Red {
            if let &Node(Red, ref rl, ref rv, ref rr) = right.as_ref() {
                if let &Node(Red, ref rrl, ref rrv, ref rrr) = rr.as_ref() {
                    return Node(Red,
                                Rc::new(Node(Black, left.clone(), val.clone(), rl.clone())),
                                rv.clone(),
                                Rc::new(Node(Black, rrl.clone(), rrv.clone(), rrr.clone())))
                }
            }
        }
        Node(color, left.clone(), val.clone(), right.clone())
    }

    pub fn singleton(x: T) -> RedBlackTree<T> {
        Node(Black, Rc::new(Empty), x, Rc::new(Empty))
    }

    fn singleton_c(x: T, color: Color) -> RedBlackTree<T> {
        Node(color, Rc::new(Empty), x, Rc::new(Empty))
    }

    fn set_root_color(&self, color: Color) -> RedBlackTree<T> {
        if let &Node(_, ref left, ref val, ref right) = self {
            Node(color, left.clone(), val.clone(), right.clone())
        } else {
            Empty
        }
    }

    pub fn from_sorted_list(xs: List<T>) -> RedBlackTree<T> {
        fn trampoline<'a, T>(xs: List<T>,
                             n: usize,
                             k: Box<FnBox(RedBlackTree<T>, i32, List<T>) -> TailRec<'a, RedBlackTree<T>> + 'a>)
            -> TailRec<'a, RedBlackTree<T>>
            where T: Clone + Ord + 'a
        {
            match n {
                0 => k(Empty, 0, xs),
                1 => {
                    let (x, xs) = xs.to_cons();
                    k(RedBlackTree::singleton(x), 1, xs)
                },
                _ => {
                    TailRec::Func(box move || {
                        trampoline(xs, n/2, box move |ltree: RedBlackTree<T>, lcount: i32, rest: List<T>| {
                            let (x, xs) = rest.to_cons();
                            TailRec::Func(box move || {
                                trampoline(xs.clone(), n - n/2 -1, box move |rtree: RedBlackTree<T>, rcount: i32, rest: List<T>| {
                                    k(Node(Black,
                                           Rc::new(if lcount > rcount {ltree.set_root_color(Red)} else {ltree.clone()}),
                                           x,
                                           Rc::new(rtree.clone())
                                    ), rcount + 1, rest)
                                })
                            })
                        })
                    })
                }
            }
        }
        let n = xs.count();
        trampoline(xs, n, box |t, _, _| TailRec::Val(t)).run()
    }

    pub fn from_sorted_list_cps(xs: List<T>) -> RedBlackTree<T> {
        fn walk_cps<'a, T>(xs: List<T>,
                           n: usize,
                           k: Box<FnBox(RedBlackTree<T>, i32, List<T>) -> RedBlackTree<T> + 'a>)
            -> RedBlackTree<T>
            where T: Clone + Ord,
        {
            match n {
                0 => k(Empty, 0, xs),
                1 => {
                    let (x, xs) = xs.to_cons();
                    k(RedBlackTree::singleton(x), 1, xs)
                },
                _ => {
                    walk_cps(xs, n/2, box move |ltree: RedBlackTree<T>, lcount: i32, rest: List<T>| {
                        let (x, xs) = rest.decom();
                        walk_cps(xs.clone(), n - n/2 -1, box move |rtree: RedBlackTree<T>, rcount: i32, rest: List<T>| {
                            k(Node(
                                Black,
                                Rc::new(if lcount > rcount {ltree.set_root_color(Red)} else {ltree.clone()}),
                                x.clone(),
                                Rc::new(rtree.clone())
                            ), rcount + 1, rest)
                        })
                    })
                }
            }
        }
        let n = xs.count();
        walk_cps(xs, n, box |t, _, _| t)
    }

    pub fn from_sorted_list_so(xs: List<T>) -> RedBlackTree<T> {
        fn walk<T>(xs: List<T>, n: usize) -> (RedBlackTree<T>, i32, List<T>)
            where T: Clone + Ord
        {
            match n {
                0 => (Empty, 0, xs),
                1 => {
                    let (x, xs) = xs.to_cons();
                    (RedBlackTree::singleton(x), 1, xs)
                },
                _ => {
                    let (ltree, lcount, rest) = walk(xs, n/2);
                    let (x, xs) = rest.to_cons();
                    let (rtree, rcount, rest) = walk(xs, n - n/2 -1);
                    (Node(
                        Black,
                        Rc::new(if lcount > rcount {ltree.set_root_color(Red)} else {ltree}),
                        x,
                        Rc::new(rtree)
                    ), rcount + 1, rest)
                }
            }
        }
        let n = xs.count();
        walk(xs, n).0
    }

    pub fn from_sorted_list_naive(xs: List<T>) -> RedBlackTree<T> {
        fn walk<T>(xs: List<T>, n: usize) -> (RedBlackTree<T>, i32)
            where T: Clone + Ord
        {
            match n {
                0 => (Empty, 0),
                1 => (RedBlackTree::singleton_c(xs.head().clone(), Red), 0),
                2 => {
                    let x0 = xs.head().clone();
                    let x1 = xs.tail().head().clone();
                    (Node(
                        Black,
                        Rc::new(Node(Red, Rc::new(Empty), x0, Rc::new(Empty) )),
                        x1,
                        Rc::new(Empty)
                    ), 1)
                },
                3 => {
                    let x0 = xs.head().clone();
                    let x1 = xs.tail().head().clone();
                    let x2 = xs.tail().tail().head().clone();
                    (Node(
                        Red,
                        Rc::new(Node(
                            Black,
                            Rc::new(Empty),
                            x0,
                            Rc::new(Empty)
                        )),
                        x1,
                        Rc::new(Node(
                            Black,
                            Rc::new(Empty),
                            x2,
                            Rc::new(Empty)
                        ))
                    ), 1)
                }
                _ => {
                    let (heads, roottail) = xs.split_at(n/2);
                    let root = roottail.head();
                    let tail = roottail.tail();
                    let (ltree, lbc) = walk(heads, n/2);
                    let (rtree, rbc) = walk(tail.clone(), n - n/2 - 1);
                    if n%2 == 1 {
                        (Node(
                            Red,
                            Rc::new(ltree.set_root_color(Black)),
                            root.clone(),
                            Rc::new(rtree.set_root_color(Black))
                        ), if (n/2)%2 == 1 {lbc + 1} else {lbc})
                    } else if lbc == rbc {
                        (Node(
                            Black,
                            Rc::new(ltree.clone()),
                            root.clone(),
                            Rc::new(rtree.clone())
                        ), lbc + 1)
                    } else {
                        if (n/2)%2 == 1 {
                            (Node(
                                Black,
                                Rc::new(ltree.set_root_color(Black)),
                                root.clone(),
                                Rc::new(rtree.clone())
                            ), rbc + 1)
                        } else {
                            (Node(
                                Black,
                                Rc::new(ltree.clone()),
                                root.clone(),
                                Rc::new(rtree.set_root_color(Black))
                            ), lbc + 1)
                        }
                    }
                }
            }
        }
        let n = xs.count();
        walk(xs, n).0
    }

    pub fn from_sorted_iter<I>(iter: I) -> RedBlackTree<T>
        where I: IntoIterator<Item=T>
    {
        iter.into_iter().fold(Empty, |tree, x| {
            tree.insert(x)
        })
    }

    pub fn insert_naive(&self, elem: T) -> Self {
        fn ins<T: Clone + Ord>(tree: &RedBlackTree<T>, elem: T) -> RedBlackTree<T> {
            match tree {
                &Empty => Node(Red, Rc::new(Empty), elem, Rc::new(Empty)),
                &Node(color, ref left, ref val, ref right) => {
                    match elem.cmp(val) {
                        Less =>  RedBlackTree::balance(color, &Rc::new(ins(left, elem)), val, right),
                        Greater => RedBlackTree::balance(color, left, val, &Rc::new(ins(right, elem))),
                        Equal => tree.clone()
                    }
                }
            }
        }
        if let Node(_, left, val, right) = ins(self, elem) {
            Node(Black, left, val, right)
        } else {
            panic!("ins result is empty.")
        }
    }
}

impl <T> RedBlackTree<T>
    where T: Clone + Ord + fmt::Debug
{
    fn black_count(&self) -> Result<i32, String> {
        match self {
            &Empty => Ok(0),
            &Node(color, ref left, ref val, ref right) => {
                let lbc = left.black_count()?;
                let rbc = right.black_count()?;
                if lbc != rbc { return Err(format!("Unbalanced! On node {:?}, left black count is {} but right is {}.", val, lbc, rbc)) }
                if color == Red {
                    Ok(lbc)
                } else {
                    Ok(lbc + 1)
                }
            }
        }
    }
    pub fn self_check(&self) -> Result<(), String>
        where T: ::std::fmt::Debug
    {
        match self {
            &Empty => Ok(()),
            &Node(color, ref left, ref val, ref right) => {
                if let &Node(lcolor, _, ref lval, _) = left.as_ref() {
                    if val < lval { return Err(format!("Not btree! left: {:?} is greater than parent {:?}.", lval, val)) }
                    if color == Red && lcolor == Red { return Err(format!("Color error! Both node {:?} and right {:?} is Red.", val, lval)) }
                }
                if let &Node(rcolor, _, ref rval, _) = right.as_ref() {
                    if rval < val { return Err(format!("Not btree! right: {:?} is less than parent {:?}.", rval, val)) }
                    if color == Red && rcolor == Red { return Err(format!("Color error! Both node {:?} and right {:?} is Red.", val, rval)) }
                }
                let _ = self.black_count()?;
                let _ = left.self_check()?;
                let _ = right.self_check()?;
                Ok(())
            }
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Direction {Left, Right, Root}

impl <T> Set<T> for RedBlackTree<T>
    where T: Clone + Ord
{
    fn empty() -> Self {
        Empty
    }

    fn member(&self, elem: &T) -> bool {
        match self {
            &Empty => false,
            &Node(_, ref left, ref val, ref right) => {
                match elem.cmp(val) {
                    Less => left.member(elem),
                    Greater => right.member(elem),
                    Equal => true
                }
            }
        }
    }

    fn insert(&self, elem: T) -> Self {
        fn ins<T: Clone + Ord>(tree: &RedBlackTree<T>, elem: T, d: Direction) -> RedBlackTree<T> {
            println!("d: {:?}", d);
            match tree {
                &Empty => Node(Red, Rc::new(Empty), elem, Rc::new(Empty)),
                &Node(color, ref left, ref val, ref right) => {
                    match elem.cmp(val) {
                        Less => {
                            let left = ins(left, elem, Direction::Left);
                            if d == Direction::Left {
                                RedBlackTree::llbalance(color, &Rc::new(left), val, right)
                            } else if d == Direction::Right {
                                RedBlackTree::lrbalance(color, &Rc::new(left), val, right)
                            } else {
                                Node(color, Rc::new(left), val.clone(), right.clone())
                            }
                        },
                        Greater => {
                            let right = ins(right, elem, Direction::Right);
                            if d == Direction::Left {
                                RedBlackTree::lbalance(color, left, val, &Rc::new(right))
                            } else if d == Direction::Right {
                                RedBlackTree::rrbalance(color, left, val, &Rc::new(right))
                            } else {
                                Node(color, left.clone(), val.clone(), Rc::new(right))
                            }
                        },
                        Equal => tree.clone(),
                    }
                }
            }
        }
        if let Node(_, left, val, right) = ins(self, elem, Direction::Root) {
            Node(Black, left, val, right)
        } else {
            panic!("ins result is empty.")
        }
    }
}
