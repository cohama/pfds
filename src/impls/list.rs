use std::rc::Rc;
use std::iter::{FromIterator};

/// 2.1 リスト

pub trait Stack<T>
    where T: Clone
{
    fn empty() -> Self;
    fn is_empty(&self) -> bool;

    fn snoc(&self, x: T) -> Self;
    fn head(&self) -> &T; // panic if the stack is empty.
    fn tail(&self) -> &Self; // panic if the stack is empty.

    fn append(&self, ys: &Self) -> Self
        where Self: Clone
    {
        if self.is_empty() {
            ys.clone()
        } else {
            self.tail().append(ys).snoc(self.head().clone())
        }
    }
}

// List の Clone セマンティクス的にはただ内部の参照カウントを増加
// させるだけで、実際にメモリコピーが行われるわけではない
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
                    self.tail().snoc(value)
                } else {
                    self.tail().update(index - 1, value)
                }
            }
        }
    }

    pub fn suffixes(&self) -> List<List<T>> {
        match self {
            &Nil => Nil,
            &Cons(_) => self.tail().suffixes().snoc(self.clone())
        }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {point: self}
    }

    pub fn singleton(x: T) -> List<T> {
        Cons(Rc::new((x, List::empty())))
    }

    pub fn decom(&self) -> (&T, &List<T>) {
        (self.head(), self.tail())
    }

    pub fn to_cons(&self) -> (T, List<T>) {
        (self.head().clone(), self.tail().clone())
    }

    pub fn reverse(&self) -> List<T> {
        self.iter().fold(Nil, |acc, x| {
            acc.snoc(x.clone())
        })
    }

    pub fn count(&self) -> usize {
        match self {
            &Nil => 0,
            &Cons(ref rx) => 1 + rx.1.count()
        }
    }

    pub fn split_at(&self, n: usize) -> (List<T>, List<T>) {
        fn walk<U>(xs: &List<U>, n: usize, heads: List<U>) -> (List<U>, List<U>)
            where U: Clone
        {
            match (xs, n) {
                (&Nil, _) | (_, 0) => (heads.reverse(), xs.clone()),
                (&Cons(ref rx), n) => {
                    let heads = heads.snoc(rx.0.clone());
                    walk(&rx.1, n-1, heads)
                }
            }
        }
        walk(self, n, Nil)
    }

}

pub struct Iter<'a, T: 'a> {
    point: &'a List<T>
}

impl <'a, T> Iterator for Iter<'a, T>
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

pub struct IntoIter<T> {
    vec: ::std::vec::IntoIter<T>
}

impl <T> Iterator for IntoIter<T>
    where T: Clone
{
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.vec.next()
    }
}

impl <T> IntoIterator for List<T>
    where T: Clone
{
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> IntoIter<T> {
        let vec = self.iter()
            .map(|x|x.clone())
            .collect::<Vec<T>>()
            .into_iter();
        IntoIter {vec}
    }
}

impl <T> FromIterator<T> for List<T>
    where T: Clone
{
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> List<T> {
        iter.into_iter().fold(List::empty(), |xs, x| {
            xs.snoc(x)
        })
    }
}

use ::std::fmt;
impl <T> fmt::Debug for List<T>
    where T: fmt::Debug + Clone
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for (i, v) in self.iter().enumerate() {
            if i != 0 { write!(f, ", ")? }
            write!(f, "{:?}", v)?;
        }
        write!(f, "]")
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
    fn snoc(&self, x: T) -> List<T> {
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

