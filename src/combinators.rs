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

// Backtracking

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

// Value mapping

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
        self.parser.parse(iter).and_then(|_|
            self.other.parse(iter)
        )
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
        (self.bind)(
            self.parser.parse(iter)?
        ).parse(iter)
    }
}

// Error mapping

#[derive(Clone)]
pub struct MapErr<P, I, T, E, F, O> where
    I: Iterator + Clone,
    P: Parser<I, T, E>,
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
    P: Parser<I, T, E>,
    O: Fn(E) -> F
{
    pub fn new(parser: P, inner_map: O) -> MapErr<P, I, T, E, F, O> {
        MapErr {
            parser: parser, 
            inner_map: inner_map,
            _i: std::marker::PhantomData,
            _t: std::marker::PhantomData,
            _e: std::marker::PhantomData,
        }
    }
}

impl<P, I, T, E, F, O> UnsizedParser<I, T, F> for MapErr<P, I, T, E, F, O> where
    I: Iterator + Clone,
    P: Parser<I, T, E>,
    O: Fn(E) -> F
{
    fn parse(&self, iter: &mut I) -> Result<T, F> {
        self.parser.parse(iter).map_err(&self.inner_map)
    }
}

#[derive(Clone)]
pub struct OrElse<P, I, T, E, F, O> where
    I: Iterator + Clone,
    P: Parser<I, T, E>,
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
    P: Parser<I, T, E>,
    O: Fn(E) -> Result<T, F>
{
    pub fn new(parser: P, inner_bind: O) -> OrElse<P, I, T, E, F, O> {
        OrElse {
            parser: parser, 
            inner_bind: inner_bind,
            _i: std::marker::PhantomData,
            _t: std::marker::PhantomData,
            _e: std::marker::PhantomData,
        }
    }
}

impl<P, I, T, E, F, O> UnsizedParser<I, T, F> for OrElse<P, I, T, E, F, O> where
    I: Iterator + Clone,
    P: Parser<I, T, E>,
    O: Fn(E) -> Result<T, F>
{
    fn parse(&self, iter: &mut I) -> Result<T, F> {
        self.parser.parse(iter).or_else(&self.inner_bind)
    }
}

#[derive(Clone)]
pub struct OrCompose<P, I, T, E, Q, F> where
    I: Iterator + Clone,
    P: Parser<I, T, E>,
    Q: Parser<I, T, F>
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
    P: Parser<I, T, E>,
    Q: Parser<I, T, F>
{
    pub fn new(parser: P, other: Q) -> OrCompose<P, I, T, E, Q, F> {
        OrCompose {
            parser: parser, 
            other: other,
            _i: std::marker::PhantomData,
            _t: std::marker::PhantomData,
            _e: std::marker::PhantomData,
            _f: std::marker::PhantomData,
        }
    }
}

impl<P, I, T, E, Q, F> UnsizedParser<I, T, F> for OrCompose<P, I, T, E, Q, F> where
    I: Iterator + Clone,
    P: Parser<I, T, E>,
    Q: Parser<I, T, F>
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
    P: Parser<I, T, E>,
    Q: Parser<I, T, F>,
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
    P: Parser<I, T, E>,
    Q: Parser<I, T, F>,
    O: Fn(E) -> Q
{
    pub fn new(parser: P, bind: O) -> OrElseCompose<P, I, T, E, Q, F, O> {
        OrElseCompose {
            parser: parser, 
            bind: bind,
            _i: std::marker::PhantomData,
            _t: std::marker::PhantomData,
            _e: std::marker::PhantomData,
            _f: std::marker::PhantomData,
        }
    }
}

impl<P, I, T, E, Q, F, O> UnsizedParser<I, T, F> for OrElseCompose<P, I, T, E, Q, F, O> where
    I: Iterator + Clone,
    P: Parser<I, T, E>,
    Q: Parser<I, T, F>,
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
    P: Parser<I, T, E>
{
    parser: P, 
    _i: std::marker::PhantomData<I>,
    _t: std::marker::PhantomData<T>,
    _e: std::marker::PhantomData<E>,
}

impl<P, I, T, E> Many<P, I, T, E> where
    I: Iterator + Clone,
    P: Parser<I, T, E>
{
    pub fn new(parser: P) -> Many<P, I, T, E> {
        Many {
            parser: parser, 
            _i: std::marker::PhantomData,
            _t: std::marker::PhantomData,
            _e: std::marker::PhantomData,
        }
    }
}

impl<P, I, T, E, F> UnsizedParser<I, Vec<T>, F> for Many<P, I, T, E> where
    I: Iterator + Clone,
    P: Parser<I, T, E>
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
    P: Parser<I, T, E>
{
    parser: P, 
    _i: std::marker::PhantomData<I>,
    _t: std::marker::PhantomData<T>,
    _e: std::marker::PhantomData<E>,
}

impl<P, I, T, E> Some<P, I, T, E> where
    I: Iterator + Clone,
    P: Parser<I, T, E>
{
    pub fn new(parser: P) -> Some<P, I, T, E> {
        Some {
            parser: parser, 
            _i: std::marker::PhantomData,
            _t: std::marker::PhantomData,
            _e: std::marker::PhantomData,
        }
    }
}

impl<P, I, T, E> UnsizedParser<I, Vec<T>, E> for Some<P, I, T, E> where
    I: Iterator + Clone,
    P: Parser<I, T, E>
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
    P: Parser<I, T, E>,
    Q: Parser<I, U, F>
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
    P: Parser<I, T, E>,
    Q: Parser<I, U, F>
{
    pub fn new(parser: P, until: Q) -> Least<P, I, T, E, Q, U, F> {
        Least {
            parser: parser,
            until: until,
            _i: std::marker::PhantomData,
            _t: std::marker::PhantomData,
            _e: std::marker::PhantomData,
            _u: std::marker::PhantomData,
            _f: std::marker::PhantomData,
        }
    }
}

impl<P, I, T, E, Q, U, F> UnsizedParser<I, (Vec<T>, U), F> for Least<P, I, T, E, Q, U, F> where
    I: Iterator + Clone,
    P: Parser<I, T, E>,
    Q: Parser<I, U, F>
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
    P: Parser<I, T, E>,
    Q: Parser<I, U, F>
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
    P: Parser<I, T, E>,
    Q: Parser<I, U, F>
{
    pub fn new(parser: P, until: Q) -> Most<P, I, T, E, Q, U, F> {
        Most {
            parser: parser,
            until: until,
            _i: std::marker::PhantomData,
            _t: std::marker::PhantomData,
            _e: std::marker::PhantomData,
            _u: std::marker::PhantomData,
            _f: std::marker::PhantomData,
        }
    }
}

impl<P, I, T, E, Q, U, F> UnsizedParser<I, (Vec<T>, U), E> for Most<P, I, T, E, Q, U, F> where
    I: Iterator + Clone,
    P: Parser<I, T, E>,
    Q: Parser<I, U, F>
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
    P: Parser<I, T, E>,
    Q: Parser<I, (), F>
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
    P: Parser<I, T, E>,
    Q: Parser<I, (), F>
{
    pub fn new(parser: P, recover: Q) -> Continue<P, I, T, E, Q, F> {
        Continue {
            parser: parser,
            recover: recover,
            _i: std::marker::PhantomData,
            _t: std::marker::PhantomData,
            _e: std::marker::PhantomData,
            _f: std::marker::PhantomData,
        }
    }
}

impl<P, I, T, E, Q, F> UnsizedParser<I, Result<T, E>, F> for Continue<P, I, T, E, Q, F> where
    I: Iterator + Clone,
    P: Parser<I, T, E>,
    Q: Parser<I, (), F>
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
    P: Parser<I, T, E>,
    Q: Parser<I, (), F>
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
    P: Parser<I, T, E>,
    Q: Parser<I, (), F>
{
    pub fn new(parser: P, recover: Q) -> Recover<P, I, T, E, Q, F> {
        Recover {
            parser: parser,
            recover: recover,
            _i: std::marker::PhantomData,
            _t: std::marker::PhantomData,
            _e: std::marker::PhantomData,
            _f: std::marker::PhantomData,
        }
    }
}

impl<P, I, T, E, Q, F> UnsizedParser<I, Result<T, E>, F> for Recover<P, I, T, E, Q, F> where
    I: Iterator + Clone,
    P: Parser<I, T, E>,
    Q: Parser<I, (), F>
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
    P: Parser<I, T, E>,
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
    P: Parser<I, T, E>,
    T: Into<Result<U, E>>
{
    pub fn new(parser: P) -> AbsorbErr<P, I, T, E, U> {
        AbsorbErr {
            parser: parser,
            _i: std::marker::PhantomData,
            _t: std::marker::PhantomData,
            _e: std::marker::PhantomData,
            _u: std::marker::PhantomData,
        }
    }
}

impl<P, I, T, E, U> UnsizedParser<I, U, E> for AbsorbErr<P, I, T, E, U> where
    I: Iterator + Clone,
    P: Parser<I, T, E>,
    T: Into<Result<U, E>>
{
    fn parse(&self, iter: &mut I) -> Result<U, E> {
        self.parser.parse(iter)?.into()
    }
}