use std::cell::OnceCell;
use super::*;

#[derive(Clone)]
pub struct Wrap<T> where
    T: Clone
{
    val: T,
}

impl<T> Wrap<T> where
    T: Clone
{
    pub fn new(val: T) -> Wrap<T> {
        Wrap{
            val,
        }
    }
}

impl<I, T, E> Parser<I, T, E> for Wrap<T> where
    I: Iterator + Clone,
    T: Clone
{
    fn parse(&self, _iter: &mut I) -> Result<T, E> {
        Ok(self.val.clone())
    }
}

#[derive(Clone)]
pub struct Fail<E> where
    E: Clone
{
    err: E,
}

impl<E> Fail<E> where
    E: Clone
{
    pub fn new(err: E) -> Fail<E> {
        Fail{
            err,
        }
    }
}

impl<I, T, E> Parser<I, T, E> for Fail<E> where
    I: Iterator + Clone,
    E: Clone
{
    fn parse(&self, _iter: &mut I) -> Result<T, E> {
        Err(self.err.clone())
    }
}

#[derive(Clone)]
pub struct Lense<P, I, T, E, J, F> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>,
    J: Iterator + Clone,
    F: Fn(&mut J) -> &mut I
{
    parser: P,
    lense: F,
    _i: std::marker::PhantomData<I>,
    _t: std::marker::PhantomData<T>,
    _e: std::marker::PhantomData<E>,
    _j: std::marker::PhantomData<J>,
}

impl<P, I, T, E, J, F> Lense<P, I, T, E, J, F> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>,
    J: Iterator + Clone,
    F: Fn(&mut J) -> &mut I
{
    pub fn new(parser: P, lense: F) -> Lense<P, I, T, E, J, F> {
        Lense {
            parser,
            lense,
            _i: std::marker::PhantomData,
            _t: std::marker::PhantomData,
            _e: std::marker::PhantomData,
            _j: std::marker::PhantomData,
        }
    }
}

impl<P, I, T, E, J, F> Parser<J, T, E> for Lense<P, I, T, E, J, F> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>,
    J: Iterator + Clone,
    F: Fn(&mut J) -> &mut I
{
    fn parse(&self, jter: &mut J) -> Result<T, E> {        
        let mut iter: &mut I = (self.lense)(jter);
        self.parser.parse(&mut iter)
    }
}

// Backtracking

#[derive(Clone)]
pub struct Attempt<P, I, T, E> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>
{
    parser: P,
    _i: std::marker::PhantomData<I>,
    _t: std::marker::PhantomData<T>,
    _e: std::marker::PhantomData<E>,
}

impl<P, I, T, E> Attempt<P, I, T, E> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>
{
    pub fn new(parser: P) -> Attempt<P, I, T, E> {
        Attempt{
            parser,
            _i: std::marker::PhantomData,
            _t: std::marker::PhantomData,
            _e: std::marker::PhantomData,
        }
    }
}

impl<P, I, T, E> Parser<I, T, E> for Attempt<P, I, T, E> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>
{
    fn parse(&self, iter: &mut I) -> Result<T, E> {
        self.parser.attempt_parse(iter)
    }
}

#[derive(Clone)]
pub struct Scry<P, I, T, E> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>
{
    parser: P,
    _i: std::marker::PhantomData<I>,
    _t: std::marker::PhantomData<T>,
    _e: std::marker::PhantomData<E>,
}

impl<P, I, T, E> Scry<P, I, T, E> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>
{
    pub fn new(parser: P) -> Scry<P, I, T, E> {
        Scry{
            parser,
            _i: std::marker::PhantomData,
            _t: std::marker::PhantomData,
            _e: std::marker::PhantomData,
        }
    }
}

impl<P, I, T, E> Parser<I, T, E> for Scry<P, I, T, E> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>
{
    fn parse(&self, iter: &mut I) -> Result<T, E> {
        self.parser.scry_parse(iter)
    }
}

#[derive(Clone)]
pub struct Backtrack<P, I, T, E> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>
{
    parser: P,
    _i: std::marker::PhantomData<I>,
    _t: std::marker::PhantomData<T>,
    _e: std::marker::PhantomData<E>,
}

impl<P, I, T, E> Backtrack<P, I, T, E> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>
{
    pub fn new(parser: P) -> Backtrack<P, I, T, E> {
        Backtrack{
            parser,
            _i: std::marker::PhantomData,
            _t: std::marker::PhantomData,
            _e: std::marker::PhantomData,
        }
    }
}

impl<P, I, T, E> Parser<I, T, E> for Backtrack<P, I, T, E> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>
{
    fn parse(&self, iter: &mut I) -> Result<T, E> {
        self.parser.backtrack_parse(iter)
    }
}

// Value mapping

#[derive(Clone)]
pub struct Map<P, I, T, E, U, F> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>,
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
    P: SizedParser<I, T, E>,
    F: Fn(T) -> U
{
    pub fn new(parser: P, inner_map: F) -> Map<P, I, T, E, U, F> {
        Map{
            parser,
            inner_map,
            _i: std::marker::PhantomData,
            _t: std::marker::PhantomData,
            _e: std::marker::PhantomData,
        }
    }
}

impl<P, I, T, E, U, F> Parser<I, U, E> for Map<P, I, T, E, U, F> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>,
    F: Fn(T) -> U
{
    fn parse(&self, iter: &mut I) -> Result<U, E> {
        self.parser.parse(iter).map(&self.inner_map)
    }
}

#[derive(Clone)]
pub struct AndThen<P, I, T, E, U, F> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>,
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
    P: SizedParser<I, T, E>,
    F: Fn(T) -> Result<U, E>
{
    pub fn new(parser: P, inner_bind: F) -> AndThen<P, I, T, E, U, F> {
        AndThen{
            parser,
            inner_bind,
            _i: std::marker::PhantomData,
            _t: std::marker::PhantomData,
            _e: std::marker::PhantomData,
        }
    }
}

impl<P, I, T, E, U, F> Parser<I, U, E> for AndThen<P, I, T, E, U, F> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>,
    F: Fn(T) -> Result<U, E>
{
    fn parse(&self, iter: &mut I) -> Result<U, E> {
        self.parser.parse(iter).and_then(&self.inner_bind)
    }
}

#[derive(Clone)]
pub struct AndCompose<P, I, T, E, Q, U> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>,
    Q: SizedParser<I, U, E>
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
    P: SizedParser<I, T, E>,
    Q: SizedParser<I, U, E>
{
    pub fn new(parser: P, other: Q) -> AndCompose<P, I, T, E, Q, U> {
        AndCompose{
            parser,
            other,
            _i: std::marker::PhantomData,
            _t: std::marker::PhantomData,
            _e: std::marker::PhantomData,
            _u: std::marker::PhantomData,
        }
    }
}

impl<P, I, T, E, Q, U> Parser<I, U, E> for AndCompose<P, I, T, E, Q, U> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>,
    Q: SizedParser<I, U, E>
{
    fn parse(&self, iter: &mut I) -> Result<U, E> {
        self.parser.parse(iter).and_then(|_|
            self.other.parse(iter)
        )
    }
}

#[derive(Clone)]
pub struct AndThenCompose<P, I, T, E, Q, U, F> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>,
    Q: SizedParser<I, U, E>,
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
    P: SizedParser<I, T, E>,
    Q: SizedParser<I, U, E>,
    F: Fn(T) -> Q
{
    pub fn new(parser: P, bind: F) -> AndThenCompose<P, I, T, E, Q, U, F> {
        AndThenCompose{
            parser,
            bind,
            _i: std::marker::PhantomData,
            _t: std::marker::PhantomData,
            _e: std::marker::PhantomData,
            _q: std::marker::PhantomData,
            _u: std::marker::PhantomData,
        }
    }
}

impl<P, I, T, E, Q, U, F> Parser<I, U, E> for AndThenCompose<P, I, T, E, Q, U, F> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>,
    Q: SizedParser<I, U, E>,
    F: Fn(T) -> Q
{
    fn parse(&self, iter: &mut I) -> Result<U, E> {
        (self.bind)(
            self.parser.parse(iter)?
        ).parse(iter)
    }
}

#[derive(Clone)]
pub struct PreserveAndCompose<P, I, T, E, Q, U> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>,
    Q: SizedParser<I, U, E>
{
    parser: P, 
    other: Q,
    _i: std::marker::PhantomData<I>,
    _t: std::marker::PhantomData<T>,
    _e: std::marker::PhantomData<E>,
    _u: std::marker::PhantomData<U>,
}

impl<P, I, T, E, Q, U> PreserveAndCompose<P, I, T, E, Q, U> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>,
    Q: SizedParser<I, U, E>
{
    pub fn new(parser: P, other: Q) -> PreserveAndCompose<P, I, T, E, Q, U> {
        PreserveAndCompose{
            parser,
            other,
            _i: std::marker::PhantomData,
            _t: std::marker::PhantomData,
            _e: std::marker::PhantomData,
            _u: std::marker::PhantomData,
        }
    }
}

impl<P, I, T, E, Q, U> Parser<I, T, E> for PreserveAndCompose<P, I, T, E, Q, U> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>,
    Q: SizedParser<I, U, E>
{
    fn parse(&self, iter: &mut I) -> Result<T, E> {
        self.parser.parse(iter).and_then(|t|
            self.other.parse(iter).map(|_| t)
        )
    }
}

// Error mapping

#[derive(Clone)]
pub struct MapErr<P, I, T, E, F, O> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>,
    O: Fn(E) -> F
{
    parser: P, 
    inner_map: O,
    _i: std::marker::PhantomData<I>,
    _t: std::marker::PhantomData<T>,
    _e: std::marker::PhantomData<E>,
}

impl<P, I, T, E, F, O> MapErr<P, I, T, E, F, O> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>,
    O: Fn(E) -> F
{
    pub fn new(parser: P, inner_map: O) -> MapErr<P, I, T, E, F, O> {
        MapErr {
            parser,
            inner_map,
            _i: std::marker::PhantomData,
            _t: std::marker::PhantomData,
            _e: std::marker::PhantomData,
        }
    }
}

impl<P, I, T, E, F, O> Parser<I, T, F> for MapErr<P, I, T, E, F, O> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>,
    O: Fn(E) -> F
{
    fn parse(&self, iter: &mut I) -> Result<T, F> {
        self.parser.parse(iter).map_err(&self.inner_map)
    }
}

#[derive(Clone)]
pub struct OrElse<P, I, T, E, F, O> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>,
    O: Fn(E) -> Result<T, F>
{
    parser: P, 
    inner_bind: O,
    _i: std::marker::PhantomData<I>,
    _t: std::marker::PhantomData<T>,
    _e: std::marker::PhantomData<E>,
}

impl<P, I, T, E, F, O> OrElse<P, I, T, E, F, O> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>,
    O: Fn(E) -> Result<T, F>
{
    pub fn new(parser: P, inner_bind: O) -> OrElse<P, I, T, E, F, O> {
        OrElse {
            parser,
            inner_bind,
            _i: std::marker::PhantomData,
            _t: std::marker::PhantomData,
            _e: std::marker::PhantomData,
        }
    }
}

impl<P, I, T, E, F, O> Parser<I, T, F> for OrElse<P, I, T, E, F, O> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>,
    O: Fn(E) -> Result<T, F>
{
    fn parse(&self, iter: &mut I) -> Result<T, F> {
        self.parser.parse(iter).or_else(&self.inner_bind)
    }
}

#[derive(Clone)]
pub struct OrCompose<P, I, T, E, Q, F> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>,
    Q: SizedParser<I, T, F>
{
    parser: P, 
    other: Q,
    _i: std::marker::PhantomData<I>,
    _t: std::marker::PhantomData<T>,
    _e: std::marker::PhantomData<E>,
    _f: std::marker::PhantomData<F>,
}

impl<P, I, T, E, Q, F> OrCompose<P, I, T, E, Q, F> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>,
    Q: SizedParser<I, T, F>
{
    pub fn new(parser: P, other: Q) -> OrCompose<P, I, T, E, Q, F> {
        OrCompose {
            parser,
            other,
            _i: std::marker::PhantomData,
            _t: std::marker::PhantomData,
            _e: std::marker::PhantomData,
            _f: std::marker::PhantomData,
        }
    }
}

impl<P, I, T, E, Q, F> Parser<I, T, F> for OrCompose<P, I, T, E, Q, F> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>,
    Q: SizedParser<I, T, F>
{
    fn parse(&self, iter: &mut I) -> Result<T, F> {
        self.parser.parse(iter).or_else(|_|
            self.other.parse(iter)
        )
    }
}

#[derive(Clone)]
pub struct OrElseCompose<P, I, T, E, Q, F, O> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>,
    Q: SizedParser<I, T, F>,
    O: Fn(E) -> Q
{
    parser: P, 
    bind: O,
    _i: std::marker::PhantomData<I>,
    _t: std::marker::PhantomData<T>,
    _e: std::marker::PhantomData<E>,
    _f: std::marker::PhantomData<F>,
}

impl<P, I, T, E, Q, F, O> OrElseCompose<P, I, T, E, Q, F, O> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>,
    Q: SizedParser<I, T, F>,
    O: Fn(E) -> Q
{
    pub fn new(parser: P, bind: O) -> OrElseCompose<P, I, T, E, Q, F, O> {
        OrElseCompose {
            parser,
            bind,
            _i: std::marker::PhantomData,
            _t: std::marker::PhantomData,
            _e: std::marker::PhantomData,
            _f: std::marker::PhantomData,
        }
    }
}

impl<P, I, T, E, Q, F, O> Parser<I, T, F> for OrElseCompose<P, I, T, E, Q, F, O> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>,
    Q: SizedParser<I, T, F>,
    O: Fn(E) -> Q
{
    fn parse(&self, iter: &mut I) -> Result<T, F> {
        self.parser.parse(iter).or_else(|e|
            (self.bind)(e).parse(iter)
        )
    }
}

// Vector Combinators

#[derive(Clone)]
pub struct Many<P, I, T, E> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>
{
    parser: P, 
    _i: std::marker::PhantomData<I>,
    _t: std::marker::PhantomData<T>,
    _e: std::marker::PhantomData<E>,
}

impl<P, I, T, E> Many<P, I, T, E> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>
{
    pub fn new(parser: P) -> Many<P, I, T, E> {
        Many {
            parser,
            _i: std::marker::PhantomData,
            _t: std::marker::PhantomData,
            _e: std::marker::PhantomData,
        }
    }
}

impl<P, I, T, E, F> Parser<I, Vec<T>, F> for Many<P, I, T, E> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>
{
    fn parse(&self, iter: &mut I) -> Result<Vec<T>, F> {
        let mut values = vec![];
        while let Ok(val) = self.parser.parse(iter) {
            values.push(val)
        }
        Ok(values)
    }
}

#[derive(Clone)]
pub struct Some<P, I, T, E> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>
{
    parser: P, 
    _i: std::marker::PhantomData<I>,
    _t: std::marker::PhantomData<T>,
    _e: std::marker::PhantomData<E>,
}

impl<P, I, T, E> Some<P, I, T, E> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>
{
    pub fn new(parser: P) -> Some<P, I, T, E> {
        Some {
            parser, 
            _i: std::marker::PhantomData,
            _t: std::marker::PhantomData,
            _e: std::marker::PhantomData,
        }
    }
}

impl<P, I, T, E> Parser<I, Vec<T>, E> for Some<P, I, T, E> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>
{
    fn parse(&self, iter: &mut I) -> Result<Vec<T>, E> {
        let mut values = vec![self.parser.parse(iter)?];
        while let Ok(val) = self.parser.parse(iter) {
            values.push(val)
        }
        Ok(values)
    }
}

#[derive(Clone)]
pub struct Least<P, I, T, E, Q, U, F> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>,
    Q: SizedParser<I, U, F>
{
    parser: P,
    until: Q,
    _i: std::marker::PhantomData<I>,
    _t: std::marker::PhantomData<T>,
    _e: std::marker::PhantomData<E>,
    _u: std::marker::PhantomData<U>,
    _f: std::marker::PhantomData<F>,
}

impl<P, I, T, E, Q, U, F> Least<P, I, T, E, Q, U, F> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>,
    Q: SizedParser<I, U, F>
{
    pub fn new(parser: P, until: Q) -> Least<P, I, T, E, Q, U, F> {
        Least {
            parser,
            until,
            _i: std::marker::PhantomData,
            _t: std::marker::PhantomData,
            _e: std::marker::PhantomData,
            _u: std::marker::PhantomData,
            _f: std::marker::PhantomData,
        }
    }
}

impl<P, I, T, E, Q, U, F> Parser<I, (Vec<T>, U), F> for Least<P, I, T, E, Q, U, F> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>,
    Q: SizedParser<I, U, F>
{
    fn parse(&self, iter: &mut I) -> Result<(Vec<T>, U), F> {
        let mut values = vec![];
        let u = loop {
            match self.until.parse(iter) {
                Ok(u) => break Ok(u),
                Err(e) => match self.parser.parse(iter) {
                    Ok(val) => values.push(val),
                    Err(_) => break Err(e)
                }
            }
        }?;
        Ok((values, u))
    }
}

#[derive(Clone)]
pub struct Most<P, I, T, E, Q, U, F> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>,
    Q: SizedParser<I, U, F>
{
    parser: P,
    until: Q,
    _i: std::marker::PhantomData<I>,
    _t: std::marker::PhantomData<T>,
    _e: std::marker::PhantomData<E>,
    _u: std::marker::PhantomData<U>,
    _f: std::marker::PhantomData<F>,
}

impl<P, I, T, E, Q, U, F> Most<P, I, T, E, Q, U, F> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>,
    Q: SizedParser<I, U, F>
{
    pub fn new(parser: P, until: Q) -> Most<P, I, T, E, Q, U, F> {
        Most {
            parser,
            until,
            _i: std::marker::PhantomData,
            _t: std::marker::PhantomData,
            _e: std::marker::PhantomData,
            _u: std::marker::PhantomData,
            _f: std::marker::PhantomData,
        }
    }
}

impl<P, I, T, E, Q, U, F> Parser<I, (Vec<T>, U), E> for Most<P, I, T, E, Q, U, F> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>,
    Q: SizedParser<I, U, F>
{
    fn parse(&self, iter: &mut I) -> Result<(Vec<T>, U), E> {
        let mut stack = vec![iter.clone()];
        let mut values = vec![];
        let e = loop {
            let mut child = stack.last().unwrap().clone();
            match self.parser.parse(&mut child) {
                Ok(val) => {
                    stack.push(child);
                    values.push(val)
                },
                Err(e) => break e,
            }
        };
        loop {
            let mut parent = stack.pop().unwrap();
            match self.until.parse(&mut parent) {
                Ok(u) => {
                    *iter = parent;
                    break Ok((values, u))
                },
                Err(_) => {
                    if let None = values.pop() {
                        *iter = parent;
                        break Err(e)
                    }
                },
            }
        }
    }
}

// Error recovery

#[derive(Clone)]
pub struct Continue<P, I, T, E, Q, F> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>,
    Q: SizedParser<I, (), F>
{
    parser: P,
    recover: Q,
    _i: std::marker::PhantomData<I>,
    _t: std::marker::PhantomData<T>,
    _e: std::marker::PhantomData<E>,
    _f: std::marker::PhantomData<F>,
}

impl<P, I, T, E, Q, F> Continue<P, I, T, E, Q, F> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>,
    Q: SizedParser<I, (), F>
{
    pub fn new(parser: P, recover: Q) -> Continue<P, I, T, E, Q, F> {
        Continue {
            parser,
            recover,
            _i: std::marker::PhantomData,
            _t: std::marker::PhantomData,
            _e: std::marker::PhantomData,
            _f: std::marker::PhantomData,
        }
    }
}

impl<P, I, T, E, Q, F> Parser<I, Result<T, E>, F> for Continue<P, I, T, E, Q, F> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>,
    Q: SizedParser<I, (), F>
{
    fn parse(&self, iter: &mut I) -> Result<Result<T, E>, F> {
        let res = self.parser.parse(iter);
        self.recover.parse(iter)?;
        Ok(res)
    }
}

#[derive(Clone)]
pub struct Recover<P, I, T, E, Q, F> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>,
    Q: SizedParser<I, (), F>
{
    parser: P,
    recover: Q,
    _i: std::marker::PhantomData<I>,
    _t: std::marker::PhantomData<T>,
    _e: std::marker::PhantomData<E>,
    _f: std::marker::PhantomData<F>,
}

impl<P, I, T, E, Q, F> Recover<P, I, T, E, Q, F> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>,
    Q: SizedParser<I, (), F>
{
    pub fn new(parser: P, recover: Q) -> Recover<P, I, T, E, Q, F> {
        Recover {
            parser,
            recover,
            _i: std::marker::PhantomData,
            _t: std::marker::PhantomData,
            _e: std::marker::PhantomData,
            _f: std::marker::PhantomData,
        }
    }
}

impl<P, I, T, E, Q, F> Parser<I, Result<T, E>, F> for Recover<P, I, T, E, Q, F> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>,
    Q: SizedParser<I, (), F>
{
    fn parse(&self, iter: &mut I) -> Result<Result<T, E>, F> {
        match self.parser.parse(iter) {
            Ok(res) => Ok(Ok(res)),
            Err(e) => self.recover.parse(iter).map(|_| Err(e)),
        }
    }
}

#[derive(Clone)]
pub struct AbsorbErr<P, I, T, E, U> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>,
    T: Into<Result<U, E>>
{
    parser: P,
    _i: std::marker::PhantomData<I>,
    _t: std::marker::PhantomData<T>,
    _e: std::marker::PhantomData<E>,
    _u: std::marker::PhantomData<U>,
}

impl<P, I, T, E, U> AbsorbErr<P, I, T, E, U> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>,
    T: Into<Result<U, E>>
{
    pub fn new(parser: P) -> AbsorbErr<P, I, T, E, U> {
        AbsorbErr {
            parser,
            _i: std::marker::PhantomData,
            _t: std::marker::PhantomData,
            _e: std::marker::PhantomData,
            _u: std::marker::PhantomData,
        }
    }
}

impl<P, I, T, E, U> Parser<I, U, E> for AbsorbErr<P, I, T, E, U> where
    I: Iterator + Clone,
    P: SizedParser<I, T, E>,
    T: Into<Result<U, E>>
{
    fn parse(&self, iter: &mut I) -> Result<U, E> {
        self.parser.parse(iter)?.into()
    }
}

// Indirection

#[derive(Clone)]
pub struct RefParser<'p, P, I, T, E> where
    I: Iterator + Clone,
    P: Parser<I, T, E>
{
    parser: &'p P,
    _i: std::marker::PhantomData<I>,
    _t: std::marker::PhantomData<T>,
    _e: std::marker::PhantomData<E>,
}

impl<'p, P, I, T, E> RefParser<'p, P, I, T, E> where
    I: Iterator + Clone,
    P: Parser<I, T, E>
{
    pub fn new(parser: &'p P) -> RefParser<'p, P, I, T, E> {
        RefParser {
            parser: parser,
            _i: std::marker::PhantomData,
            _t: std::marker::PhantomData,
            _e: std::marker::PhantomData,
        }
    }
}

impl<'p, P, I, T, E> Parser<I, T, E> for RefParser<'p, P, I, T, E> where
    I: Iterator + Clone,
    P: Parser<I, T, E>
{
    fn parse(&self, iter: &mut I) -> Result<T, E> {
        self.parser.parse(iter)
    }
}

#[derive(Clone)]
pub struct ForwardDef<'p, I, T, E> where
    I: Iterator + Clone
{
    parser: OnceCell<&'p dyn Parser<I, T, E>>
}

impl<'p, I, T, E> ForwardDef<'p, I, T, E> where
    I: Iterator + Clone
{
    pub fn new() -> ForwardDef<'p, I, T, E> {
        ForwardDef {
            parser : OnceCell::new()
        }
    }

    pub fn define(&self, parser: &'p impl Parser<I, T, E>) -> Result<(), &'p dyn Parser<I, T, E>> where
    {
        self.parser.set(parser)
    }
}

impl<'p, I, T, E> Parser<I, T, E> for ForwardDef<'p, I, T, E> where
    I: Iterator + Clone
{
    fn parse(&self, iter: &mut I) -> Result<T, E> {
        self.parser.get().unwrap().parse(iter)
    }
}