use std::marker::PhantomData;
use super::*;

#[derive(Clone)]
pub struct Wrap<T, E> where
    T: Clone
{
    val: T,
    _e: PhantomData<E>,
}

impl<T, E> Wrap<T, E> where
    T: Clone
{
    pub fn new(val: T) -> Wrap<T, E> {
        Wrap{
            val,
            _e: PhantomData,
        }
    }
}

impl<I, T, E> Parser<I> for Wrap<T, E> where
    I: Iterator + Clone,
    T: Clone
{
    type Value = T;
    type Error = E;
    
    fn parse(&self, _iter: &mut I) -> ParseResult<T, E> {
        ParseInfo::default().ok(self.val.clone())
    }
}

#[derive(Clone)]
pub struct Fail<T, E> where
    E: Clone
{
    err: E,
    _t: PhantomData<T>,
}

impl<T, E> Fail<T, E> where
    E: Clone
{
    pub fn new(err: E) -> Fail<T, E> {
        Fail{
            err,
            _t: PhantomData,
        }
    }
}

impl<I, T, E> Parser<I> for Fail<T, E> where
    I: Iterator + Clone,
    E: Clone
{
    type Value = T;
    type Error = E;

    fn parse(&self, _iter: &mut I) -> ParseResult<T, E> {
        ParseInfo::default().err(self.err.clone())
    }
}

#[derive(Clone)]
pub struct Arrow<F> {
    func: F,
}

impl<F> Arrow<F> {
    pub fn new(func: F) -> Arrow<F> {
        Arrow {
            func,
        }
    }
}

impl<F, I, T, E> Parser<I> for Arrow<F> where
    I: Iterator + Clone,
    F: Fn(&mut I) -> ParseResult<T, E>
{
    type Value = T;
    type Error = E;

    fn parse(&self, iter: &mut I) -> ParseResult<Self::Value, Self::Error> {
        (self.func)(iter)
    }
}