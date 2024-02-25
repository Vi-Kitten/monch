#![feature(decl_macro)]
#![feature(never_type)]

#[cfg(test)]
mod tests;
pub mod combinators;
pub mod errors;

use combinators::*;

#[derive(Clone, Copy, Default)]
pub struct ParseInfo {
    pub taken: usize,
    pub read: usize,
}

impl ParseInfo {
    pub fn new(taken: usize, read: usize) -> ParseInfo {
        ParseInfo {
            taken,
            read,
        }
    }

    pub fn ok<T, E>(self, val: T) -> ParseResult<T, E> {
        ParseResult::new(self, Ok(val))
    }

    pub fn err<T, E>(self, err: E) -> ParseResult<T, E> {
        ParseResult::new(self, Err(err))
    }

    pub fn with<T, E>(self, res: Result<T, E>) -> ParseResult<T, E> {
        ParseResult::new(self, res)
    }
}

/// not commutative
impl std::ops::Add for ParseInfo {
    type Output = ParseInfo;

    fn add(self, rhs: Self) -> Self::Output {
        ParseInfo::new(
            self.taken + rhs.taken,
            std::cmp::max(self.read, self.taken + rhs.read)
        )
    }
}

/// not commutative
impl std::ops::AddAssign for ParseInfo {
    fn add_assign(&mut self, rhs: Self) {
        self.read = std::cmp::max(self.read, self.taken + rhs.read);
        self.taken += rhs.taken;
    }
}

pub struct ParseResult<T, E> {
    info: ParseInfo,
    result: Result<T, E>,
}

impl<T, E> ParseResult<T, E> {
    pub fn new(info: ParseInfo, result: Result<T, E>) -> ParseResult<T, E> {
        ParseResult {
            info,
            result,
        }
    }

    pub fn get_info(&self) -> ParseInfo {
        self.info
    }

    pub fn into_result(self) -> Result<T, E> {
        self.result
    }

    pub fn record_to(self, info: &mut ParseInfo) -> Result<T, E> {
        *info += self.get_info();
        self.into_result()
    }
}

pub fn wrap<T, E>(t: T) -> Wrap<T, E> where
    T: Clone
{
    Wrap::new(t)
}

pub fn fail<T, E>(e: E) -> Fail<T, E> where
    E: Clone
{
    Fail::new(e)
}

pub trait Parser<I> where
    I: Iterator + Clone
{
    type Value;
    type Error;

    fn parse(&self, iter: &mut I) -> ParseResult<Self::Value, Self::Error>;

    fn attempt_parse(&self, iter: &mut I) -> ParseResult<Self::Value, Self::Error> {
        let mut info = ParseInfo::default();
        let backup = iter.clone();
        match self.parse(iter).record_to(&mut info) {
            Ok(val) => info.ok(val),
            Err(err) => {
                *iter = backup;
                info.taken = 0;
                info.err(err)
            }
        }
    }

    fn scry_parse(&self, iter: &mut I) -> ParseResult<Self::Value, Self::Error> {
        let mut info = ParseInfo::default();
        let backup = iter.clone();
        match self.parse(iter).record_to(&mut info) {
            Ok(val) => {
                *iter = backup;
                info.taken = 0;
                info.ok(val)
            },
            Err(err) => info.err(err)
        }
    }

    fn backtrack_parse(&self, iter: &mut I) -> ParseResult<Self::Value, Self::Error> {
        let mut info = ParseInfo::default();
        let res = self.parse(&mut iter.clone()).record_to(&mut info);
        info.taken = 0;
        ParseResult::new(info, res)
    }
}

impl<P, I> SizedParser<I> for P where
    I: Iterator + Clone,
    P: Sized + Parser<I> {}

pub trait SizedParser<I>: Parser<I> where
    I: Iterator + Clone,
    Self: Sized
{
    fn reference<'p>(&'p self) -> RefParser<'_, Self> {
        RefParser::new(self)
    }

    fn discard(self) -> Map<Self, impl Fn(Self::Value) -> ()> {
        self.map(|_| ())
    }

    fn lense<J, F>(self, f: F) -> Lense<Self, F> where
        J: Iterator + Clone,
        F: Fn(&mut J) -> &mut I
    {
        Lense::new(self, f)
    }

    // Backtracking

    // backtrack on failure
    fn attempt(self) -> Attempt<Self> {
        Attempt::new(self)
    }

    // backtrack on success
    fn scry(self) -> Scry<Self> {
        Scry::new(self)
    }

    // always backtrack
    fn backtrack(self) -> Backtrack<Self> {
        Backtrack::new(self)
    }

    // Value mapping

    fn map<U, F>(self, f: F) -> Map<Self, F> where
        F: Fn(Self::Value) -> U
    {
        Map::new(self, f)
    }

    fn and_then<U, F>(self, f: F) -> AndThen<Self, F> where
        F: Fn(Self::Value) -> Result<U, Self::Error>
    {
        AndThen::new(self, f)
    }

    fn and_compose<U, P>(self, p: P) -> AndCompose<Self, P> where
        P: SizedParser<I, Value=U, Error=Self::Error>
    {
        AndCompose::new(self, p)
    }

    fn preserve_and_compose<U, P>(self, p: P) -> PreserveAndCompose<Self, P> where
        P: SizedParser<I, Value=U, Error=Self::Error>
    {
        PreserveAndCompose::new(self, p)
    }

    fn and_then_compose<U, P, F>(self, f: F) -> AndThenCompose<Self, F> where
        P: SizedParser<I, Value=U, Error=Self::Error>,
        F: Fn(Self::Value) -> P
    {
        AndThenCompose::new(self, f)
    }

    // Error mapping

    fn map_err<F, O>(self, o: O) -> MapErr<Self, O> where
        O: Fn(Self::Error) -> F
    {
        MapErr::new(self, o)
    }

    fn or_else<F, O>(self, o: O) -> OrElse<Self, O> where
        O: Fn(Self::Error) -> Result<Self::Value, F>
    {
        OrElse::new(self, o)
    }

    fn or_compose<F, P>(self, p: P) -> OrCompose<Self, P> where
        P: SizedParser<I, Value=Self::Value, Error=F>
    {
        OrCompose::new(self, p)
    }

    fn or_else_compose<F, P, O>(self, o: O) -> OrElseCompose<Self, O> where
        P: SizedParser<I, Value=Self::Value, Error=F>,
        O: Fn(Self::Error) -> P
    {
        OrElseCompose::new(self, o)
    }

    // Vector Combinators

    fn many<E>(self) -> Many<Self, E> {
        Many::new(self)
    }

    fn some(self) -> Some<Self> {
        Some::new(self)
    }

    fn least_until<U, F, P>(self, end: P) -> Least<Self, P> where
        P: SizedParser<I, Value=U, Error=F>
    {
        Least::new(self, end)
    }

    // already attempts due to creation of stack structure
    fn most_until<U, F, P>(self, end: P) -> Most<Self, P> where
        P: SizedParser<I, Value=U, Error=F>
    {
        Most::new(self, end)
    }

    // Error recovery

    fn continue_with<F, P>(self, p: P) -> Continue<Self, P> where
        P: SizedParser<I, Value=(), Error=F>
    {
        Continue::new(self, p)
    }

    fn recover_with<F, P>(self, p: P) -> Recover<Self, P> where
        P: SizedParser<I, Value=(), Error=F>
    {
        Recover::new(self, p)
    }

    fn absorb_err<U>(self) -> AbsorbErr<Map<Self, impl Fn(Self::Value) -> Result<U, Self::Error>>> where
        Self::Value: Into<Result<U, Self::Error>>
    {
        AbsorbErr::new(self.map(|val| val.into()))
    }
}