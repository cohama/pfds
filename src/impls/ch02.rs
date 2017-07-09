use std::rc::Rc;

/// 2.1 リスト

pub trait Stack<T>
    where T: Clone
{
    fn empty() -> Self;
    fn is_empty(&self) -> bool;

    fn cons(&self, x: T) -> Self;
    fn head(&self) -> &T; // panic if the stack is empty.
    fn tail(&self) -> &Self; // panic if the stack is empty.

    fn append(&self, ys: &Self) -> Self
        where Self: Clone
    {
        if self.is_empty() {
            ys.clone()
        } else {
            self.tail().append(ys).cons(self.head().clone())
        }
    }
}

#[derive(Clone)]
pub enum List<T> {
    Nil,
    Cons(Rc<(T, List<T>)>)
}
use self::List::*;

impl <T> List<T>
    where T: Clone
{
    pub fn update(&self, index: usize, value: T) -> List<T> {
        match self {
            &Nil => panic!("nil update"),
            &Cons(_) => {
                if index == 0 {
                    self.tail().cons(value)
                } else {
                    self.tail().update(index - 1, value)
                }
            }
        }
    }

    pub fn suffixes(&self) -> List<List<T>> {
        match self {
            &Nil => Nil,
            &Cons(_) => self.tail().suffixes().cons(self.clone())
        }
    }

    pub fn iter(&self) -> ListIter<T> {
        ListIter {point: self}
    }
}

pub struct ListIter<'a, T: 'a> {
    point: &'a List<T>
}

impl <'a, T> Iterator for ListIter<'a, T>
    where T: Clone
{
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        if self.point.is_empty() {
            None
        } else {
            let ret = self.point.head();
            self.point = self.point.tail();
            Some(ret)
        }
    }
}

use ::std::fmt;
impl <T> fmt::Debug for List<T>
    where T: fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Nil => write!(f, "Nil"),
            &Cons(ref xxs) => {
                // let (x, xs) = (&xxs.0, &xxs.1);
                let (ref x, ref xs) = **xxs;
                write!(f, "Cons({:?}({}), {:?})", x, Rc::strong_count(&xxs), xs)
            }
        }
    }
}

impl <T> fmt::Display for List<T>
    where T: fmt::Display + Clone
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for (i, v) in self.iter().enumerate() {
            if i != 0 { write!(f, ", ")? }
            write!(f, "{}", v)?;
        }
        write!(f, "]")
    }
}

impl <T> Stack<T> for List<T>
    where T: Clone
{
    fn empty() -> List<T> {Nil}
    fn is_empty(&self) -> bool {
        match self {
            &List::Nil => true,
            &List::Cons(_) => false
        }
    }
    fn cons(&self, x: T) -> List<T> {
        Cons(Rc::new((x, self.clone())))
    }
    fn head(&self) -> &T {
        if let &List::Cons(ref xxs) = self {
            &xxs.0
        } else {
            panic!("nil head")
        }
    }
    fn tail(&self) -> &List<T> {
        if let &List::Cons(ref xxs) = self {
            &xxs.1
        } else {
            panic!("nil tail")
        }
    }
}


/// 2.2 二分探索木

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

pub fn run() {
    // let xs = Nil.cons(1).cons(2).cons(3);
    // let ys = Nil.cons(4).cons(5).cons(6);
    // let zs = xs.append(&ys);
    // println!("{:?}", xs);
    // println!("{:?}", ys);
    // println!("{:?}", zs);
    // let xs = Nil.cons(4).cons(3).cons(2).cons(1).cons(0);
    // let ys = xs.update(2, 7);
    // println!("{}", xs);
    // println!("{:?}", xs);
    // println!("{:?}", ys);

    // let a = Nil.cons(4).cons(3).cons(2).cons(1);
    // println!("{:?}", a.suffixes());
    // println!("{}", a.suffixes());
    let xs = Tree::empty().insert('d').insert('b').insert('g').insert('a').insert('c').insert('f').insert('h');
    let ys = xs.insert('e');

    println!("{:?}", xs);
    println!("{:?}", ys);
}
