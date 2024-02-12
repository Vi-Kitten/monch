
enum ParseError<Msg = String> {
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

fn ok_parse<I, T, E>(t: T) -> impl Parser<I, T, E> where
    I: Iterator + Clone,
    T: Clone
{
    move |_: &mut I| {
        Ok(t.clone())
    }
}

fn err_parse<I, T, E>(e: E) -> impl Parser<I, T, E> where
    I: Iterator + Clone,
    E: Clone
{
    move |_: &mut I| {
        Err(e.clone())
    }
}

trait Parser<I, T, E = ParseError> where
    I: Iterator + Clone
{

    fn parse(&self, iter: &mut I) -> Result<T, E>;       

    fn discard(&self) -> impl Parser<I, (), E> {
        move |iter: &mut I| {
            self.parse(iter).map(|_| ())
        }
    }

    fn lense<J>(&self, f: &impl Fn(&mut J) -> &mut I) -> impl Parser<J, T, E> where
        J: Iterator + Clone
    {
        move |jter: &mut J| {
            let iter = f(jter);
            self.parse(iter)
        }
    }

    //* Backtracking

    // backtrack on failure
    fn attempt(&self) -> impl Parser<I, T, E> {
        move |iter: &mut I| {
            let backup = iter.clone();
            self.parse(iter).map_err(|e| {
                *iter = backup;
                e                
            })
        }
    }

    // backtrack on success
    fn scy(&self) -> impl Parser<I, T, E> {
        move |iter: &mut I| {
            let backup = iter.clone();
            let val = self.parse(iter)?;
            *iter = backup;
            Ok(val)
        }
    }

    // always backtrack
    fn backtrack(&self) -> impl Parser<I, T, E> {
        move |iter: &mut I| {
            self.parse(&mut iter.clone())
        }
    }

    //* Value mapping

    fn map<U>(&self, f: &impl Fn(T) -> U) -> impl Parser<I, U, E> {
        move |iter: &mut I| {
            self.parse(iter).map(&f)
        }
    }

    // variants:
    // scry_and...
    // backtrack_and...

    fn and_then<U>(&self, f: &impl Fn(T) -> Result<U, E>) -> impl Parser<I, U, E> {
        move |iter: &mut I| {
            self.parse(iter).and_then(&f)
        }
    }

    fn and_parse<U>(&self, p: &impl Parser<I, U, E>) -> impl Parser<I, U, E> {
        move |iter: &mut I| {
            self.parse(iter).and_then(|_| p.parse(iter))
        }
    }

    fn and_then_parse<U, P>(&self, f: &impl Fn(T) -> P) -> impl Parser<I, U, E> where
        P: Parser<I, U, E>
    {
        move |iter: &mut I| {
            let t = self.parse(iter)?;
            f(t).parse(iter)
        }
    }

    //* Error mapping

    fn map_err<F>(&self, o: &impl Fn(E) -> F) -> impl Parser<I, T, F> {
        move |iter: &mut _| {
            self.parse(iter).map_err(&o)
        }
    }

    // variants:
    // attempt_or...
    // backtrack_or...

    fn or_else<F>(&self, o: &impl Fn(E) -> Result<T, F>) -> impl Parser<I, T, F> {
        move |iter: &mut I| {
            self.parse(iter).or_else(&o)
        }
    }

    fn or_parse<F>(&self, p: &impl Parser<I, T, F>) -> impl Parser<I, T, F> {
        move |iter: &mut I| {
            self.parse(iter).or_else(|_| p.parse(iter))
        }
    }

    fn or_else_parse<F, P>(&self, o: &impl Fn(E) -> P) -> impl Parser<I, T, F> where
        P: Parser<I, T, F>
    {
        move |iter: &mut I| {
            self.parse(iter).or_else(|e| {
                o(e).parse(iter)
            })
        }
    }

    //* Combinators

    fn many<F>(&self) -> impl Parser<I, Vec<T>, F> {
        move |iter: &mut I| {
            let mut values = vec![];
            while let Ok(val) = self.parse(iter) {
                values.push(val)
            }
            Ok(values)
        }
    }

    fn some(&self) -> impl Parser<I, Vec<T>, E> {
        move |iter: &mut I| {
            let val = self.parse(iter)?;
            self.many().parse(iter).map(|mut values| {
                values.insert(0, val);
                values
            })
        }
    }

    fn least_until<U, F>(&self, end: &impl Parser<I, U, F>) -> impl Parser<I, (Vec<T>, U), F> {
        move |iter: &mut I| {
            let mut values = vec![];
            let u = loop {
                match match end.parse(iter) {
                    Ok(u) => break u,
                    Err(e) => (e, self.parse(iter)),
                } {
                    (_, Ok(val)) => values.push(val),
                    (e, Err(_)) => return Err(e),
                }
            };
            Ok((values, u))
        }
    }

    fn most_until<U, F>(&self, end: &impl Parser<I, U, F>) -> impl Parser<I, (Vec<T>, U), E> {
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

    //* Error recovery

    // variants:
    // scry_then_continue_with
    // backtrack_then_continue_with

    fn continue_with<F>(&self, p: &impl Parser<I, (), F>) -> impl Parser<I, Result<T, E>, F> {
        move |iter: &mut I| {
            let res = self.parse(iter);
            p.parse(iter)?;
            Ok(res)
        }
    }

    // variants:
    // attempt_then_recover_with
    // backtrack_then_recover_with

    fn recover_with<F>(&self, p: &impl Parser<I, (), F>) -> impl Parser<I, Result<T, E>, F> {
        move |iter: &mut I| {
            match self.parse(iter) {
                Ok(res) => Ok(Ok(res)),
                Err(e) => p.parse(iter).map(|_| Err(e)),
            }
        }
    }

    fn absorb_err<U>(&self) -> impl Parser<I, U, E> where
        T: Into<Result<U, E>>
    {
        move |iter: &mut I| {
            self.parse(iter)?.into()
        }
    }

}