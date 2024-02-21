#![feature(decl_macro)]
#![feature(never_type)]


#[cfg(test)]
mod tests;
pub mod indirection;
pub mod combinators;
use combinators::*;

pub enum ParseError<Msg = String> {
    UnexpectedEnd,
    Expected(Msg),
    Context(Msg, Box<ParseError>),
    Bundle(Vec<ParseError>)
}

impl<F, I, T, E> UnsizedParser<I, T, E> for F where
    F: for<'i> Fn(&'i mut I) -> Result<T, E>,
    I: Iterator + Clone
{
    fn parse(&self, iter: &mut I) -> Result<T, E> {
        self(iter)
    }
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

pub macro recursive {
    ($p:expr) => {
        |iter: &mut _| {
            $p.parse(iter)
        }
    }
}

pub trait Parser<I, T, E = ParseError>: UnsizedParser<I, T, E> where
    I: Iterator + Clone,
    Self: Sized
{
    fn discard(self) -> Map<Self, I, T, E, (), impl Fn(T)> {
        self.map(|_| ())
    }

    fn lense<J>(self, f: impl Fn(&mut J) -> &mut I) -> impl Parser<J, T, E> where
        J: Iterator + Clone
    {
        move |jter: &mut J| {
            let mut iter: &mut I = f(jter);
            self.parse(&mut iter)
        }
    }

    //* Backtracking

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

    //* Value mapping

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

    fn scry_and_then<U, F>(self, f: F) -> AndThen<Scry<Self, I, T, E>, I, T, E, U, F> where
        F: Fn(T) -> Result<U, E>
    {
        self.scry().and_then(f)
    }

    fn backtrack_and_then<U, F>(self, f: F) -> AndThen<Backtrack<Self, I, T, E>, I, T, E, U, F> where
        F: Fn(T) -> Result<U, E>
    {
        self.backtrack().and_then(f)
    }

    fn and_compose<U, P>(self, p: P) -> AndCompose<Self, I, T, E, P, U> where
        P: Parser<I, U, E>
    {
        AndCompose::new(self, p)
    }

    fn scry_and_compose<U, P>(self, p: P) -> AndCompose<Scry<Self, I, T, E>, I, T, E, P, U> where
        P: Parser<I, U, E>
    {
        self.scry().and_compose(p)
    }

    fn backtrack_and_compose<U, P>(self, p: P) -> AndCompose<Backtrack<Self, I, T, E>, I, T, E, P, U> where
        P: Parser<I, U, E>
    {
        self.backtrack().and_compose(p)
    }

    fn and_then_compose<U, P, F>(self, f: F) -> AndThenCompose<Self, I, T, E, P, U, F> where
        P: Parser<I, U, E>,
        F: Fn(T) -> P
    {
        AndThenCompose::new(self, f)
    }

    fn scry_and_then_compose<U, P, F>(self, f: F) -> AndThenCompose<Scry<Self, I, T, E>, I, T, E, P, U, F> where
        P: Parser<I, U, E>,
        F: Fn(T) -> P
    {
        self.scry().and_then_compose(f)
    }

    fn backtrack_and_then_compose<U, P, F>(self, f: F) -> AndThenCompose<Backtrack<Self, I, T, E>, I, T, E, P, U, F> where
        P: Parser<I, U, E>,
        F: Fn(T) -> P
    {
        self.backtrack().and_then_compose(f)
    }

    //* Error mapping

    fn map_err<F, O>(self, o: O) -> MapErr<Self, I, T, E, F, O> where
        O: Fn(E) -> F
    {
        MapErr::new(self, o)
    }

    // variants:
    // attempt_or...
    // backtrack_or...

    fn or_else<F, O>(self, o: O) -> OrElse<Self, I, T, E, F, O> where
        O: Fn(E) -> Result<T, F>
    {
        OrElse::new(self, o)
    }

    fn attempt_or_else<F, O>(self, o: O) -> OrElse<Attempt<Self, I, T, E>, I, T, E, F, O> where
        O: Fn(E) -> Result<T, F>
    {
        self.attempt().or_else(o)
    }

    fn backtrack_or_else<F, O>(self, o: O) -> OrElse<Backtrack<Self, I, T, E>, I, T, E, F, O> where
        O: Fn(E) -> Result<T, F>
    {
        self.backtrack().or_else(o)
    }

    fn or_compose<F, P>(self, p: P) -> OrCompose<Self, I, T, E, P, F> where
        P: Parser<I, T, F>
    {
        OrCompose::new(self, p)
    }

    fn attempt_or_compose<F, P>(self, p: P) -> OrCompose<Attempt<Self, I, T, E>, I, T, E, P, F> where
        P: Parser<I, T, F>
    {
        self.attempt().or_compose(p)
    }

    fn backtrack_or_compose<F, P>(self, p: P) -> OrCompose<Backtrack<Self, I, T, E>, I, T, E, P, F> where
        P: Parser<I, T, F>
    {
        self.backtrack().or_compose(p)
    }

    fn or_else_compose<F, P, O>(self, o: O) -> OrElseCompose<Self, I, T, E, P, F, O> where
        P: Parser<I, T, F>,
        O: Fn(E) -> P
    {
        OrElseCompose::new(self, o)
    }

    fn attempt_or_else_compose<F, P, O>(self, o: O) -> OrElseCompose<Attempt<Self, I, T, E>, I, T, E, P, F, O> where
        P: Parser<I, T, F>,
        O: Fn(E) -> P
    {
        self.attempt().or_else_compose(o)
    }

    fn backtrack_or_else_compose<F, P, O>(self, o: O) -> OrElseCompose<Backtrack<Self, I, T, E>, I, T, E, P, F, O> where
        P: Parser<I, T, F>,
        O: Fn(E) -> P
    {
        self.backtrack().or_else_compose(o)
    }

    //* Vector Combinators

    fn many(self) -> Many<Self, I, T, E> {
        Many::new(self)
    }

    fn attempt_many(self) -> Many<Attempt<Self, I, T, E>, I, T, E> {
        self.attempt().many()
    }

    fn some(self) -> Some<Self, I, T, E> {
        Some::new(self)
    }

    fn attempt_some(self) -> Some<Attempt<Self, I, T, E>, I, T, E> {
        self.attempt().some()
    }

    fn least_until<U, F, P>(self, end: P) -> Least<Self, I, T, E, P, U, F> where
        P: Parser<I, U, F>
    {
        Least::new(self, end)
    }

    fn attempt_least_until<U, F, P>(self, end: P) -> Least<Attempt<Self, I, T, E>, I, T, E, P, U, F> where
        P: Parser<I, U, F>
    {
        self.attempt().least_until(end)
    }

    fn attempt_most_until<U, F>(self, end: impl Parser<I, U, F>) -> impl Parser<I, (Vec<T>, U), E> {
        Most::new(self, end)
    }

    //* Error recovery

    fn continue_with<F, P>(self, p: P) -> Continue<Self, I, T, E, P, F> where
        P: Parser<I, (), F>
    {
        Continue::new(self, p)
    }

    fn scry_then_continue_with<F, P>(self, p: P) -> Continue<Scry<Self, I, T, E>, I, T, E, P, F> where
        P: Parser<I, (), F>
    {
        self.scry().continue_with(p)
    }

    fn backtrack_then_continue_with<F, P>(self, p: P) -> Continue<Backtrack<Self, I, T, E>, I, T, E, P, F> where
        P: Parser<I, (), F>
    {
        self.backtrack().continue_with(p)
    }

    fn recover_with<F, P>(self, p: P) -> Recover<Self, I, T, E, P, F> where
        P: Parser<I, (), F>
    {
        Recover::new(self, p)
    }

    fn attempt_then_recover_with<F, P>(self, p: P) -> Recover<Attempt<Self, I, T, E>, I, T, E, P, F> where
        P: Parser<I, (), F>
    {
        self.attempt().recover_with(p)
    }

    fn backtrack_then_recover_with<F, P>(self, p: P) -> Recover<Backtrack<Self, I, T, E>, I, T, E, P, F> where
        P: Parser<I, (), F>
    {
        self.backtrack().recover_with(p)
    }

    fn absorb_err<U>(self) -> AbsorbErr<Self, I, T, E, U> where
        T: Into<Result<U, E>>
    {
        AbsorbErr::new(self)
    }
}