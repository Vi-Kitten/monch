#![feature(unboxed_closures)]
#![feature(fn_traits)]

#![feature(decl_macro)]
#![feature(never_type)]


#[cfg(test)]
mod tests;
pub mod combinators;
use combinators::*;

pub enum ParseError<Msg = String> {
    UnexpectedEnd,
    Expected(Msg),
    Context(Msg, Box<ParseError>),
    Bundle(Vec<ParseError>)
}

pub fn parse_ok<T>(t: T) -> ParseOk<T> where
    T: Clone
{
    ParseOk::new(t)
}

pub fn parse_err<E>(e: E) -> ParseErr<E> where
    E: Clone
{
    ParseErr::new(e)
}

pub trait UnsizedParser<I, T, E = ParseError> where
    I: Iterator + Clone
{
    fn parse(&self, iter: &mut I) -> Result<T, E>;

    fn attempt_parse(&self, iter: &mut I) -> Result<T, E> {
        let backup = iter.clone();
        self.parse(iter).map_err(|e| {
            *iter = backup;
            e                
        })
    }

    fn scry_parse(&self, iter: &mut I) -> Result<T, E> {
        let backup = iter.clone();
        let val = self.parse(iter)?;
        *iter = backup;
        Ok(val)
    }

    fn backtrack_parse(&self, iter: &mut I) -> Result<T, E> {
        self.parse(&mut iter.clone())
    }
}

impl<P, I, T, E> Parser<I, T, E> for P where
    I: Iterator + Clone,
    P: Sized + UnsizedParser<I, T, E> {}

pub trait Parser<I, T, E = ParseError>: UnsizedParser<I, T, E> where
    I: Iterator + Clone,
    Self: Sized
{
    fn reference<'p>(&'p self) -> RefParser<'p, Self, I, T, E> {
        RefParser::new(self)
    }

    fn discard(self) -> Map<Self, I, T, E, (), impl Fn(T)> {
        self.map(|_| ())
    }

    fn lense<J, F>(self, f: F) -> Lense<Self, I, T, E, J, F> where
        J: Iterator + Clone,
        F: Fn(&mut J) -> &mut I
    {
        Lense::new(self, f)
    }

    // Backtracking

    // backtrack on failure
    fn attempt(self) -> Attempt<Self, I, T, E> {
        Attempt::new(self)
    }

    // backtrack on success
    fn scry(self) -> Scry<Self, I, T, E>  {
        Scry::new(self)
    }

    // always backtrack
    fn backtrack(self) -> Backtrack<Self, I, T, E>  {
        Backtrack::new(self)
    }

    // Value mapping

    fn map<U, F>(self, f: F) -> Map<Self, I, T, E, U, F> where
        F: Fn(T) -> U
    {
        Map::new(self, f)
    }

    fn and_then<U, F>(self, f: F) -> AndThen<Self, I, T, E, U, F> where
        F: Fn(T) -> Result<U, E>
    {
        AndThen::new(self, f)
    }

    fn and_compose<U, P>(self, p: P) -> AndCompose<Self, I, T, E, P, U> where
        P: Parser<I, U, E>
    {
        AndCompose::new(self, p)
    }

    fn preserve_and_compose<U, P>(self, p: P) -> PreserveAndCompose<Self, I, T, E, P, U> where
        P: Parser<I, U, E>
    {
        PreserveAndCompose::new(self, p)
    }

    fn and_then_compose<U, P, F>(self, f: F) -> AndThenCompose<Self, I, T, E, P, U, F> where
        P: Parser<I, U, E>,
        F: Fn(T) -> P
    {
        AndThenCompose::new(self, f)
    }

    // Error mapping

    fn map_err<F, O>(self, o: O) -> MapErr<Self, I, T, E, F, O> where
        O: Fn(E) -> F
    {
        MapErr::new(self, o)
    }

    fn or_else<F, O>(self, o: O) -> OrElse<Self, I, T, E, F, O> where
        O: Fn(E) -> Result<T, F>
    {
        OrElse::new(self, o)
    }

    fn or_compose<F, P>(self, p: P) -> OrCompose<Self, I, T, E, P, F> where
        P: Parser<I, T, F>
    {
        OrCompose::new(self, p)
    }

    fn or_else_compose<F, P, O>(self, o: O) -> OrElseCompose<Self, I, T, E, P, F, O> where
        P: Parser<I, T, F>,
        O: Fn(E) -> P
    {
        OrElseCompose::new(self, o)
    }

    // Vector Combinators

    fn many(self) -> Many<Self, I, T, E> {
        Many::new(self)
    }

    fn some(self) -> Some<Self, I, T, E> {
        Some::new(self)
    }

    fn least_until<U, F, P>(self, end: P) -> Least<Self, I, T, E, P, U, F> where
        P: Parser<I, U, F>
    {
        Least::new(self, end)
    }

    // already attempts due to creation of stack structure
    fn most_until<U, F, P>(self, end: P) -> Most<Self, I, T, E, P, U, F> where
        P: Parser<I, U, F>
    {
        Most::new(self, end)
    }

    // Error recovery

    fn continue_with<F, P>(self, p: P) -> Continue<Self, I, T, E, P, F> where
        P: Parser<I, (), F>
    {
        Continue::new(self, p)
    }

    fn recover_with<F, P>(self, p: P) -> Recover<Self, I, T, E, P, F> where
        P: Parser<I, (), F>
    {
        Recover::new(self, p)
    }

    fn absorb_err<U>(self) -> AbsorbErr<Self, I, T, E, U> where
        T: Into<Result<U, E>>
    {
        AbsorbErr::new(self)
    }
}