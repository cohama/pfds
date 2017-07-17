use std::rc::Rc;
use std::iter::FromIterator;

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

    pub fn iter(&self) -> ListIter<T> {
        ListIter {point: self}
    }

    pub fn singleton(x: T) -> List<T> {
        Cons(Rc::new((x, List::empty())))
    }

    pub fn decom(&self) -> (&T, &List<T>) {
        (self.head(), self.tail())
    }

    pub fn reverse(&self) -> List<T> {
        self.iter().fold(Nil, |acc, x| {
            acc.snoc(x.clone())
        })
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

