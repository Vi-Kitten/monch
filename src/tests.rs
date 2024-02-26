use super::*;

struct Expect {
    expect: String,
    err: String,
}

impl Expect {
    pub fn new<T, E>(expect: T, err: E) -> Expect where
        T: Into<String>,
        E: Into<String>
    {
        Expect {
            expect: expect.into(),
            err: err.into(),
        }
    }
}

impl<I> Parser<I> for Expect where
    I: Iterator<Item=char> + Clone
{
    type Value = String;
    type Error = String;

    fn parse(&self, iter: &mut I) -> ParseResult<String, String> {
        let found = iter.take(self.expect.len()).collect::<String>();
        let info = ParseInfo::new(found.len(), self.expect.len());
        if found == self.expect {
            info.ok(self.expect.clone())
        } else {
            info.err(self.err.clone())
        }
    }
}

fn expect<I, T, E>(expect: T, err: E) -> impl Parser<I, Value=String, Error=String> where
    I: Iterator<Item=char> + Clone,
    T: Into<String>,
    E: Into<String>
{
    Expect::new(expect, err)
}

struct ExpectEnd {
    err: String,
}

impl ExpectEnd {
    pub fn new<E>(err: E) -> ExpectEnd where
        E: Into<String>
    {
        ExpectEnd {
            err: err.into(),
        }
    }
}

impl<I> Parser<I> for ExpectEnd where
    I: Iterator<Item=char> + Clone
{
    type Value = ();
    type Error = String;

    fn parse(&self, iter: &mut I) -> ParseResult<(), String> {
        if let None = iter.next() {
            ParseInfo::new(0, 1).ok(())
        } else {
            ParseInfo::new(1, 1).err(self.err.clone())
        }
    }
}

fn expect_end<I, E>(err: E) -> impl Parser<I, Value=(), Error=String> where
    I: Iterator<Item=char> + Clone,
    E: Into<String>
{
    ExpectEnd::new(err)
}
    
#[test]
fn test_parse() {
    let mut iter = "abc".chars();

    let mut info = ParseInfo::default();
    assert_eq!(
        expect("abc", "test_failure")
        .parse(&mut iter).record_to(&mut info),
        Ok("abc".into())
    );
    assert_eq!(
        info,
        ParseInfo::new(3, 3)
    );

    info = ParseInfo::default();
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter).record_to(&mut info),
        Ok(())
    );
    assert_eq!(
        info,
        ParseInfo::new(0, 1)
    )
}

#[test]
fn test_wrap() {
    let mut iter = "".chars();

    let mut info = ParseInfo::default();
    assert_eq!(
        wrap::<_, !>("abc".into())
        .parse(&mut iter).record_to(&mut info),
        Ok("abc")
    );
    assert_eq!(
        info,
        ParseInfo::new(0, 0)
    );

    info = ParseInfo::default();
    assert_eq!(
        expect_end("test_faileure")
        .parse(&mut iter).record_to(&mut info),
        Ok(())
    );
    assert_eq!(
        info,
        ParseInfo::new(0, 1)
    )
}

#[test]
fn test_fail() {
    let mut iter = "".chars();

    let mut info = ParseInfo::default();
    assert_eq!(
        fail::<!, _>(format!("err"))
        .parse(&mut iter).record_to(&mut info),
        Err("err".into())
    );
    assert_eq!(
        info,
        ParseInfo::new(0, 0)
    );

    info = ParseInfo::default();
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter).record_to(&mut info),
        Ok(())
    );
    assert_eq!(
        info,
        ParseInfo::new(0, 1)
    )
}

#[test]
fn test_discard() {
    let mut iter = "abc".chars();

    let mut info = ParseInfo::default();
    assert_eq!(
        expect("abc", "test_failure")
        .discard()
        .parse(&mut iter).record_to(&mut info),
        Ok(())
    );
    assert_eq!(
        info,
        ParseInfo::new(3, 3)
    );

    info = ParseInfo::default();
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter).record_to(&mut info),
        Ok(())
    );
    assert_eq!(
        info,
        ParseInfo::new(0, 1)
    )
}

#[test]
fn test_lense() {
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

    impl<I: Iterator + Clone> Clone for WrappedIter<I> {
        fn clone(&self) -> Self {
            let WrappedIter(iter) = self;
            WrappedIter(iter.clone())
        }
    }

    let mut iter = WrappedIter("abc".chars());

    let mut info = ParseInfo::default();
    assert_eq!(
        expect("abc", "test_failure")
        .lense(WrappedIter::get_mut)
        .parse(&mut iter).record_to(&mut info),
        Ok("abc".into())
    );
    assert_eq!(
        info,
        ParseInfo::new(3, 3)
    );

    info = ParseInfo::default();
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter).record_to(&mut info),
        Ok(())
    );
    assert_eq!(
        info,
        ParseInfo::new(0, 1)
    )
}

// Backtracking

// backtrack on failure
#[test]
fn test_attempt() {
    let mut iter = "abc".chars();

    let mut info = ParseInfo::default();
    assert_eq!(
        expect("def", "err")
        .attempt()
        .parse(&mut iter).record_to(&mut info),
        Err("err".into())
    );
    assert_eq!(
        info,
        ParseInfo::new(0, 3)
    );

    info = ParseInfo::default();
    assert_eq!(
        expect("abc", "test_failure")
        .attempt()
        .parse(&mut iter).record_to(&mut info),
        Ok("abc".into())
    );
    assert_eq!(
        info,
        ParseInfo::new(3, 3)
    );


    info = ParseInfo::default();
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter).record_to(&mut info),
        Ok(())
    );
    assert_eq!(
        info,
        ParseInfo::new(0, 1)
    )
}

#[test]
fn test_attempt_parse() {
    let mut iter = "abc".chars();

    let mut info = ParseInfo::default();
    assert_eq!(
        expect("def", "err")
        .attempt_parse(&mut iter).record_to(&mut info),
        Err("err".into())
    );
    assert_eq!(
        info,
        ParseInfo::new(0, 3)
    );

    info = ParseInfo::default();
    assert_eq!(
        expect("abc", "test_failure")
        .attempt_parse(&mut iter).record_to(&mut info),
        Ok("abc".into())
    );
    assert_eq!(
        info,
        ParseInfo::new(3, 3)
    );

    info = ParseInfo::default();
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter).record_to(&mut info),
        Ok(())
    );
    assert_eq!(
        info,
        ParseInfo::new(0, 1)
    )
}

// backtrack on success
#[test]
fn test_scry() {
    let mut iter = "abc".chars();

    let mut info = ParseInfo::default();
    assert_eq!(
        expect("abc", "test_failure")
        .scry()
        .parse(&mut iter).record_to(&mut info),
        Ok("abc".into())
    );
    assert_eq!(
        info,
        ParseInfo::new(0, 3)
    );

    info = ParseInfo::default();
    assert_eq!(
        expect("def", "err")
        .scry()
        .parse(&mut iter).record_to(&mut info),
        Err("err".into())
    );
    assert_eq!(
        info,
        ParseInfo::new(3, 3)
    );

    info = ParseInfo::default();
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter).record_to(&mut info),
        Ok(())
    );
    assert_eq!(
        info,
        ParseInfo::new(0, 1)
    )
}

#[test]
fn test_scry_parse() {
    let mut iter = "abc".chars();

    let mut info = ParseInfo::default();
    assert_eq!(
        expect("abc", "test_failure")
        .scry_parse(&mut iter).record_to(&mut info),
        Ok("abc".into())
    );
    assert_eq!(
        info,
        ParseInfo::new(0, 3)
    );

    info = ParseInfo::default();
    assert_eq!(
        expect("def", "err")
        .scry_parse(&mut iter).record_to(&mut info),
        Err("err".into())
    );
    assert_eq!(
        info,
        ParseInfo::new(3, 3)
    );

    info = ParseInfo::default();
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter).record_to(&mut info),
        Ok(())
    );
    assert_eq!(
        info,
        ParseInfo::new(0, 1)
    )
}

// always backtrack
#[test]
fn test_backtrack() {
    let mut iter = "abc".chars();

    let mut info = ParseInfo::default();
    assert_eq!(
        expect("abc", "test_failure")
        .backtrack()
        .parse(&mut iter).record_to(&mut info),
        Ok("abc".into())
    );
    assert_eq!(
        info,
        ParseInfo::new(0, 3)
    );

    info = ParseInfo::default();
    assert_eq!(
        expect("def", "err")
        .backtrack()
        .parse(&mut iter).record_to(&mut info),
        Err("err".into())
    );
    assert_eq!(
        info,
        ParseInfo::new(0, 3)
    );

    info = ParseInfo::default();
    assert_eq!(
        expect("abc", "test_failure")
        .parse(&mut iter).record_to(&mut info),
        Ok("abc".into())
    );
    assert_eq!(
        info,
        ParseInfo::new(3, 3)
    );

    info = ParseInfo::default();
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter).record_to(&mut info),
        Ok(())
    );
    assert_eq!(
        info,
        ParseInfo::new(0, 1)
    )
}

#[test]
fn test_backtrack_parse() {
    let mut iter = "abc".chars();

    let mut info = ParseInfo::default();
    assert_eq!(
        expect("abc", "test_failure")
        .backtrack_parse(&mut iter).record_to(&mut info),
        Ok("abc".into())
    );
    assert_eq!(
        info,
        ParseInfo::new(0, 3)
    );

    info = ParseInfo::default();
    assert_eq!(
        expect("def", "err")
        .backtrack_parse(&mut iter).record_to(&mut info),
        Err("err".into())
    );
    assert_eq!(
        info,
        ParseInfo::new(0, 3)
    );

    info = ParseInfo::default();
    assert_eq!(
        expect("abc", "test_failure")
        .parse(&mut iter).record_to(&mut info),
        Ok("abc".into())
    );
    assert_eq!(
        info,
        ParseInfo::new(3, 3)
    );

    info = ParseInfo::default();
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter).record_to(&mut info),
        Ok(())
    );
    assert_eq!(
        info,
        ParseInfo::new(0, 1)
    )
}

// Value mapping

#[test]
fn test_map() {
    let mut iter = "abc".chars();

    let mut info = ParseInfo::default();
    assert_eq!(
        expect("abc", "test_failure")
        .map(|s| s.to_uppercase())
        .parse(&mut iter).record_to(&mut info),
        Ok(format!("ABC"))
    );
    assert_eq!(
        info,
        ParseInfo::new(3, 3)
    );

    info = ParseInfo::default();
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter).record_to(&mut info),
        Ok(())
    );
    assert_eq!(
        info,
        ParseInfo::new(0, 1)
    )
}

#[test]
fn test_and_then() {
    let mut iter = "abc".chars();

    let mut info = ParseInfo::default();
    assert_eq!(
        expect("abc", "test_failure")
        .and_then(|s| Ok(s.to_uppercase()))
        .parse(&mut iter).record_to(&mut info),
        Ok("ABC".into())
    );
    assert_eq!(
        info,
        ParseInfo::new(3, 3)
    );

    info = ParseInfo::default();
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter).record_to(&mut info),
        Ok(())
    );
    assert_eq!(
        info,
        ParseInfo::new(0, 1)
    )
}

#[test]
fn test_and_compose() {
    let mut iter = "abcdef".chars();

    let mut info = ParseInfo::default();
    assert_eq!(
        expect("abc", "test_failure_0")
        .and_compose(expect("def", "test_failure_1"))
        .parse(&mut iter).record_to(&mut info),
        Ok("def".into())
    );
    assert_eq!(
        info,
        ParseInfo::new(6, 6)
    );

    info = ParseInfo::default();
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter).record_to(&mut info),
        Ok(())
    );
    assert_eq!(
        info,
        ParseInfo::new(0, 1)
    )
}

#[test]
fn test_preserve_and_compose() {
    let mut iter = "abcdef".chars();

    let mut info = ParseInfo::default();
    assert_eq!(
        expect("abc", "test_failure_0")
        .preserve_and_compose(expect("def", "test_failure_1"))
        .parse(&mut iter).record_to(&mut info),
        Ok("abc".into())
    );
    assert_eq!(
        info,
        ParseInfo::new(6, 6)
    );

    info = ParseInfo::default();
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter).record_to(&mut info),
        Ok(())
    );
    assert_eq!(
        info,
        ParseInfo::new(0, 1)
    )
}

#[test]
fn test_and_then_compose() {
    let mut iter = "abcABC".chars();

    let mut info = ParseInfo::default();
    assert_eq!(
        expect("abc", "test_failure_0")
        .and_then_compose(
            |s| expect(s.to_uppercase(), "test_failure_1")
        )
        .parse(&mut iter).record_to(&mut info),
        Ok("ABC".into())
    );
    assert_eq!(
        info,
        ParseInfo::new(6, 6)
    );

    info = ParseInfo::default();
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter).record_to(&mut info),
        Ok(())
    );
    assert_eq!(
        info,
        ParseInfo::new(0, 1)
    )
}

// Error mapping

#[test]
fn test_map_err() {
    let mut iter = "def".chars();

    let mut info = ParseInfo::default();
    assert_eq!(
        expect("abc", "err")
        .map_err(|e| e.to_uppercase())
        .parse(&mut iter).record_to(&mut info),
        Err("ERR".into())
    );
    assert_eq!(
        info,
        ParseInfo::new(3, 3)
    );

    info = ParseInfo::default();
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter).record_to(&mut info),
        Ok(())
    );
    assert_eq!(
        info,
        ParseInfo::new(0, 1)
    )
}

#[test]
fn test_or_else() {
    let mut iter = "def".chars();

    let mut info = ParseInfo::default();
    assert_eq!(
        expect("abc", "err")
        .or_else(|e| Err(e.to_uppercase()))
        .parse(&mut iter).record_to(&mut info),
        Err("ERR".into())
    );
    assert_eq!(
        info,
        ParseInfo::new(3, 3)
    );

    info = ParseInfo::default();
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter).record_to(&mut info),
        Ok(())
    );
    assert_eq!(
        info,
        ParseInfo::new(0, 1)
    )
}

#[test]
fn test_or_compose() {
    let mut iter = "defghi".chars();

    let mut info = ParseInfo::default();
    assert_eq!(
        expect("abc", "err")
        .or_compose(expect("ghi", "test_failure"))
        .parse(&mut iter).record_to(&mut info),
        Ok("ghi".into())
    );
    assert_eq!(
        info,
        ParseInfo::new(6, 6)
    );

    info = ParseInfo::default();
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter).record_to(&mut info),
        Ok(())
    );
    assert_eq!(
        info,
        ParseInfo::new(0, 1)
    )
}

#[test]
fn test_or_else_compose() {
    let mut iter = "defghijkl".chars();

    let mut info = ParseInfo::default();
    assert_eq!(
        expect("abc", "err")
        .or_else_compose(|e| expect("def", e))
        .parse(&mut iter).record_to(&mut info),
        Err("err".into())
    );
    assert_eq!(
        info,
        ParseInfo::new(6, 6)
    );

    info = ParseInfo::default();
    assert_eq!(
        expect("jkl", "test_failure")
        .or_else_compose(|e| expect("mno", e))
        .parse(&mut iter).record_to(&mut info),
        Ok("jkl".into())
    );
    assert_eq!(
        info,
        ParseInfo::new(3, 3)
    );

    info = ParseInfo::default();
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter).record_to(&mut info),
        Ok(())
    );
    assert_eq!(
        info,
        ParseInfo::new(0, 1)
    )
}

// Vector Combinators

#[test]
fn test_many() {
    let mut iter = "abcabcdefghi".chars();

    let mut info = ParseInfo::default();
    assert_eq!(
        expect("abc", "test_failure")
        .many()
        .map_err::<!, _>(|_: !| unreachable!())
        .parse(&mut iter).record_to(&mut info),
        Ok(vec!["abc".into(), "abc".into()])
    );
    assert_eq!(
        info,
        ParseInfo::new(9, 9)
    );

    info = ParseInfo::default();
    assert_eq!(
        expect("ghi", "test_failure")
        .parse(&mut iter).record_to(&mut info),
        Ok("ghi".into())
    );
    assert_eq!(
        info,
        ParseInfo::new(3, 3)
    );

    info = ParseInfo::default();
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter).record_to(&mut info),
        Ok(())
    );
    assert_eq!(
        info,
        ParseInfo::new(0, 1)
    )
}

#[test]
fn test_some() {
    let mut iter = "defghighijklmno".chars();

    let mut info = ParseInfo::default();
    assert_eq!(
        expect("abc", "err")
        .some()
        .parse(&mut iter).record_to(&mut info),
        Err("err".into())
    );
    assert_eq!(
        info,
        ParseInfo::new(3, 3)
    );

    info = ParseInfo::default();
    assert_eq!(
        expect("ghi", "test_failure")
        .some()
        .parse(&mut iter).record_to(&mut info),
        Ok(vec!["ghi".into(), "ghi".into()])
    );
    assert_eq!(
        info,
        ParseInfo::new(9, 9)
    );

    info = ParseInfo::default();
    assert_eq!(
        expect("mno", "test_failure")
        .parse(&mut iter).record_to(&mut info),
        Ok("mno".into())
    );
    assert_eq!(
        info,
        ParseInfo::new(3, 3)
    );

    info = ParseInfo::default();
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter).record_to(&mut info),
        Ok(())
    );
    assert_eq!(
        info,
        ParseInfo::new(0, 1)
    )
}

#[test]
fn test_least_until() {
    let mut iter = ".def.def.def:ghi".chars();

    let mut info = ParseInfo::default();
    assert_eq!(
        expect("abc", "test_failure")
        .least_until(expect(":", "err"))
        .parse(&mut iter).record_to(&mut info),
        Err("err".into())
    );
    assert_eq!(
        info,
        ParseInfo::new(4, 4)
    );

    info = ParseInfo::default();
    assert_eq!(
        expect("def", "test_failure_0")
        .least_until(expect(":", "test_failure_1"))
        .parse(&mut iter).record_to(&mut info),
        Ok((vec!["def".into(), "def".into()], ":".into()))
    );
    assert_eq!(
        info,
        ParseInfo::new(9, 9)
    );

    info = ParseInfo::default();
    assert_eq!(
        expect("ghi", "test_failure")
        .parse(&mut iter).record_to(&mut info),
        Ok("ghi".into())
    );
    assert_eq!(
        info,
        ParseInfo::new(3, 3)
    );

    info = ParseInfo::default();
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter).record_to(&mut info),
        Ok(())
    );
    assert_eq!(
        info,
        ParseInfo::new(0, 1)
    )
}

#[test]
fn test_most_until() {
    let mut iter = "abcabcdef".chars();

    let mut info = ParseInfo::default();
    assert_eq!(
        expect("abc", "test_failure_0")
        .most_until(expect("def", "test_failure_1"))
        .parse(&mut iter).record_to(&mut info),
        Ok((vec!["abc".into(), "abc".into()], "def".into()))
    );
    assert_eq!(
        info,
        ParseInfo::new(9, 9)
    );

    info = ParseInfo::default();
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter).record_to(&mut info),
        Ok(())
    );
    assert_eq!(
        info,
        ParseInfo::new(0, 1)
    )
}

// Error recovery

#[test]
fn test_continue_with() {
    let mut iter = "abcdefghijkl".chars();

    let mut info = ParseInfo::default();
    assert_eq!(
        expect("abc", "test_failure_0")
        .continue_with(expect("def", "test_failure_1").discard())
        .parse(&mut iter).record_to(&mut info),
        Ok(Ok("abc".into()))
    );
    assert_eq!(
        info,
        ParseInfo::new(6, 6)
    );

    info = ParseInfo::default();
    assert_eq!(
        expect("def", "err")
        .continue_with(expect("jkl", "test_failure").discard())
        .parse(&mut iter).record_to(&mut info),
        Ok(Err("err".into()))
    );
    assert_eq!(
        info,
        ParseInfo::new(6, 6)
    );

    info = ParseInfo::default();
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter).record_to(&mut info),
        Ok(())
    );
    assert_eq!(
        info,
        ParseInfo::new(0, 1)
    )
}

#[test]
fn test_recover_with() {
    let mut iter = "abcdefghi".chars();

    let mut info = ParseInfo::default();
    assert_eq!(
        expect("abc", "test_failure_0")
        .recover_with(expect("def", "test_failure_1").discard())
        .parse(&mut iter).record_to(&mut info),
        Ok(Ok("abc".into()))
    );
    assert_eq!(
        info,
        ParseInfo::new(3, 3)
    );

    info = ParseInfo::default();
    assert_eq!(
        expect("abc", "err")
        .recover_with(expect("ghi", "test_failure").discard())
        .parse(&mut iter).record_to(&mut info),
        Ok(Err("err".into()))
    );
    assert_eq!(
        info,
        ParseInfo::new(6, 6)
    );

    info = ParseInfo::default();
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter).record_to(&mut info),
        Ok(())
    );
    assert_eq!(
        info,
        ParseInfo::new(0, 1)
    )
}

#[test]
fn test_absorb_err() {
    let mut iter = "abcdef".chars();
    
    let mut info = ParseInfo::default();
    assert_eq!(
        expect("abc", "test_failure")
        .map(Result::Ok)
        .absorb_err()
        .parse(&mut iter).record_to(&mut info),
        Ok("abc".into())
    );
    assert_eq!(
        info,
        ParseInfo::new(3, 3)
    );

    info = ParseInfo::default();
    assert_eq!(
        expect("def", "test_failure")
        .map(|_| Err("err".into()))
        .absorb_err::<!>()
        .parse(&mut iter).record_to(&mut info),
        Err("err".into())
    );
    assert_eq!(
        info,
        ParseInfo::new(3, 3)
    );

    info = ParseInfo::default();
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter).record_to(&mut info),
        Ok(())
    );
    assert_eq!(
        info,
        ParseInfo::new(0, 1)
    )
}

// Functionality Testing

#[derive(Debug, PartialEq, Eq)]
enum TestRecExpr {
    Abc,
    Exprs(Vec<TestRecExpr>)
}

#[test]
fn test_recursive_capability() {
    use TestRecExpr::*;
    let expr_parser: ForwardDef<'_, std::str::Chars<'static>, TestRecExpr, String> = ForwardDef::new();
    let inner_expr_parser =
        expect("abc", "expected 'abc'").map(|_| Abc)
        .attempt()
        .or_compose(
            expect("(", "expected '('")
            .and_compose(
                expr_parser.reference()
                .most_until(
                    expect(")", "expected ')'")
                    .attempt()
                )
                .map(|(exprs, _)| Exprs(exprs))
            )
        );
    let Ok(()) = expr_parser.define(&inner_expr_parser) else {
        unreachable!();
    };
    let mut iter = "abc".chars();

    let mut info = ParseInfo::default();
    assert_eq!(
        expr_parser
        .parse(&mut iter).record_to(&mut info),
        Ok(
            Abc
        )
    );
    assert_eq!(
        info,
        ParseInfo::new(3, 3)
    );

    info = ParseInfo::default();
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter).record_to(&mut info),
        Ok(())
    );
    assert_eq!(
        info,
        ParseInfo::new(0, 1)
    );

    let mut iter = "(abcabcabc)".chars();

    let mut info = ParseInfo::default();
    assert_eq!(
        expr_parser
        .parse(&mut iter).record_to(&mut info),
        Ok(
            Exprs(vec![
                Abc,
                Abc,
                Abc
            ])
        )
    );
    assert_eq!(
        info,
        ParseInfo::new(11, 13)
    );

    info = ParseInfo::default();
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter).record_to(&mut info),
        Ok(())
    );
    assert_eq!(
        info,
        ParseInfo::new(0, 1)
    );

    let mut iter = "(abcabc(abcabc)abc)".chars();

    let mut info = ParseInfo::default();
    assert_eq!(
        expr_parser
        .parse(&mut iter).record_to(&mut info),
        Ok(
            Exprs(vec![
                Abc,
                Abc,
                Exprs(vec![
                    Abc,
                    Abc
                ]),
                Abc
            ])
        )
    );
    assert_eq!(
        info,
        ParseInfo::new(19, 21)
    );

    info = ParseInfo::default();
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter).record_to(&mut info),
        Ok(())
    );
    assert_eq!(
        info,
        ParseInfo::new(0, 1)
    )
}

#[test]
fn test_apply_macro() {
    let mut iter = "abcdefghi".chars();

    let mut info = ParseInfo::default();
    assert_eq!(
        apply!(
            |a, b, c| (a, b, c),
            expect("abc", "test_failure_0"),
            expect("def", "test_failure_1"),
            expect("ghi", "test_failure_2")

        )
        .parse(&mut iter).record_to(&mut info),
        Ok(("abc".into(), "def".into(), "ghi".into()))
    );
    assert_eq!(
        info,
        ParseInfo::new(9, 9)
    );

    info = ParseInfo::default();
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter).record_to(&mut info),
        Ok(())
    );
    assert_eq!(
        info,
        ParseInfo::new(0, 1)
    )
}