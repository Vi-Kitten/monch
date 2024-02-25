use std::{cell::OnceCell, marker::PhantomData};
use super::*;
use super::state::*;

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

impl<Tok, State, T, E> Parser<Tok, State> for Wrap<T, E> where
    T: Clone
{
    type Value = T;
    type Error = E;
    
    fn parse<I>(&self, _iter: &mut I) -> Result<T, E> where
        I: StatefulIterator<Item=Tok, State=State> + Clone
    {
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

impl<Tok, State, T, E> Parser<Tok, State> for Fail<T, E> where
    E: Clone
{
    type Value = T;
    type Error = E;
    
    fn parse<I>(&self, _iter: &mut I) -> Result<T, E> where
        I: StatefulIterator<Item=Tok, State=State> + Clone
    {
        Err(self.err.clone())
    }
}

#[derive(Clone)]
pub struct Lense<P, F> {
    parser: P,
    lense: F
}

impl<P, F> Lense<P, F> {
    pub fn new(parser: P, lense: F) -> Lense<P, F> {
        Lense {
            parser,
            lense,
        }
    }
}

impl<Tok, State, InternalState, P, F> Parser<Tok, State> for Lense<P, F> where
    P: SizedParser<Tok, InternalState>,
    F: Fn(&mut State) -> &mut InternalState
{
    type Value = P::Value;
    type Error = P::Error;

    fn parse<J>(&self, jter: &mut J) -> Result<P::Value, P::Error> where
        J: StatefulIterator<Item=Tok, State=State> + Clone
    {
        let mut iter = SubState::new(jter.clone(), &self.lense);
        let res = self.parser.parse(&mut iter);
        *jter = iter.inner();
        res
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

impl<Tok, State, P> Parser<Tok, State> for Attempt<P> where
    P: SizedParser<Tok, State>
{
    type Value = P::Value;
    type Error = P::Error;
    
    fn parse<I>(&self, iter: &mut I) -> Result<P::Value, P::Error> where
        I: StatefulIterator<Item=Tok, State=State> + Clone
    {
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

impl<Tok, State, P> Parser<Tok, State> for Scry<P> where
    P: SizedParser<Tok, State>
{
    type Value = P::Value;
    type Error = P::Error;
    
    fn parse<I>(&self, iter: &mut I) -> Result<P::Value, P::Error> where
        I: StatefulIterator<Item=Tok, State=State> + Clone
    {
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

impl<Tok, State, P> Parser<Tok, State> for Backtrack<P> where
    P: SizedParser<Tok, State>
{
    type Value = P::Value;
    type Error = P::Error;
    
    fn parse<I>(&self, iter: &mut I) -> Result<P::Value, P::Error> where
        I: StatefulIterator<Item=Tok, State=State> + Clone
    {
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

impl<Tok, State, P, U, F> Parser<Tok, State> for Map<P, F> where
    P: SizedParser<Tok, State>,
    F: Fn(P::Value) -> U
{
    type Value = U;
    type Error = P::Error;
    
    fn parse<I>(&self, iter: &mut I) -> Result<U, P::Error> where
        I: StatefulIterator<Item=Tok, State=State> + Clone
    {
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

impl<Tok, State, P, U, F> Parser<Tok, State> for AndThen<P, F> where
    P: SizedParser<Tok, State>,
    F: Fn(P::Value) -> Result<U, P::Error>
{
    type Value = U;
    type Error = P::Error;
    
    fn parse<I>(&self, iter: &mut I) -> Result<U, P::Error> where
        I: StatefulIterator<Item=Tok, State=State> + Clone
    {
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

impl<Tok, State, P, E, Q> Parser<Tok, State> for AndCompose<P, Q> where
    P: SizedParser<Tok, State, Error=E>,
    Q: SizedParser<Tok, State, Error=E>
{
    type Value = Q::Value;
    type Error = E;
    
    fn parse<I>(&self, iter: &mut I) -> Result<Q::Value, E> where
        I: StatefulIterator<Item=Tok, State=State> + Clone
    {
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

impl<Tok, State, P, E, Q> Parser<Tok, State> for PreserveAndCompose<P, Q> where
    P: SizedParser<Tok, State, Error=E>,
    Q: SizedParser<Tok, State, Error=E>
{
    type Value = P::Value;
    type Error = E;
    
    fn parse<I>(&self, iter: &mut I) -> Result<P::Value, E> where
        I: StatefulIterator<Item=Tok, State=State> + Clone
    {
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

impl<Tok, State, P, E, Q, F> Parser<Tok, State> for AndThenCompose<P, F> where
    P: SizedParser<Tok, State, Error=E>,
    Q: SizedParser<Tok, State, Error=E>,
    F: Fn(P::Value) -> Q
{
    type Value = Q::Value;
    type Error = E;
    
    fn parse<I>(&self, iter: &mut I) -> Result<Q::Value, E> where
        I: StatefulIterator<Item=Tok, State=State> + Clone
    {
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

impl<Tok, State, P, F, O> Parser<Tok, State> for MapErr<P, O> where
    P: SizedParser<Tok, State>,
    O: Fn(P::Error) -> F
{
    type Value = P::Value;
    type Error = F;
    
    fn parse<I>(&self, iter: &mut I) -> Result<P::Value, F> where
        I: StatefulIterator<Item=Tok, State=State> + Clone
    {
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

impl<Tok, State, P, F, O> Parser<Tok, State> for OrElse<P, O> where
    P: SizedParser<Tok, State>,
    O: Fn(P::Error) -> Result<P::Value, F>
{
    type Value = P::Value;
    type Error = F;
    
    fn parse<I>(&self, iter: &mut I) -> Result<P::Value, F> where
        I: StatefulIterator<Item=Tok, State=State> + Clone
    {
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

impl<Tok, State, P, T, Q> Parser<Tok, State> for OrCompose<P, Q> where
    P: SizedParser<Tok, State, Value=T>,
    Q: SizedParser<Tok, State, Value=T>
{
    type Value = T;
    type Error = Q::Error;
    
    fn parse<I>(&self, iter: &mut I) -> Result<T, Q::Error> where
        I: StatefulIterator<Item=Tok, State=State> + Clone
    {
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

impl<Tok, State, P, T, Q, O> Parser<Tok, State> for OrElseCompose<P, O> where
    P: SizedParser<Tok, State, Value=T>,
    Q: SizedParser<Tok, State, Value=T>,
    O: Fn(P::Error) -> Q
{
    type Value = T;
    type Error = Q::Error;
    
    fn parse<I>(&self, iter: &mut I) -> Result<T, Q::Error> where
        I: StatefulIterator<Item=Tok, State=State> + Clone
    {
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

impl<Tok, State, P, F> Parser<Tok, State> for Many<P, F> where
    P: SizedParser<Tok, State>
{
    type Value = Vec<P::Value>;
    type Error = F;
    
    fn parse<I>(&self, iter: &mut I) -> Result<Vec<P::Value>, F> where
        I: StatefulIterator<Item=Tok, State=State> + Clone
    {
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

impl<Tok, State, P> Parser<Tok, State> for Some<P> where
    P: SizedParser<Tok, State>
{
    type Value = Vec<P::Value>;
    type Error = P::Error;
    
    fn parse<I>(&self, iter: &mut I) -> Result<Vec<P::Value>, P::Error> where
        I: StatefulIterator<Item=Tok, State=State> + Clone
    {
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

impl<Tok, State, P, Q> Parser<Tok, State> for Least<P, Q> where
    P: SizedParser<Tok, State>,
    Q: SizedParser<Tok, State>
{
    type Value = (Vec<P::Value>, Q::Value);
    type Error = Q::Error;
    
    fn parse<I>(&self, iter: &mut I) -> Result<(Vec<P::Value>, Q::Value), Q::Error> where
        I: StatefulIterator<Item=Tok, State=State> + Clone
    {
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

impl<Tok, State, P, Q> Parser<Tok, State> for Most<P, Q> where
    P: SizedParser<Tok, State>,
    Q: SizedParser<Tok, State>
{
    type Value = (Vec<P::Value>, Q::Value);
    type Error = P::Error;
    
    fn parse<I>(&self, iter: &mut I) -> Result<(Vec<P::Value>, Q::Value), P::Error> where
        I: StatefulIterator<Item=Tok, State=State> + Clone
    {
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

impl<Tok, State, P, Q> Parser<Tok, State> for Continue<P, Q> where
    P: SizedParser<Tok, State>,
    Q: SizedParser<Tok, State, Value=()>
{
    type Value = Result<P::Value, P::Error>;
    type Error = Q::Error;
    
    fn parse<I>(&self, iter: &mut I) -> Result<Result<P::Value, P::Error>, Q::Error> where
        I: StatefulIterator<Item=Tok, State=State> + Clone
    {
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

impl<Tok, State, P, Q> Parser<Tok, State> for Recover<P, Q> where
    P: SizedParser<Tok, State>,
    Q: SizedParser<Tok, State, Value=()>
{
    type Value = Result<P::Value, P::Error>;
    type Error = Q::Error;
    
    fn parse<I>(&self, iter: &mut I) -> Result<Result<P::Value, P::Error>, Q::Error> where
        I: StatefulIterator<Item=Tok, State=State> + Clone
    {
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

impl<Tok, State, P, T, E> Parser<Tok, State> for AbsorbErr<P> where
    P: SizedParser<Tok, State, Value=Result<T, E>, Error=E>
{
    type Value = T;
    type Error = E;
    
    fn parse<I>(&self, iter: &mut I) -> Result<T, E> where
        I: StatefulIterator<Item=Tok, State=State> + Clone
    {
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

impl<'p, Tok, State, P> Parser<Tok, State> for RefParser<'p, P> where
    P: Parser<Tok, State>
{
    type Value = P::Value;
    type Error = P::Error;
    
    fn parse<I>(&self, iter: &mut I) -> Result<P::Value, P::Error> where
        I: StatefulIterator<Item=Tok, State=State> + Clone
    {
        self.parser.parse(iter)
    }
}

#[derive(Clone)]
pub struct Arrow<F> {
    f: F,
}

impl<F> Arrow<F> {
    pub fn new(f: F) -> Arrow<F> {
        Arrow {
            f,
        }
    }
}

impl<Tok, State, T, E, F> Parser<Tok, State> for Arrow<F> where
    F: Fn(&mut dyn DynCloneStatefulIterator<Item=Tok, State=State>) -> Result<T, E>
{
    type Value = T;
    type Error = E;

    fn parse<I>(&self, iter: &mut I) -> Result<T, E> where
        I: StatefulIterator<Item=Tok, State=State> + Clone
    {
        (self.f)(iter)
    }
}

pub struct ForwardDef<Tok, State, T, E> {
    parser: OnceCell<Box<dyn Fn(&mut dyn DynCloneStatefulIterator<Item=Tok, State=State>) -> Result<T, E>>>
}

impl<Tok, State, T, E> ForwardDef<Tok, State, T, E> {
    pub fn new() -> ForwardDef<Tok, State, T, E> {
        ForwardDef {
            parser : OnceCell::new()
        }
    }

    pub fn define(&self, parser: impl Parser<Tok, State, Value=T, Error=E>) ->
        Result<(), Box<dyn Fn(&mut dyn DynCloneStatefulIterator<Item=Tok, State=State>) -> Result<T, E>>>
    {
        self.parser.set(Box::new(
            move |iter: &mut dyn DynCloneStatefulIterator<Item=Tok, State=State>| {
                parser.parse(iter)
            }
        ))
    }
}

impl<Tok, State, T, E> Parser<Tok, State> for ForwardDef<Tok, State, T, E> {
    type Value = T;
    type Error = E;
    
    fn parse<I>(&self, iter: &mut I) -> Result<T, E> where
        I: StatefulIterator<Item=Tok, State=State> + Clone
    {
        (self.parser.get().unwrap())(iter)
    }
}

// Application

#[derive(Clone)]
pub struct Apply0<F, E> {
    f: F,
    _e: PhantomData<E>,
}

impl<F, E> Apply0<F, E> {
    pub fn new(f: F) -> Apply0<F, E> {
        Apply0 {
            f,
            _e: PhantomData
        }
    }
}

impl<Tok, State, F, T, E> Parser<Tok, State> for Apply0<F, E> where
    F: Fn() -> T
{
    type Value = T;
    type Error = E;

    fn parse<I>(&self, _iter: &mut I) -> Result<T, E> where
        I: StatefulIterator<Item=Tok, State=State> + Clone
    {
        Ok(
            (self.f)()
        )
    }
}

#[derive(Clone)]
pub struct Apply1<F, P1> {
    f: F,
    p1: P1,
}

impl<F, P1> Apply1<F, P1> {
    pub fn new(f: F, p1: P1) -> Apply1<F, P1> {
        Apply1 {
            f,
            p1,
        }
    }
}

impl<Tok, State, F, T, E, P1> Parser<Tok, State> for Apply1<F, P1> where
    P1: Parser<Tok, State, Error=E>,
    F: Fn(P1::Value) -> T
{
    type Value = T;
    type Error = E;

    fn parse<I>(&self, iter: &mut I) -> Result<T, E> where
        I: StatefulIterator<Item=Tok, State=State> + Clone
    {
        Ok(
            (self.f)(
                self.p1.parse(iter)?
            )
        )
    }
}

#[derive(Clone)]
pub struct Apply2<F, P1, P2> {
    f: F,
    p1: P1,
    p2: P2,
}

impl<F, P1, P2> Apply2<F, P1, P2> {
    pub fn new(f: F, p1: P1, p2: P2) -> Apply2<F, P1, P2> {
        Apply2 {
            f,
            p1,
            p2,
        }
    }
}

impl<Tok, State, F, T, E, P1, P2> Parser<Tok, State> for Apply2<F, P1, P2> where
    P1: Parser<Tok, State, Error=E>,
    P2: Parser<Tok, State, Error=E>,
    F: Fn(P1::Value, P2::Value) -> T
{
    type Value = T;
    type Error = E;

    fn parse<I>(&self, iter: &mut I) -> Result<T, E> where
        I: StatefulIterator<Item=Tok, State=State> + Clone
    {
        Ok(
            (self.f)(
                self.p1.parse(iter)?,
                self.p2.parse(iter)?
            )
        )
    }
}

#[derive(Clone)]
pub struct Apply3<F, P1, P2, P3> {
    f: F,
    p1: P1,
    p2: P2,
    p3: P3,
}

impl<F, P1, P2, P3> Apply3<F, P1, P2, P3> {
    pub fn new(f: F, p1: P1, p2: P2, p3: P3) -> Apply3<F, P1, P2, P3> {
        Apply3 {
            f,
            p1,
            p2,
            p3,
        }
    }
}

impl<Tok, State, F, T, E, P1, P2, P3> Parser<Tok, State> for Apply3<F, P1, P2, P3> where
    P1: Parser<Tok, State, Error=E>,
    P2: Parser<Tok, State, Error=E>,
    P3: Parser<Tok, State, Error=E>,
    F: Fn(P1::Value, P2::Value, P3::Value) -> T
{
    type Value = T;
    type Error = E;

    fn parse<I>(&self, iter: &mut I) -> Result<T, E> where
        I: StatefulIterator<Item=Tok, State=State> + Clone
    {
        Ok(
            (self.f)(
                self.p1.parse(iter)?,
                self.p2.parse(iter)?,
                self.p3.parse(iter)?
            )
        )
    }
}

#[derive(Clone)]
pub struct Apply4<F, P1, P2, P3, P4> {
    f: F,
    p1: P1,
    p2: P2,
    p3: P3,
    p4: P4,
}

impl<F, P1, P2, P3, P4> Apply4<F, P1, P2, P3, P4> {
    pub fn new(f: F, p1: P1, p2: P2, p3: P3, p4: P4) -> Apply4<F, P1, P2, P3, P4> {
        Apply4 {
            f,
            p1,
            p2,
            p3,
            p4,
        }
    }
}

impl<Tok, State, F, T, E, P1, P2, P3, P4> Parser<Tok, State> for Apply4<F, P1, P2, P3, P4> where
    P1: Parser<Tok, State, Error=E>,
    P2: Parser<Tok, State, Error=E>,
    P3: Parser<Tok, State, Error=E>,
    P4: Parser<Tok, State, Error=E>,
    F: Fn(P1::Value, P2::Value, P3::Value, P4::Value) -> T
{
    type Value = T;
    type Error = E;

    fn parse<I>(&self, iter: &mut I) -> Result<T, E> where
        I: StatefulIterator<Item=Tok, State=State> + Clone
    {
        Ok(
            (self.f)(
                self.p1.parse(iter)?,
                self.p2.parse(iter)?,
                self.p3.parse(iter)?,
                self.p4.parse(iter)?
            )
        )
    }
}