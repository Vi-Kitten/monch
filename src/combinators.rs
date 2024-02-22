use std::{cell::OnceCell, marker::PhantomData};
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
    
    fn parse(&self, _iter: &mut I) -> Result<T, E> {
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

    fn parse(&self, _iter: &mut I) -> Result<T, E> {
        Err(self.err.clone())
    }
}

#[derive(Clone)]
pub struct Lense<P, F> {
    parser: P,
    lense: F,
}

impl<P, F> Lense<P, F> {
    pub fn new(parser: P, lense: F) -> Lense<P, F> {
        Lense {
            parser,
            lense,
        }
    }
}

impl<P, I, T, E, J, F> Parser<J> for Lense<P, F> where
    I: Iterator + Clone,
    P: SizedParser<I, Value=T, Error=E>,
    J: Iterator + Clone,
    F: Fn(&mut J) -> &mut I
{
    type Value = T;
    type Error = E;

    fn parse(&self, jter: &mut J) -> Result<T, E> {        
        let mut iter: &mut I = (self.lense)(jter);
        self.parser.parse(&mut iter)
    }
}

// Backtracking

#[derive(Clone)]
pub struct Attempt<P> {
    parser: P,
}

impl<P> Attempt<P> {
    pub fn new(parser: P) -> Attempt<P> {
        Attempt{
            parser,
        }
    }
}

impl<P, I, T, E> Parser<I> for Attempt<P> where
    I: Iterator + Clone,
    P: SizedParser<I, Value=T, Error=E>
{
    type Value = T;
    type Error = E;
    
    fn parse(&self, iter: &mut I) -> Result<T, E> {
        self.parser.attempt_parse(iter)
    }
}

#[derive(Clone)]
pub struct Scry<P> {
    parser: P,
}

impl<P> Scry<P> {
    pub fn new(parser: P) -> Scry<P> {
        Scry{
            parser,
        }
    }
}

impl<P, I, T, E> Parser<I> for Scry<P> where
    I: Iterator + Clone,
    P: SizedParser<I, Value=T, Error=E>
{
    type Value = T;
    type Error = E;
    
    fn parse(&self, iter: &mut I) -> Result<T, E> {
        self.parser.scry_parse(iter)
    }
}

#[derive(Clone)]
pub struct Backtrack<P> {
    parser: P,
}

impl<P> Backtrack<P> {
    pub fn new(parser: P) -> Backtrack<P> {
        Backtrack{
            parser,
        }
    }
}

impl<P, I, T, E> Parser<I> for Backtrack<P> where
    I: Iterator + Clone,
    P: SizedParser<I, Value=T, Error=E>
{
    type Value = T;
    type Error = E;
    
    fn parse(&self, iter: &mut I) -> Result<T, E> {
        self.parser.backtrack_parse(iter)
    }
}

// Value mapping

#[derive(Clone)]
pub struct Map<P, F> {
    parser: P, 
    inner_map: F,
}

impl<P, F> Map<P, F> {
    pub fn new(parser: P, inner_map: F) -> Map<P, F> {
        Map{
            parser,
            inner_map,
        }
    }
}

impl<P, I, T, E, U, F> Parser<I> for Map<P, F> where
    I: Iterator + Clone,
    P: SizedParser<I, Value=T, Error=E>,
    F: Fn(T) -> U
{
    type Value = U;
    type Error = E;
    
    fn parse(&self, iter: &mut I) -> Result<U, E> {
        self.parser.parse(iter).map(&self.inner_map)
    }
}

#[derive(Clone)]
pub struct AndThen<P, F> where {
    parser: P, 
    inner_bind: F,
}

impl<P, F> AndThen<P, F> {
    pub fn new(parser: P, inner_bind: F) -> AndThen<P, F> {
        AndThen{
            parser,
            inner_bind,
        }
    }
}

impl<P, I, T, E, U, F> Parser<I> for AndThen<P, F> where
    I: Iterator + Clone,
    P: SizedParser<I, Value=T, Error=E>,
    F: Fn(T) -> Result<U, E>
{
    type Value = U;
    type Error = E;
    
    fn parse(&self, iter: &mut I) -> Result<U, E> {
        self.parser.parse(iter).and_then(&self.inner_bind)
    }
}

#[derive(Clone)]
pub struct AndCompose<P, Q> {
    parser: P, 
    other: Q,
}

impl<P, Q> AndCompose<P, Q> {
    pub fn new(parser: P, other: Q) -> AndCompose<P, Q> {
        AndCompose{
            parser,
            other,
        }
    }
}

impl<P, I, E, Q> Parser<I> for AndCompose<P, Q> where
    I: Iterator + Clone,
    P: SizedParser<I, Error=E>,
    Q: SizedParser<I, Error=E>
{
    type Value = Q::Value;
    type Error = E;
    
    fn parse(&self, iter: &mut I) -> Result<Q::Value, E> {
        self.parser.parse(iter).and_then(|_|
            self.other.parse(iter)
        )
    }
}

#[derive(Clone)]
pub struct PreserveAndCompose<P, Q> {
    parser: P, 
    other: Q,
}

impl<P, Q> PreserveAndCompose<P, Q> where {
    pub fn new(parser: P, other: Q) -> PreserveAndCompose<P, Q> {
        PreserveAndCompose{
            parser,
            other,
        }
    }
}

impl<P, I, T, E, Q> Parser<I> for PreserveAndCompose<P, Q> where
    I: Iterator + Clone,
    P: SizedParser<I, Value=T, Error=E>,
    Q: SizedParser<I, Error=E>
{
    type Value = T;
    type Error = E;
    
    fn parse(&self, iter: &mut I) -> Result<T, E> {
        self.parser.parse(iter).and_then(|t|
            self.other.parse(iter).map(|_| t)
        )
    }
}

#[derive(Clone)]
pub struct AndThenCompose<P, F> {
    parser: P, 
    bind: F,
}

impl<P, F> AndThenCompose<P, F> {
    pub fn new(parser: P, bind: F) -> AndThenCompose<P, F> {
        AndThenCompose{
            parser,
            bind,
        }
    }
}

impl<P, I, T, E, Q, F> Parser<I> for AndThenCompose<P, F> where
    I: Iterator + Clone,
    P: SizedParser<I, Value=T, Error=E>,
    Q: SizedParser<I, Error=E>,
    F: Fn(T) -> Q
{
    type Value = Q::Value;
    type Error = E;
    
    fn parse(&self, iter: &mut I) -> Result<Q::Value, E> {
        (self.bind)(
            self.parser.parse(iter)?
        ).parse(iter)
    }
}

// Error mapping

#[derive(Clone)]
pub struct MapErr<P, O> {
    parser: P, 
    inner_map: O,
}

impl<P, O> MapErr<P, O> {
    pub fn new(parser: P, inner_map: O) -> MapErr<P, O> {
        MapErr {
            parser,
            inner_map,
        }
    }
}

impl<P, I, T, E, F, O> Parser<I> for MapErr<P, O> where
    I: Iterator + Clone,
    P: SizedParser<I, Value=T, Error=E>,
    O: Fn(E) -> F
{
    type Value = T;
    type Error = F;
    
    fn parse(&self, iter: &mut I) -> Result<T, F> {
        self.parser.parse(iter).map_err(&self.inner_map)
    }
}

#[derive(Clone)]
pub struct OrElse<P, O> {
    parser: P, 
    inner_bind: O,
}

impl<P, O> OrElse<P, O> {
    pub fn new(parser: P, inner_bind: O) -> OrElse<P, O> {
        OrElse {
            parser,
            inner_bind,
        }
    }
}

impl<P, I, T, E, F, O> Parser<I> for OrElse<P, O> where
    I: Iterator + Clone,
    P: SizedParser<I, Value=T, Error=E>,
    O: Fn(E) -> Result<T, F>
{
    type Value = T;
    type Error = F;
    
    fn parse(&self, iter: &mut I) -> Result<T, F> {
        self.parser.parse(iter).or_else(&self.inner_bind)
    }
}

#[derive(Clone)]
pub struct OrCompose<P, Q> {
    parser: P, 
    other: Q,
}

impl<P, Q> OrCompose<P, Q> {
    pub fn new(parser: P, other: Q) -> OrCompose<P, Q> {
        OrCompose {
            parser,
            other,
        }
    }
}

impl<P, I, T, E, Q> Parser<I> for OrCompose<P, Q> where
    I: Iterator + Clone,
    P: SizedParser<I, Value=T, Error=E>,
    Q: SizedParser<I, Value=T>
{
    type Value = T;
    type Error = Q::Error;
    
    fn parse(&self, iter: &mut I) -> Result<T, Q::Error> {
        self.parser.parse(iter).or_else(|_|
            self.other.parse(iter)
        )
    }
}

#[derive(Clone)]
pub struct OrElseCompose<P, O> {
    parser: P, 
    bind: O,
}

impl<P, O> OrElseCompose<P, O> {
    pub fn new(parser: P, bind: O) -> OrElseCompose<P, O> {
        OrElseCompose {
            parser,
            bind,
        }
    }
}

impl<P, I, T, E, Q, O> Parser<I> for OrElseCompose<P, O> where
    I: Iterator + Clone,
    P: SizedParser<I, Value=T, Error=E>,
    Q: SizedParser<I, Value=T>,
    O: Fn(E) -> Q
{
    type Value = T;
    type Error = Q::Error;
    
    fn parse(&self, iter: &mut I) -> Result<T, Q::Error> {
        self.parser.parse(iter).or_else(|e|
            (self.bind)(e).parse(iter)
        )
    }
}

// Vector Combinators

#[derive(Clone)]
pub struct Many<P, F> {
    parser: P,
    _f: PhantomData<F>,
}

impl<P, F> Many<P, F> {
    pub fn new(parser: P) -> Many<P, F> {
        Many {
            parser,
            _f: PhantomData,
        }
    }
}

impl<P, I, T, E, F> Parser<I> for Many<P, F> where
    I: Iterator + Clone,
    P: SizedParser<I, Value=T, Error=E>
{
    type Value = Vec<T>;
    type Error = F;
    
    fn parse(&self, iter: &mut I) -> Result<Vec<T>, F> {
        let mut values = vec![];
        while let Ok(val) = self.parser.parse(iter) {
            values.push(val)
        }
        Ok(values)
    }
}

#[derive(Clone)]
pub struct Some<P> {
    parser: P,
}

impl<P> Some<P> {
    pub fn new(parser: P) -> Some<P> {
        Some {
            parser,
        }
    }
}

impl<P, I, T, E> Parser<I> for Some<P> where
    I: Iterator + Clone,
    P: SizedParser<I, Value=T, Error=E>
{
    type Value = Vec<T>;
    type Error = E;
    
    fn parse(&self, iter: &mut I) -> Result<Vec<T>, E> {
        let mut values = vec![self.parser.parse(iter)?];
        while let Ok(val) = self.parser.parse(iter) {
            values.push(val)
        }
        Ok(values)
    }
}

#[derive(Clone)]
pub struct Least<P, Q> {
    parser: P,
    until: Q,
}

impl<P, Q> Least<P, Q> {
    pub fn new(parser: P, until: Q) -> Least<P, Q> {
        Least {
            parser,
            until,
        }
    }
}

impl<P, I, T, E, Q> Parser<I> for Least<P, Q> where
    I: Iterator + Clone,
    P: SizedParser<I, Value=T, Error=E>,
    Q: SizedParser<I>
{
    type Value = (Vec<T>, Q::Value);
    type Error = Q::Error;
    
    fn parse(&self, iter: &mut I) -> Result<(Vec<T>, Q::Value), Q::Error> {
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
pub struct Most<P, Q> {
    parser: P,
    until: Q,
}

impl<P, Q> Most<P, Q> {
    pub fn new(parser: P, until: Q) -> Most<P, Q> {
        Most {
            parser,
            until,
        }
    }
}

impl<P, I, T, E, Q> Parser<I> for Most<P, Q> where
    I: Iterator + Clone,
    P: SizedParser<I, Value=T, Error=E>,
    Q: SizedParser<I>
{
    type Value = (Vec<T>, Q::Value);
    type Error = E;
    
    fn parse(&self, iter: &mut I) -> Result<(Vec<T>, Q::Value), E> {
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
pub struct Continue<P, Q> {
    parser: P,
    recover: Q,
}

impl<P, Q> Continue<P, Q> {
    pub fn new(parser: P, recover: Q) -> Continue<P, Q> {
        Continue {
            parser,
            recover,
        }
    }
}

impl<P, I, T, E, Q> Parser<I> for Continue<P, Q> where
    I: Iterator + Clone,
    P: SizedParser<I, Value=T, Error=E>,
    Q: SizedParser<I, Value=()>
{
    type Value = Result<T, E>;
    type Error = Q::Error;
    
    fn parse(&self, iter: &mut I) -> Result<Result<T, E>, Q::Error> {
        let res = self.parser.parse(iter);
        self.recover.parse(iter)?;
        Ok(res)
    }
}

#[derive(Clone)]
pub struct Recover<P, Q> {
    parser: P,
    recover: Q,
}

impl<P, Q> Recover<P, Q> {
    pub fn new(parser: P, recover: Q) -> Recover<P, Q> {
        Recover {
            parser,
            recover,
        }
    }
}

impl<P, I, T, E, Q> Parser<I> for Recover<P, Q> where
    I: Iterator + Clone,
    P: SizedParser<I, Value=T, Error=E>,
    Q: SizedParser<I, Value=()>
{
    type Value = Result<T, E>;
    type Error = Q::Error;
    
    fn parse(&self, iter: &mut I) -> Result<Result<T, E>, Q::Error> {
        match self.parser.parse(iter) {
            Ok(res) => Ok(Ok(res)),
            Err(e) => self.recover.parse(iter).map(|_| Err(e)),
        }
    }
}

#[derive(Clone)]
pub struct AbsorbErr<P> {
    parser: P,
}

impl<P> AbsorbErr<P> {
    pub fn new(parser: P) -> AbsorbErr<P> {
        AbsorbErr {
            parser,
        }
    }
}

impl<P, I, E, U> Parser<I> for AbsorbErr<P> where
    I: Iterator + Clone,
    P: SizedParser<I, Value=Result<U, E>, Error=E>
{
    type Value = U;
    type Error = E;
    
    fn parse(&self, iter: &mut I) -> Result<U, E> {
        self.parser.parse(iter)?
    }
}

// Indirection

#[derive(Clone)]
pub struct RefParser<'p, P> {
    parser: &'p P,
}

impl<'p, P> RefParser<'p, P> {
    pub fn new(parser: &'p P) -> RefParser<'p, P> {
        RefParser {
            parser: parser,
        }
    }
}

impl<'p, P, I, T, E> Parser<I> for RefParser<'p, P> where
    I: Iterator + Clone,
    P: Parser<I, Value=T, Error=E>
{
    type Value = T;
    type Error = E;
    
    fn parse(&self, iter: &mut I) -> Result<T, E> {
        self.parser.parse(iter)
    }
}

#[derive(Clone)]
pub struct ForwardDef<'p, I, T, E> where
    I: Iterator + Clone
{
    parser: OnceCell<&'p dyn Parser<I, Value=T, Error=E>>
}

impl<'p, I, T, E> ForwardDef<'p, I, T, E> where
    I: Iterator + Clone
{
    pub fn new() -> ForwardDef<'p, I, T, E> {
        ForwardDef {
            parser : OnceCell::new()
        }
    }

    pub fn define(&self, parser: &'p impl Parser<I, Value=T, Error=E>) -> Result<(), &'p dyn Parser<I, Value=T, Error=E>> where
    {
        self.parser.set(parser)
    }
}

impl<'p, I, T, E> Parser<I> for ForwardDef<'p, I, T, E> where
    I: Iterator + Clone
{
    type Value = T;
    type Error = E;
    
    fn parse(&self, iter: &mut I) -> Result<T, E> {
        self.parser.get().unwrap().parse(iter)
    }
}