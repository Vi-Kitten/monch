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

pub fn parse_ok<I, T, E>(t: T) -> ParseOk<T> where
    I: Iterator + Clone,
    T: Clone
{
    ParseOk::new(t)
}

pub fn parse_err<I, T, E>(e: E) -> ParseErr<E> where
    I: Iterator + Clone,
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

    fn map_err<F>(self, o: impl Fn(E) -> F) -> impl Parser<I, T, F> {
        move |iter: &mut _| {
            self.parse(iter).map_err(&o)
        }
    }

    // variants:
    // attempt_or...
    // backtrack_or...

    fn or_else<F>(self, o: impl Fn(E) -> Result<T, F>) -> impl Parser<I, T, F> {
        move |iter: &mut I| {
            self.parse(iter).or_else(&o)
        }
    }

    fn attempt_or_else<F>(self, o: impl Fn(E) -> Result<T, F>) -> impl Parser<I, T, F> {
        move |iter: &mut I| {
            self.attempt_parse(iter).or_else(&o)
        }
    }

    fn backtrack_or_else<F>(self, o: impl Fn(E) -> Result<T, F>) -> impl Parser<I, T, F> {
        move |iter: &mut I| {
            self.backtrack_parse(iter).or_else(&o)
        }
    }

    fn or_compose<F>(self, p: impl Parser<I, T, F>) -> impl Parser<I, T, F> {
        move |iter: &mut I| {
            self.parse(iter).or_else(|_| p.parse(iter))
        }
    }

    fn attempt_or_compose<F>(self, p: impl Parser<I, T, F>) -> impl Parser<I, T, F> {
        move |iter: &mut I| {
            self.attempt_parse(iter).or_else(|_| p.parse(iter))
        }
    }

    fn backtrack_or_compose<F>(self, p: impl Parser<I, T, F>) -> impl Parser<I, T, F> {
        move |iter: &mut I| {
            self.backtrack_parse(iter).or_else(|_| p.parse(iter))
        }
    }

    fn or_else_compose<F, P>(self, o: impl Fn(E) -> P) -> impl Parser<I, T, F> where
        P: Parser<I, T, F>
    {
        move |iter: &mut I| {
            self.parse(iter).or_else(|e| {
                o(e).parse(iter)
            })
        }
    }

    fn attempt_or_else_compose<F, P>(self, o: impl Fn(E) -> P) -> impl Parser<I, T, F> where
        P: Parser<I, T, F>
    {
        move |iter: &mut I| {
            self.attempt_parse(iter).or_else(|e| {
                o(e).parse(iter)
            })
        }
    }

    fn backtrack_or_else_compose<F, P>(self, o: impl Fn(E) -> P) -> impl Parser<I, T, F> where
        P: Parser<I, T, F>
    {
        move |iter: &mut I| {
            self.backtrack_parse(iter).or_else(|e| {
                o(e).parse(iter)
            })
        }
    }

    //* Vector Combinators

    fn many<F>(self) -> impl Parser<I, Vec<T>, F> {
        move |iter: &mut I| {
            let mut values = vec![];
            while let Ok(val) = self.parse(iter) {
                values.push(val)
            }
            Ok(values)
        }
    }

    fn attempt_many<F>(self) -> impl Parser<I, Vec<T>, F> {
        move |iter: &mut I| {
            let mut values = vec![];
            while let Ok(val) = self.attempt_parse(iter) {
                values.push(val)
            }
            Ok(values)
        }
    }

    fn some(self) -> impl Parser<I, Vec<T>, E> {
        move |iter: &mut I| {
            let mut values = vec![self.parse(iter)?];
            while let Ok(val) = self.parse(iter) {
                values.push(val)
            }
            Ok(values)
        }
    }

    fn attempt_some(self) -> impl Parser<I, Vec<T>, E> {
        move |iter: &mut I| {
            let mut values = vec![self.attempt_parse(iter)?];
            while let Ok(val) = self.attempt_parse(iter) {
                values.push(val)
            }
            Ok(values)
        }
    }

    fn least_until<U, F>(self, end: impl Parser<I, U, F>) -> impl Parser<I, (Vec<T>, U), F> {
        move |iter: &mut I| {
            let mut values = vec![];
            let u = loop {
                match end.parse(iter) {
                    Ok(u) => break Ok(u),
                    Err(e) => match self.parse(iter) {
                        Ok(val) => values.push(val),
                        Err(_) => break Err(e)
                    }
                }
            }?;
            Ok((values, u))
        }
    }

    fn attempt_least_until<U, F>(self, end: impl Parser<I, U, F>) -> impl Parser<I, (Vec<T>, U), F> {
        move |iter: &mut I| {
            let mut values = vec![];
            let u = loop {
                match end.parse(iter) {
                    Ok(u) => break Ok(u),
                    Err(e) => match self.attempt_parse(iter) {
                        Ok(val) => values.push(val),
                        Err(_) => break Err(e)
                    }
                }
            }?;
            Ok((values, u))
        }
    }

    fn most_until<U, F>(self, end: impl Parser<I, U, F>) -> impl Parser<I, (Vec<T>, U), E> {
        move |iter: &mut I| {
            let mut stack = vec![iter.clone()];
            let mut values = vec![];
            let e = loop {
                let mut child = stack.last().unwrap().clone();
                match self.parse(&mut child) {
                    Ok(val) => {
                        stack.push(child);
                        values.push(val)
                    },
                    Err(e) => match end.parse(&mut child) {
                        Ok(u) => {
                            *iter = child;
                            return Ok((values, u))
                        },
                        Err(_) => break e,
                    },
                }
            };
            loop {
                let mut parent = stack.pop().unwrap();
                match end.parse(&mut parent) {
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

    fn attempt_most_until<U, F>(self, end: impl Parser<I, U, F>) -> impl Parser<I, (Vec<T>, U), E> {
        move |iter: &mut I| {
            let mut stack = vec![iter.clone()];
            let mut values = vec![];
            let e = loop {
                let mut child = stack.last().unwrap().clone();
                match self.parse(&mut child) {
                    Ok(val) => {
                        stack.push(child);
                        values.push(val)
                    },
                    Err(e) => break e,
                }
            };
            loop {
                let mut parent = stack.pop().unwrap();
                match end.parse(&mut parent) {
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

    //* Error recovery

    fn continue_with<F>(self, p: impl Parser<I, (), F>) -> impl Parser<I, Result<T, E>, F> {
        move |iter: &mut I| {
            let res = self.parse(iter);
            p.parse(iter)?;
            Ok(res)
        }
    }

    fn scry_then_continue_with<F>(self, p: impl Parser<I, (), F>) -> impl Parser<I, Result<T, E>, F> {
        move |iter: &mut I| {
            let res = self.scry_parse(iter);
            p.parse(iter)?;
            Ok(res)
        }
    }

    fn backtrack_then_continue_with<F>(self, p: impl Parser<I, (), F>) -> impl Parser<I, Result<T, E>, F> {
        move |iter: &mut I| {
            let res = self.backtrack_parse(iter);
            p.parse(iter)?;
            Ok(res)
        }
    }

    fn recover_with<F>(self, p: impl Parser<I, (), F>) -> impl Parser<I, Result<T, E>, F> {
        move |iter: &mut I| {
            match self.parse(iter) {
                Ok(res) => Ok(Ok(res)),
                Err(e) => p.parse(iter).map(|_| Err(e)),
            }
        }
    }

    fn attempt_then_recover_with<F>(self, p: impl Parser<I, (), F>) -> impl Parser<I, Result<T, E>, F> {
        move |iter: &mut I| {
            match self.attempt_parse(iter) {
                Ok(res) => Ok(Ok(res)),
                Err(e) => p.parse(iter).map(|_| Err(e)),
            }
        }
    }

    fn backtrack_then_recover_with<F>(self, p: impl Parser<I, (), F>) -> impl Parser<I, Result<T, E>, F> {
        move |iter: &mut I| {
            match self.backtrack_parse(iter) {
                Ok(res) => Ok(Ok(res)),
                Err(e) => p.parse(iter).map(|_| Err(e)),
            }
        }
    }

    fn absorb_err<U>(self) -> impl Parser<I, U, E> where
        T: Into<Result<U, E>>
    {
        move |iter: &mut I| {
            self.parse(iter)?.into()
        }
    }
}