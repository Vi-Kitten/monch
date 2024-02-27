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
    
    fn parse(&self, _iter: &mut I, _info: &mut ParseInfo) -> Result<T, E> {
        Ok(self.val.clone())
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

    fn parse(&self, _iter: &mut I, _info: &mut ParseInfo) -> Result<T, E> {
        Err(self.err.clone())
    }
}

#[derive(Clone)]
pub struct ExpectTokens<T, const N: usize> {
    expect: [T; N],
}

impl<T, const N: usize> ExpectTokens<T, N> {
    pub fn new(expect: [T; N]) -> ExpectTokens<T, N> {
        ExpectTokens {
            expect,
        }
    }
}

impl<I, T, const N: usize> Parser<I> for ExpectTokens<T, N> where
    I: Iterator<Item=T> + Clone,
    T: Eq + Clone,
{
    type Value = [T; N];
    type Error = [T; N];

    fn parse(&self, iter: &mut I, info: &mut ParseInfo) -> Result<[T; N], [T; N]> {
        info.read += N;
        for expected_token in &self.expect {
            iter.next()
                .filter(|found_token| {
                    info.taken += 1;
                    found_token == expected_token
                })
                .ok_or_else(|| self.expect.clone())?;
        };
        Ok(self.expect.clone())
    }
}

#[derive(Clone)]
pub struct Region<S, B>{
    name: String,
    start: S,
    body: B,
}

impl<S, B> Region<S, B> {
    pub fn new(name: String, start: S, body: B) -> Region<S, B> {
        Region {
            name,
            start,
            body,
        }
    }
}

impl<I, S, B> Parser<I> for Region<S, B> where
    I: Iterator + Clone,
    S: Parser<I>,
    B: Parser<I>
{
    type Value = B::Value;
    type Error = Result<super::errors::BranchInternalError<B::Error>, super::errors::BranchEntryError<S::Error>>;

    fn parse(&self, iter: &mut I, info: &mut ParseInfo) -> Result<Self::Value, Self::Error> {
        self.start
            .parse(iter, info)
            .map_err(|err|
                Result::Err(super::errors::BranchEntryError(self.name.clone(), err))
            )
            .and_then(|_| self.body
                .parse(iter, info)
                .map_err(|err|
                    Result::Ok(super::errors::BranchInternalError(self.name.clone(), err))
                )
            )
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
    F: Fn(&mut I, &mut ParseInfo) -> Result<T, E>
{
    type Value = T;
    type Error = E;

    fn parse(&self, iter: &mut I, info: &mut ParseInfo) -> Result<Self::Value, Self::Error> {
        (self.func)(iter, info)
    }
}