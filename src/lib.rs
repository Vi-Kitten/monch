#![feature(decl_macro)]

pub enum ParseError<Msg = String> {
    UnexpectedEnd,
    Expected(Msg),
    Context(Msg, Box<ParseError>),
    Bundle(Vec<ParseError>)
}

impl<F, I, T, E> Parser<I, T, E> for F where
    F: for<'i> Fn(&'i mut I) -> Result<T, E>,
    I: Iterator + Clone
{
    fn parse(&self, iter: &mut I) -> Result<T, E> {
        self(iter)
    }
}

pub fn parse_ok<I, T, E>(t: T) -> impl Parser<I, T, E> where
    I: Iterator + Clone,
    T: Clone
{
    move |_iter: &mut I| {
        Ok(t.clone())
    }
}

pub fn parse_err<I, T, E>(e: E) -> impl Parser<I, T, E> where
    I: Iterator + Clone,
    E: Clone
{
    move |_iter: &mut I| {
        Err(e.clone())
    }
}

pub trait Parser<I, T, E = ParseError> where
    I: Iterator + Clone,
    Self: Sized
{
    fn parse(&self, iter: &mut I) -> Result<T, E>;

    fn discard(self) -> impl Parser<I, (), E> {
        move |iter: &mut I| {
            self.parse(iter).map(|_| ())
        }
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
    fn attempt(self) -> impl Parser<I, T, E> {
        move |iter: &mut I| {
            let backup = iter.clone();
            self.parse(iter).map_err(|e| {
                *iter = backup;
                e                
            })
        }
    }

    fn attempt_parse(&self, iter: &mut I) -> Result<T, E> {
        let backup = iter.clone();
        self.parse(iter).map_err(|e| {
            *iter = backup;
            e                
        })
    }

    // backtrack on success
    fn scry(self) -> impl Parser<I, T, E> {
        move |iter: &mut I| {
            let backup = iter.clone();
            let val = self.parse(iter)?;
            *iter = backup;
            Ok(val)
        }
    }

    fn scrying_parse(&self, iter: &mut I) -> Result<T, E> {
        let backup = iter.clone();
        let val = self.parse(iter)?;
        *iter = backup;
        Ok(val)
    }

    // always backtrack
    fn backtrack(self) -> impl Parser<I, T, E> {
        move |iter: &mut I| {
            self.parse(&mut iter.clone())
        }
    }

    fn backtracking_parse(&self, iter: &mut I) -> Result<T, E> {
        self.parse(&mut iter.clone())
    }

    //* Value mapping

    fn map<U>(self, f: impl Fn(T) -> U) -> impl Parser<I, U, E> {
        move |iter: &mut I| {
            self.parse(iter).map(&f)
        }
    }

    fn and_then<U>(self, f: impl Fn(T) -> Result<U, E>) -> impl Parser<I, U, E> {
        move |iter: &mut I| {
            self.parse(iter).and_then(&f)
        }
    }

    fn scry_and_then<U>(self, f: impl Fn(T) -> Result<U, E>) -> impl Parser<I, U, E> {
        move |iter: &mut I| {
            self.scrying_parse(iter).and_then(&f)
        }
    }

    fn backtrack_and_then<U>(self, f: impl Fn(T) -> Result<U, E>) -> impl Parser<I, U, E> {
        move |iter: &mut I| {
            self.backtracking_parse(iter).and_then(&f)
        }
    }

    fn and_compose<U>(self, p: impl Parser<I, U, E>) -> impl Parser<I, U, E> {
        move |iter: &mut I| {
            self.parse(iter).and_then(|_| p.parse(iter))
        }
    }

    fn scry_and_compose<U>(self, p: impl Parser<I, U, E>) -> impl Parser<I, U, E> {
        move |iter: &mut I| {
            self.scrying_parse(iter).and_then(|_| p.parse(iter))
        }
    }

    fn backtrack_and_compose<U>(self, p: impl Parser<I, U, E>) -> impl Parser<I, U, E> {
        move |iter: &mut I| {
            self.backtracking_parse(iter).and_then(|_| p.parse(iter))
        }
    }

    fn and_then_compose<U, P>(self, f: impl Fn(T) -> P) -> impl Parser<I, U, E> where
        P: Parser<I, U, E>
    {
        move |iter: &mut I| {
            let t = self.parse(iter)?;
            f(t).parse(iter)
        }
    }

    fn scry_and_then_compose<U, P>(self, f: impl Fn(T) -> P) -> impl Parser<I, U, E> where
        P: Parser<I, U, E>
    {
        move |iter: &mut I| {
            let t = self.scrying_parse(iter)?;
            f(t).parse(iter)
        }
    }

    fn backtrack_and_then_compose<U, P>(self, f: impl Fn(T) -> P) -> impl Parser<I, U, E> where
        P: Parser<I, U, E>
    {
        move |iter: &mut I| {
            let t = self.backtracking_parse(iter)?;
            f(t).parse(iter)
        }
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
            self.backtracking_parse(iter).or_else(&o)
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
            self.backtracking_parse(iter).or_else(|_| p.parse(iter))
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
            self.backtracking_parse(iter).or_else(|e| {
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

    fn some(self) -> impl Parser<I, Vec<T>, E> where
        Self: Clone
    {
        let manyself = self.clone().many();
        move |iter: &mut I| {
            let val = self.parse(iter)?;
            manyself.parse(iter).map(|mut values| {
                values.insert(0, val);
                values
            })
        }
    }

    fn attempt_some(self) -> impl Parser<I, Vec<T>, E> where
        Self: Clone
    {
        let manyself = self.clone().attempt_many();
        move |iter: &mut I| {
            let val = self.attempt_parse(iter)?;
            manyself.parse(iter).map(|mut values| {
                values.insert(0, val);
                values
            })
        }
    }

    fn least_until<U, F>(self, end: impl Parser<I, U, F>) -> impl Parser<I, (Vec<T>, U), F> {
        move |iter: &mut I| {
            let mut values = vec![];
            let u = loop {
                match end.parse(iter).or_else(|e| Err(self.parse(iter).map_err(|_| e))) {
                    Ok(u) => break Ok(u),
                    Err(Ok(val)) => values.push(val),
                    Err(Err(e)) => break Err(e),
                }
            }?;
            Ok((values, u))
        }
    }

    fn attempt_least_until<U, F>(self, end: impl Parser<I, U, F>) -> impl Parser<I, (Vec<T>, U), F> {
        move |iter: &mut I| {
            let mut values = vec![];
            let u = loop {
                match end.parse(iter).or_else(|e| Err(self.attempt_parse(iter).map_err(|_| e))) {
                    Ok(u) => break Ok(u),
                    Err(Ok(val)) => values.push(val),
                    Err(Err(e)) => break Err(e),
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
                let res = self.parse(&mut child);
                stack.push(child);
                match res {
                    Ok(val) => values.push(val),
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

    fn attempt_most_until<U, F>(self, end: impl Parser<I, U, F>) -> impl Parser<I, (Vec<T>, U), E> {
        move |iter: &mut I| {
            let mut stack = vec![iter.clone()];
            let mut values = vec![];
            let e = loop {
                let mut child = stack.last().unwrap().clone();
                let res = self.attempt_parse(&mut child);
                stack.push(child);
                match res {
                    Ok(val) => values.push(val),
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
            let res = self.scrying_parse(iter);
            p.parse(iter)?;
            Ok(res)
        }
    }

    fn backtrack_then_continue_with<F>(self, p: impl Parser<I, (), F>) -> impl Parser<I, Result<T, E>, F> {
        move |iter: &mut I| {
            let res = self.backtracking_parse(iter);
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
            match self.backtracking_parse(iter) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    struct WrappedIter<I: Iterator>(I);
    
    impl<I: Iterator> WrappedIter<I> {
        fn get_mut(&mut self) -> &mut I {
            let WrappedIter(iter) = self;
            iter
        }
    }
    
    impl<I: Iterator> Iterator for WrappedIter<I> {
        type Item = I::Item;
    
        fn next(&mut self) -> Option<Self::Item> {
            let WrappedIter(iter) = self;
            iter.next()
        }
    }
    
    fn expect<'s, S: 's + Into<String>, I: Iterator<Item = char> + Clone>(t: S, e: &'s str) -> impl Parser<I, String, &'s str> {
        let string = t.into();
        move |iter: &mut I| {
            if iter.take(string.len()).collect::<String>() == string {
                Ok(string.clone())
            } else {
                Err(e)
            }
        }
    }
        
    #[test]
    fn test_parse() {
        let mut iter = "abc".chars();
        assert_eq!(
            expect("abc", "test_failure")
            .parse(&mut iter),
            Ok("abc".into())
        )
    }

    #[test]
    fn test_parse_ok() {
        let mut iter = "abc".chars();
        assert_eq!(
            parse_ok::<_, _, &str>("abc")
            .parse(&mut iter),
            Ok("abc")
        )
    }

    #[test]
    fn test_parse_err() {
        let mut iter = "abc".chars();
        assert_eq!(
            parse_err::<_, String, _>("err")
            .parse(&mut iter),
            Err("err")
        )
    }

    #[test]
    fn test_discard() {
        let mut iter = "abc".chars();
        assert_eq!(
            expect("abc", "test_failure")
            .discard()
            .parse(&mut iter),
            Ok(())
        )
    }

    #[test]
    fn test_lense() {
        let mut iter = WrappedIter("abc".chars());
        assert_eq!(
            expect("abc", "test_failure")
            .lense(WrappedIter::get_mut)
            .parse(&mut iter),
            Ok("abc".into())
        )
    }

    //* Backtracking

    // backtrack on failure
    #[test]
    fn test_attempt() {
        let mut iter = "abc".chars();
        assert_eq!(
            expect("def", "err")
            .attempt()
            .parse(&mut iter),
            Err("err")
        );
        assert_eq!(
            expect("abc", "test_failure")
            .parse(&mut iter),
            Ok("abc".into())
        )
    }

    #[test]
    fn test_attempt_parse() {
        let mut iter = "abc".chars();
        assert_eq!(
            expect("def", "err")
            .attempt_parse(&mut iter),
            Err("err")
        );
        assert_eq!(
            expect("abc", "test_failure")
            .parse(&mut iter),
            Ok("abc".into())
        )
    }

    // backtrack on success
    #[test]
    fn test_scry() {
        let mut iter = "abc".chars();
        assert_eq!(
            expect("abc", "test_failure")
            .scry()
            .parse(&mut iter),
            Ok("abc".into())
        );
        assert_eq!(
            expect("abc", "test_failure")
            .parse(&mut iter),
            Ok("abc".into())
        )
    }

    #[test]
    fn test_scrying_parse() {
        let mut iter = "abc".chars();
        assert_eq!(
            expect("abc", "test_failure")
            .scrying_parse(&mut iter),
            Ok("abc".into())
        );
        assert_eq!(
            expect("abc", "test_failure")
            .parse(&mut iter),
            Ok("abc".into())
        )
    }

    // always backtrack
    #[test]
    fn test_backtrack() {
        let mut iter = "abc".chars();
        assert_eq!(
            expect("abc", "test_failure")
            .backtrack()
            .parse(&mut iter),
            Ok("abc".into())
        );
        assert_eq!(
            expect("def", "err")
            .backtrack()
            .parse(&mut iter),
            Err("err")
        );
        assert_eq!(
            expect("abc", "test_failure")
            .parse(&mut iter),
            Ok("abc".into())
        )
    }

    #[test]
    fn test_backtracking_parse() {
        let mut iter = "abc".chars();
        assert_eq!(
            expect("abc", "test_failure")
            .backtracking_parse(&mut iter),
            Ok("abc".into())
        );
        assert_eq!(
            expect("def", "err")
            .backtracking_parse(&mut iter),
            Err("err")
        );
        assert_eq!(
            expect("abc", "test_failure")
            .parse(&mut iter),
            Ok("abc".into())
        )
    }

    //* Value mapping

    #[test]
    fn test_map() {
        let mut iter = "abc".chars();
        assert_eq!(
            expect("abc", "test_failure")
            .map(|s| s.to_uppercase())
            .parse(&mut iter),
            Ok(format!("ABC"))
        )
    }

    #[test]
    fn test_and_then() {
        let mut iter = "abc".chars();
        assert_eq!(
            expect("abc", "test_failure")
            .and_then(|s| Ok(s.to_uppercase()))
            .parse(&mut iter),
            Ok("ABC".into())
        )
    }

    #[test]
    fn test_scry_and_then() {
        let mut iter = "abc".chars();
        assert_eq!(
            expect("abc", "test_failure")
            .scry_and_then(|s| Ok(s.to_uppercase()))
            .parse(&mut iter),
            Ok("ABC".into())
        );
        assert_eq!(
            expect("abc", "test_failure").parse(&mut iter),
            Ok("abc".into())
        )
    }

    #[test]
    fn test_backtrack_and_then() {
        let mut iter = "abc".chars();
        assert_eq!(
            expect("abc", "test_failure")
            .backtrack_and_then(|s| Ok(s.to_uppercase()))
            .parse(&mut iter),
            Ok("ABC".into())
        );
        assert_eq!(
            expect("def", "err")
            .backtrack_and_then(|s| Ok(s.to_uppercase()))
            .parse(&mut iter),
            Err("err".into())
        );
        assert_eq!(
            expect("abc", "test_failure").parse(&mut iter),
            Ok("abc".into())
        )
    }

    #[test]
    fn test_and_compose() {
        let mut iter = "abcdef".chars();
        assert_eq!(
            expect("abc", "test_failure")
            .and_compose(expect("def", "test_failure"))
            .parse(&mut iter),
            Ok("def".into())
        )
    }

    #[test]
    fn test_scry_and_compose() {
        let mut iter = "abcdefghi".chars();
        assert_eq!(
            expect("abc", "test_failure")
            .scry_and_compose(expect("abcdef", "test_failure"))
            .parse(&mut iter),
            Ok("abcdef".into())
        );
        assert_eq!(
            expect("ghi", "test_failure").parse(&mut iter),
            Ok("ghi".into())
        )
    }

    #[test]
    fn test_backtrack_and_compose() {
        let mut iter = "abcdefghi".chars();
        assert_eq!(
            expect("abc", "test_failure")
            .backtrack_and_compose(expect("abcdef", "test_failure"))
            .parse(&mut iter),
            Ok("abcdef".into())
        );
        assert_eq!(
            expect("abc", "err")
            .backtrack_and_compose(expect("abcdef", "test_failure")) // should fail earlier
            .parse(&mut iter),
            Err("err")
        );
        assert_eq!(
            expect("ghi", "test_failure")
            .parse(&mut iter),
            Ok("ghi".into())
        )
    }

    #[test]
    fn test_and_then_compose() {
        let mut iter = "abcABC".chars();
        assert_eq!(
            expect("abc", "test_failure")
            .and_then_compose(|s| expect(s.to_uppercase(), "test_failure"))
            .parse(&mut iter),
            Ok("ABC".into())
        )
    }

    #[test]
    fn test_scry_and_then_compose() {
        let mut iter = "abcABCdef".chars();
        assert_eq!(
            expect("abc", "test_failure")
            .scry_and_then_compose(|s| expect(format!("abc{}", s.to_uppercase()), "test_failure"))
            .parse(&mut iter),
            Ok("abcABC".into())
        );
        assert_eq!(
            expect("def", "test_failure")
            .parse(&mut iter),
            Ok("def".into())
        )
    }

    #[test]
    fn test_backtrack_and_then_compose() {
        let mut iter = "abcABCdef".chars();
        assert_eq!(
            expect("abc", "test_failure")
            .backtrack_and_then_compose(|s| expect(format!("abc{}", s.to_uppercase()), "test_failure"))
            .parse(&mut iter),
            Ok("abcABC".into())
        );
        assert_eq!(
            expect("abc", "err")
            .backtrack_and_then_compose(|s| expect(format!("abc{}", s.to_uppercase()), "test_failure"))
            .parse(&mut iter),
            Err("err")
        );
        assert_eq!(
            expect("def", "test_failure")
            .parse(&mut iter),
            Ok("def".into())
        )
    }

    //* Error mapping

    #[test]
    fn test_map_err() {
        todo!()
    }

    // variants:
    // attempt_or...
    // backtrack_or...

    #[test]
    fn test_or_else() {
        todo!()
    }

    #[test]
    fn test_attempt_or_else() {
        todo!()
    }

    #[test]
    fn test_backtrack_or_else() {
        todo!()
    }

    #[test]
    fn test_or_compose() {
        todo!()
    }

    #[test]
    fn test_attempt_or_compose() {
        todo!()
    }

    #[test]
    fn test_backtrack_or_compose() {
        todo!()
    }

    #[test]
    fn test_or_else_compose() {
        todo!()
    }

    #[test]
    fn test_attempt_or_else_compose() {
        todo!()
    }

    #[test]
    fn test_backtrack_or_else_compose() {
        todo!()
    }

    //* Vector Combinators

    #[test]
    fn test_many() {
        todo!()
    }

    #[test]
    fn test_attempt_many() {
        todo!()
    }

    #[test]
    fn test_some() {
        todo!()
    }

    #[test]
    fn test_attempt_some() {
        todo!()
    }

    #[test]
    fn test_least_until() {
        todo!()
    }

    #[test]
    fn test_attempt_least_until() {
        todo!()
    }

    #[test]
    fn test_most_until() {
        todo!()
    }

    #[test]
    fn test_attempt_most_until() {
        todo!()
    }

    //* Error recovery

    #[test]
    fn test_continue_with() {
        todo!()
    }

    #[test]
    fn test_scry_then_continue_with() {
        todo!()
    }

    #[test]
    fn test_backtrack_then_continue_with() {
        todo!()
    }

    #[test]
    fn test_recover_with() {
        todo!()
    }

    #[test]
    fn test_attempt_then_recover_with() {
        todo!()
    }

    #[test]
    fn test_backtrack_then_recover_with() {
        todo!()
    }

    #[test]
    fn test_absorb_err() {
        todo!()
    }
}