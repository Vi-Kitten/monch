use super::*;

#[derive(Clone)]
pub struct ParseOk<T> where
    T: Clone
{
    t: T,
}

impl<T> ParseOk<T> where
    T: Clone
{
    pub fn new(t: T) -> ParseOk<T> {
        ParseOk{
            t: t
        }
    }
}

impl<I, T, E> UnsizedParser<I, T, E> for ParseOk<T> where
    I: Iterator + Clone,
    T: Clone
{
    fn parse(&self, _iter: &mut I) -> Result<T, E> {
        Ok(self.t.clone())
    }
}

#[derive(Clone)]
pub struct ParseErr<E> where
    E: Clone
{
    e: E,
}

impl<E> ParseErr<E> where
    E: Clone
{
    pub fn new(e: E) -> ParseErr<E> {
        ParseErr{
            e: e
        }
    }
}

impl<I, T, E> UnsizedParser<I, T, E> for ParseErr<E> where
    I: Iterator + Clone,
    E: Clone
{
    fn parse(&self, _iter: &mut I) -> Result<T, E> {
        Err(self.e.clone())
    }
}

//* Backtracking

#[derive(Clone)]
pub struct Attempt<P, I, T, E> where
    I: Iterator + Clone,
    P: Parser<I, T, E>
{
    parser: P,
    _i: std::marker::PhantomData<I>,
    _t: std::marker::PhantomData<T>,
    _e: std::marker::PhantomData<E>,
}

impl<P, I, T, E> Attempt<P, I, T, E> where
    I: Iterator + Clone,
    P: Parser<I, T, E>
{
    pub fn new(parser: P) -> Attempt<P, I, T, E> {
        Attempt{
            parser: parser,
            _i: std::marker::PhantomData,
            _t: std::marker::PhantomData,
            _e: std::marker::PhantomData,
        }
    }
}

impl<P, I, T, E> UnsizedParser<I, T, E> for Attempt<P, I, T, E> where
    I: Iterator + Clone,
    P: Parser<I, T, E>
{
    fn parse(&self, iter: &mut I) -> Result<T, E> {
        self.parser.attempt_parse(iter)
    }
}

#[derive(Clone)]
pub struct Scry<P, I, T, E> where
    I: Iterator + Clone,
    P: Parser<I, T, E>
{
    parser: P,
    _i: std::marker::PhantomData<I>,
    _t: std::marker::PhantomData<T>,
    _e: std::marker::PhantomData<E>,
}

impl<P, I, T, E> Scry<P, I, T, E> where
    I: Iterator + Clone,
    P: Parser<I, T, E>
{
    pub fn new(parser: P) -> Scry<P, I, T, E> {
        Scry{
            parser: parser,
            _i: std::marker::PhantomData,
            _t: std::marker::PhantomData,
            _e: std::marker::PhantomData,
        }
    }
}

impl<P, I, T, E> UnsizedParser<I, T, E> for Scry<P, I, T, E> where
    I: Iterator + Clone,
    P: Parser<I, T, E>
{
    fn parse(&self, iter: &mut I) -> Result<T, E> {
        self.parser.scry_parse(iter)
    }
}

#[derive(Clone)]
pub struct Backtrack<P, I, T, E> where
    I: Iterator + Clone,
    P: Parser<I, T, E>
{
    parser: P,
    _i: std::marker::PhantomData<I>,
    _t: std::marker::PhantomData<T>,
    _e: std::marker::PhantomData<E>,
}

impl<P, I, T, E> Backtrack<P, I, T, E> where
    I: Iterator + Clone,
    P: Parser<I, T, E>
{
    pub fn new(parser: P) -> Backtrack<P, I, T, E> {
        Backtrack{
            parser: parser,
            _i: std::marker::PhantomData,
            _t: std::marker::PhantomData,
            _e: std::marker::PhantomData,
        }
    }
}

impl<P, I, T, E> UnsizedParser<I, T, E> for Backtrack<P, I, T, E> where
    I: Iterator + Clone,
    P: Parser<I, T, E>
{
    fn parse(&self, iter: &mut I) -> Result<T, E> {
        self.parser.backtrack_parse(iter)
    }
}

//* Value mapping

#[derive(Clone)]
pub struct Map<P, I, T, E, U, F> where
    I: Iterator + Clone,
    P: Parser<I, T, E>,
    F: Fn(T) -> U
{
    parser: P, 
    inner_map: F,
    _i: std::marker::PhantomData<I>,
    _t: std::marker::PhantomData<T>,
    _e: std::marker::PhantomData<E>,
}

impl<P, I, T, E, U, F> Map<P, I, T, E, U, F> where
    I: Iterator + Clone,
    P: Parser<I, T, E>,
    F: Fn(T) -> U
{
    pub fn new(parser: P, inner_map: F) -> Map<P, I, T, E, U, F> {
        Map{
            parser: parser,
            inner_map: inner_map,
            _i: std::marker::PhantomData,
            _t: std::marker::PhantomData,
            _e: std::marker::PhantomData,
        }
    }
}

impl<P, I, T, E, U, F> UnsizedParser<I, U, E> for Map<P, I, T, E, U, F> where
    I: Iterator + Clone,
    P: Parser<I, T, E>,
    F: Fn(T) -> U
{
    fn parse(&self, iter: &mut I) -> Result<U, E> {
        self.parser.parse(iter).map(&self.inner_map)
    }
}

#[derive(Clone)]
pub struct AndThen<P, I, T, E, U, F> where
    I: Iterator + Clone,
    P: Parser<I, T, E>,
    F: Fn(T) -> Result<U, E>
{
    parser: P, 
    inner_bind: F,
    _i: std::marker::PhantomData<I>,
    _t: std::marker::PhantomData<T>,
    _e: std::marker::PhantomData<E>,
}

impl<P, I, T, E, U, F> AndThen<P, I, T, E, U, F> where
    I: Iterator + Clone,
    P: Parser<I, T, E>,
    F: Fn(T) -> Result<U, E>
{
    pub fn new(parser: P, inner_bind: F) -> AndThen<P, I, T, E, U, F> {
        AndThen{
            parser: parser,
            inner_bind: inner_bind,
            _i: std::marker::PhantomData,
            _t: std::marker::PhantomData,
            _e: std::marker::PhantomData,
        }
    }
}

impl<P, I, T, E, U, F> UnsizedParser<I, U, E> for AndThen<P, I, T, E, U, F> where
    I: Iterator + Clone,
    P: Parser<I, T, E>,
    F: Fn(T) -> Result<U, E>
{
    fn parse(&self, iter: &mut I) -> Result<U, E> {
        self.parser.parse(iter).and_then(&self.inner_bind)
    }
}

#[derive(Clone)]
pub struct AndCompose<P, I, T, E, Q, U> where
    I: Iterator + Clone,
    P: Parser<I, T, E>,
    Q: Parser<I, U, E>
{
    parser: P, 
    other: Q,
    _i: std::marker::PhantomData<I>,
    _t: std::marker::PhantomData<T>,
    _e: std::marker::PhantomData<E>,
    _u: std::marker::PhantomData<U>,
}

impl<P, I, T, E, Q, U> AndCompose<P, I, T, E, Q, U> where
    I: Iterator + Clone,
    P: Parser<I, T, E>,
    Q: Parser<I, U, E>
{
    pub fn new(parser: P, other: Q) -> AndCompose<P, I, T, E, Q, U> {
        AndCompose{
            parser: parser,
            other: other,
            _i: std::marker::PhantomData,
            _t: std::marker::PhantomData,
            _e: std::marker::PhantomData,
            _u: std::marker::PhantomData,
        }
    }
}

impl<P, I, T, E, Q, U> UnsizedParser<I, U, E> for AndCompose<P, I, T, E, Q, U> where
    I: Iterator + Clone,
    P: Parser<I, T, E>,
    Q: Parser<I, U, E>
{
    fn parse(&self, iter: &mut I) -> Result<U, E> {
        self.parser.parse(iter).and_then(|_| self.other.parse(iter))
    }
}

#[derive(Clone)]
pub struct AndThenCompose<P, I, T, E, Q, U, F> where
    I: Iterator + Clone,
    P: Parser<I, T, E>,
    Q: Parser<I, U, E>,
    F: Fn(T) -> Q
{
    parser: P, 
    bind: F,
    _i: std::marker::PhantomData<I>,
    _t: std::marker::PhantomData<T>,
    _e: std::marker::PhantomData<E>,
    _q: std::marker::PhantomData<Q>,
    _u: std::marker::PhantomData<U>,
}

impl<P, I, T, E, Q, U, F> AndThenCompose<P, I, T, E, Q, U, F> where
    I: Iterator + Clone,
    P: Parser<I, T, E>,
    Q: Parser<I, U, E>,
    F: Fn(T) -> Q
{
    pub fn new(parser: P, bind: F) -> AndThenCompose<P, I, T, E, Q, U, F> {
        AndThenCompose{
            parser: parser,
            bind: bind,
            _i: std::marker::PhantomData,
            _t: std::marker::PhantomData,
            _e: std::marker::PhantomData,
            _q: std::marker::PhantomData,
            _u: std::marker::PhantomData,
        }
    }
}

impl<P, I, T, E, Q, U, F> UnsizedParser<I, U, E> for AndThenCompose<P, I, T, E, Q, U, F> where
    I: Iterator + Clone,
    P: Parser<I, T, E>,
    Q: Parser<I, U, E>,
    F: Fn(T) -> Q
{
    fn parse(&self, iter: &mut I) -> Result<U, E> {
        (self.bind)(self.parser.parse(iter)?).parse(iter)
    }
}
