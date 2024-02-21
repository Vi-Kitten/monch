use super::*;

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

fn expect<T, E, I>(t: T, e: E) -> impl Parser<I, String, String> where
    T: Into<String>,
    E: Into<String>,
    I: Iterator<Item = char> + Clone
{
    let t_string = t.into();
    let e_string = e.into();
    move |iter: &mut I| {
        if iter.take(t_string.len()).collect::<String>() == t_string {
            Ok(t_string.clone())
        } else {
            Err(e_string.clone())
        }
    }
}

fn expect_end<E, I>(e: E) -> impl Parser<I, (), String> where
    E: Into<String>,
    I: Iterator<Item = char> + Clone
{
    let e_string = e.into();
    move |iter: &mut I| {
        if let None = iter.next() {
            Ok(())
        } else {
            Err(e_string.clone())
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
    );
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter),
        Ok(())
    )
}

#[test]
fn test_parse_ok() {
    let mut iter = "".chars();
    assert_eq!(
        parse_ok("abc".into())
        .map_err::<!, _>(|_: !| unreachable!())
        .parse(&mut iter),
        Ok("abc")
    );
    assert_eq!(
        expect_end("test_faileure")
        .parse(&mut iter),
        Ok(())
    )
}

#[test]
fn test_parse_err() {
    let mut iter = "".chars();
    assert_eq!(
        parse_err(format!("err"))
        .map::<!, _>(|_: !| unreachable!())
        .parse(&mut iter),
        Err("err".into())
    );
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter),
        Ok(())
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
    );
    assert_eq!(
        expect_end("test_failure")
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
    );
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter),
        Ok(())
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
        Err("err".into())
    );
    assert_eq!(
        expect("abc", "test_failure")
        .parse(&mut iter),
        Ok("abc".into())
    );
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter),
        Ok(())
    )
}

#[test]
fn test_attempt_parse() {
    let mut iter = "abc".chars();
    assert_eq!(
        expect("def", "err")
        .attempt_parse(&mut iter),
        Err("err".into())
    );
    assert_eq!(
        expect("abc", "test_failure")
        .parse(&mut iter),
        Ok("abc".into())
    );
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter),
        Ok(())
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
    );
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter),
        Ok(())
    )
}

#[test]
fn test_scry_parse() {
    let mut iter = "abc".chars();
    assert_eq!(
        expect("abc", "test_failure")
        .scry_parse(&mut iter),
        Ok("abc".into())
    );
    assert_eq!(
        expect("abc", "test_failure")
        .parse(&mut iter),
        Ok("abc".into())
    );
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter),
        Ok(())
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
        Err("err".into())
    );
    assert_eq!(
        expect("abc", "test_failure")
        .parse(&mut iter),
        Ok("abc".into())
    );
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter),
        Ok(())
    )
}

#[test]
fn test_backtrack_parse() {
    let mut iter = "abc".chars();
    assert_eq!(
        expect("abc", "test_failure")
        .backtrack_parse(&mut iter),
        Ok("abc".into())
    );
    assert_eq!(
        expect("def", "err")
        .backtrack_parse(&mut iter),
        Err("err".into())
    );
    assert_eq!(
        expect("abc", "test_failure")
        .parse(&mut iter),
        Ok("abc".into())
    );
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter),
        Ok(())
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
    );
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter),
        Ok(())
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
    );
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter),
        Ok(())
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
    );
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter),
        Ok(())
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
    );
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter),
        Ok(())
    )
}

#[test]
fn test_and_compose() {
    let mut iter = "abcdef".chars();
    assert_eq!(
        expect("abc", "test_failure_0")
        .and_compose(expect("def", "test_failure_1"))
        .parse(&mut iter),
        Ok("def".into())
    );
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter),
        Ok(())
    )
}

#[test]
fn test_scry_and_compose() {
    let mut iter = "abcdefghi".chars();
    assert_eq!(
        expect("abc", "test_failure_0")
        .scry_and_compose(expect("abcdef", "test_failure_0"))
        .parse(&mut iter),
        Ok("abcdef".into())
    );
    assert_eq!(
        expect("ghi", "test_failure").parse(&mut iter),
        Ok("ghi".into())
    );
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter),
        Ok(())
    )
}

#[test]
fn test_backtrack_and_compose() {
    let mut iter = "abcdefghi".chars();
    assert_eq!(
        expect("abc", "test_failure_0")
        .backtrack_and_compose(expect("abcdef", "test_failure_1"))
        .parse(&mut iter),
        Ok("abcdef".into())
    );
    assert_eq!(
        expect("abc", "err")
        .backtrack_and_compose(expect("abcdef", "test_failure"))
        .parse(&mut iter),
        Err("err".into())
    );
    assert_eq!(
        expect("ghi", "test_failure")
        .parse(&mut iter),
        Ok("ghi".into())
    );
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter),
        Ok(())
    )
}

#[test]
fn test_and_then_compose() {
    let mut iter = "abcABC".chars();
    assert_eq!(
        expect("abc", "test_failure_0")
        .and_then_compose(
            |s| expect(s.to_uppercase(), "test_failure_1")
        )
        .parse(&mut iter),
        Ok("ABC".into())
    );
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter),
        Ok(())
    )
}

#[test]
fn test_scry_and_then_compose() {
    let mut iter = "abcABCdef".chars();
    assert_eq!(
        expect("abc", "test_failure_0")
        .scry_and_then_compose(|s|
            expect(format!("abc{}", s.to_uppercase()), "test_failure_1")
        )
        .parse(&mut iter),
        Ok("abcABC".into())
    );
    assert_eq!(
        expect("def", "test_failure")
        .parse(&mut iter),
        Ok("def".into())
    );
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter),
        Ok(())
    )
}

#[test]
fn test_backtrack_and_then_compose() {
    let mut iter = "abcABCdef".chars();
    assert_eq!(
        expect("abc", "test_failure_0")
        .backtrack_and_then_compose(|s|
            expect(format!("abc{}", s.to_uppercase()), "test_failure_1")
        )
        .parse(&mut iter),
        Ok("abcABC".into())
    );
    assert_eq!(
        expect("abc", "err")
        .backtrack_and_then_compose(|s|
            expect(format!("abc{}", s.to_uppercase()), "test_failure")
        )
        .parse(&mut iter),
        Err("err".into())
    );
    assert_eq!(
        expect("def", "test_failure")
        .parse(&mut iter),
        Ok("def".into())
    );
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter),
        Ok(())
    )
}

//* Error mapping

#[test]
fn test_map_err() {
    let mut iter = "def".chars();
    assert_eq!(
        expect("abc", "err")
        .map_err(|e| e.to_uppercase())
        .parse(&mut iter),
        Err("ERR".into())
    );
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter),
        Ok(())
    )
}

// variants:
// attempt_or...
// backtrack_or...

#[test]
fn test_or_else() {
    let mut iter = "def".chars();
    assert_eq!(
        expect("abc", "err")
        .or_else(|e| Err(e.to_uppercase()))
        .parse(&mut iter),
        Err("ERR".into())
    );
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter),
        Ok(())
    )
}

#[test]
fn test_attempt_or_else() {
    let mut iter = "def".chars();
    assert_eq!(
        expect("abc", "err")
        .attempt_or_else(|e| Err(e.to_uppercase()))
        .parse(&mut iter),
        Err("ERR".into())
    );
    assert_eq!(
        expect("def", "err")
        .attempt_or_else(|e| Err(e.to_uppercase()))
        .parse(&mut iter),
        Ok("def".into())
    );
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter),
        Ok(())
    )
}

#[test]
fn test_backtrack_or_else() {
    let mut iter = "def".chars();
    assert_eq!(
        expect("abc", "err")
        .backtrack_or_else(|e| Err(e.to_uppercase()))
        .parse(&mut iter),
        Err("ERR".into())
    );
    assert_eq!(
        expect("def", "err")
        .backtrack_or_else(|e| Err(e.to_uppercase()))
        .parse(&mut iter),
        Ok("def".into())
    );
    assert_eq!(
        expect("def", "test_failure")
        .parse(&mut iter),
        Ok("def".into())
    );
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter),
        Ok(())
    )
}

#[test]
fn test_or_compose() {
    let mut iter = "defghi".chars();
    assert_eq!(
        expect("abc", "err")
        .or_compose(expect("ghi", "test_failure"))
        .parse(&mut iter),
        Ok("ghi".into())
    );
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter),
        Ok(())
    )
}

#[test]
fn test_attempt_or_compose() {
    let mut iter = "defghi".chars();
    assert_eq!(
        expect("abc", "err")
        .attempt_or_compose(expect("def", "test_failure"))
        .parse(&mut iter),
        Ok("def".into())
    );
    assert_eq!(
        expect("ghi", "test_failure_0")
        .attempt_or_compose(expect("ijk", "test_failure_1"))
        .parse(&mut iter),
        Ok("ghi".into())
    );
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter),
        Ok(())
    )
}

#[test]
fn test_backtrack_or_compose() {
    let mut iter = "defghijkl".chars();
    assert_eq!(
        expect("abc", "err")
        .backtrack_or_compose(expect("def", "test_failure"))
        .parse(&mut iter),
        Ok("def".into())
    );
    assert_eq!(
        expect("ghi", "test_failure_0")
        .backtrack_or_compose(expect("ghijkl", "test_failure_1"))
        .parse(&mut iter),
        Ok("ghi".into())
    );
    assert_eq!(
        expect("ghijkl", "test_failure")
        .parse(&mut iter),
        Ok("ghijkl".into())
    );
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter),
        Ok(())
    )
}

#[test]
fn test_or_else_compose() {
    let mut iter = "defghijkl".chars();
    assert_eq!(
        expect("abc", "err")
        .or_else_compose(|e| expect("def", e))
        .parse(&mut iter),
        Err("err".into())
    );
    assert_eq!(
        expect("jkl", "test_failure")
        .or_else_compose(|e| expect("mno", e))
        .parse(&mut iter),
        Ok("jkl".into())
    );
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter),
        Ok(())
    )
}

#[test]
fn test_attempt_or_else_compose() {
    let mut iter = "ghijkl".chars();
    assert_eq!(
        expect("abc", "err")
        .attempt_or_else_compose(|e| expect("def", e))
        .parse(&mut iter),
        Err("err".into())
    );
    assert_eq!(
        expect("jkl", "test_failure")
        .attempt_or_else_compose(|e| expect("mno", e))
        .parse(&mut iter),
        Ok("jkl".into())
    );
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter),
        Ok(())
    )
}

#[test]
fn test_backtrack_or_else_compose() {
    let mut iter = "ghijkl".chars();
    assert_eq!(
        expect("abc", "err")
        .backtrack_or_else_compose(|e| expect("def", e))
        .parse(&mut iter),
        Err("err".into())
    );
    assert_eq!(
        expect("jkl", "test_failure")
        .backtrack_or_else_compose(|e| expect("mno", e))
        .parse(&mut iter),
        Ok("jkl".into())
    );
    assert_eq!(
        expect("jkl", "test_failure")
        .parse(&mut iter),
        Ok("jkl".into())
    );
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter),
        Ok(())
    )
}

//* Vector Combinators

#[test]
fn test_many() {
    let mut iter = "abcabcdefghi".chars();
    assert_eq!(
        expect("abc", "test_failure")
        .many()
        .map_err::<!, _>(|_: !| unreachable!())
        .parse(&mut iter),
        Ok(vec!["abc".into(), "abc".into()])
    );
    assert_eq!(
        expect("ghi", "test_failure")
        .parse(&mut iter),
        Ok("ghi".into())
    );
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter),
        Ok(())
    )
}

#[test]
fn test_attempt_many() {
    let mut iter = "abcabcdef".chars();
    assert_eq!(
        expect("abc", "test_failure")
        .attempt_many()
        .map_err::<!, _>(|_: !| unreachable!())
        .parse(&mut iter),
        Ok(vec!["abc".into(), "abc".into()])
    );
    assert_eq!(
        expect("def", "test_failure")
        .parse(&mut iter),
        Ok("def".into())
    );
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter),
        Ok(())
    )
}

#[test]
fn test_some() {
    let mut iter = "defghighijklmno".chars();
    assert_eq!(
        expect("abc", "err")
        .some()
        .parse(&mut iter),
        Err("err".into())
    );
    assert_eq!(
        expect("ghi", "test_failure")
        .some()
        .parse(&mut iter),
        Ok(vec!["ghi".into(), "ghi".into()])
    );
    assert_eq!(
        expect("mno", "test_failure")
        .parse(&mut iter),
        Ok("mno".into())
    );
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter),
        Ok(())
    )
}

#[test]
fn test_attempt_some() {
    let mut iter = "defdefghi".chars();
    assert_eq!(
        expect("abc", "err")
        .attempt_some()
        .parse(&mut iter),
        Err("err".into())
    );
    assert_eq!(
        expect("def", "test_failure")
        .attempt_some()
        .parse(&mut iter),
        Ok(vec!["def".into(), "def".into()])
    );
    assert_eq!(
        expect("ghi", "test_failure")
        .parse(&mut iter),
        Ok("ghi".into())
    );
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter),
        Ok(())
    )
}

#[test]
fn test_least_until() {
    let mut iter = ".def.def.def:ghi".chars();
    assert_eq!(
        expect("abc", "test_failure")
        .least_until(expect(":", "err"))
        .parse(&mut iter),
        Err("err".into())
    );
    assert_eq!(
        expect("def", "test_failure_0")
        .least_until(expect(":", "test_failure_1"))
        .parse(&mut iter),
        Ok((vec!["def".into(), "def".into()], ":".into()))
    );
    assert_eq!(
        expect("ghi", "test_failure")
        .parse(&mut iter),
        Ok("ghi".into())
    );
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter),
        Ok(())
    )
}

#[test]
fn test_attempt_least_until() {
    let mut iter = ".def.ghi.ghi:jkl".chars();
    assert_eq!(
        expect("abc", "test_failure")
        .attempt_least_until(expect(":", "err"))
        .parse(&mut iter),
        Err("err".into())
    );
    assert_eq!(
        expect("def", "test_failure")
        .parse(&mut iter),
        Ok("def".into())
    );
    assert_eq!(
        expect("ghi", "test_failure_0")
        .attempt_least_until(expect(":", "test_failure_1"))
        .parse(&mut iter),
        Ok((vec!["ghi".into(), "ghi".into()], ":".into()))
    );
    assert_eq!(
        expect("jkl", "test_failure")
        .parse(&mut iter),
        Ok("jkl".into())
    );
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter),
        Ok(())
    )
}

#[test]
fn test_attempt_most_until() {
    let mut iter = "abcabcdef".chars();
    assert_eq!(
        expect("abc", "test_failure_0")
        .attempt_most_until(expect("def", "test_failure_1"))
        .parse(&mut iter),
        Ok((vec!["abc".into(), "abc".into()], "def".into()))
    );
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter),
        Ok(())
    )
}

//* Error recovery

#[test]
fn test_continue_with() {
    let mut iter = "abcdefghijkl".chars();
    assert_eq!(
        expect("abc", "test_failure_0")
        .continue_with(expect("def", "test_failure_1").discard())
        .parse(&mut iter),
        Ok(Ok("abc".into()))
    );
    assert_eq!(
        expect("def", "err")
        .continue_with(expect("jkl", "test_failure").discard())
        .parse(&mut iter),
        Ok(Err("err".into()))
    );
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter),
        Ok(())
    )
}

#[test]
fn test_scry_then_continue_with() {
    let mut iter = "abcdefghijkl".chars();
    assert_eq!(
        expect("abc", "test_failure_0")
        .scry_then_continue_with(expect("abcdef", "test_failure_1").discard())
        .parse(&mut iter),
        Ok(Ok("abc".into()))
    );
    assert_eq!(
        expect("def", "err")
        .scry_then_continue_with(expect("jkl", "test_failure").discard())
        .parse(&mut iter),
        Ok(Err("err".into()))
    );
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter),
        Ok(())
    )
}

#[test]
fn test_backtrack_then_continue_with() {
    let mut iter = "abcdefghijkl".chars();
    assert_eq!(
        expect("abc", "test_failure_0")
        .backtrack_then_continue_with(expect("abcdef", "test_failure_1").discard())
        .parse(&mut iter),
        Ok(Ok("abc".into()))
    );
    assert_eq!(
        expect("def", "err")
        .backtrack_then_continue_with(expect("ghijkl", "test_failure").discard())
        .parse(&mut iter),
        Ok(Err("err".into()))
    );
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter),
        Ok(())
    )
}

#[test]
fn test_recover_with() {
    let mut iter = "abcdefghi".chars();
    assert_eq!(
        expect("abc", "test_failure_0")
        .recover_with(expect("def", "test_failure_1").discard())
        .parse(&mut iter),
        Ok(Ok("abc".into()))
    );
    assert_eq!(
        expect("abc", "err")
        .recover_with(expect("ghi", "test_failure").discard())
        .parse(&mut iter),
        Ok(Err("err".into()))
    );
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter),
        Ok(())
    )
}

#[test]
fn test_attempt_then_recover_with() {
    let mut iter = "abcdefghi".chars();
    assert_eq!(
        expect("abc", "test_failure_0")
        .attempt_then_recover_with(expect("def", "test_failure_1").discard())
        .parse(&mut iter),
        Ok(Ok("abc".into()))
    );
    assert_eq!(
        expect("abc", "err")
        .attempt_then_recover_with(expect("defghi", "test_failure").discard())
        .parse(&mut iter),
        Ok(Err("err".into()))
    );
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter),
        Ok(())
    )
}

#[test]
fn test_backtrack_then_recover_with() {
    let mut iter = "abcdef".chars();
    assert_eq!(
        expect("abc", "test_failure_0")
        .backtrack_then_recover_with(expect("def", "test_failure_1").discard())
        .parse(&mut iter),
        Ok(Ok("abc".into()))
    );
    assert_eq!(
        expect("abcghi", "err")
        .backtrack_then_recover_with(expect("abcdef", "test_failure").discard())
        .parse(&mut iter),
        Ok(Err("err".into()))
    );
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter),
        Ok(())
    )
}

#[test]
fn test_absorb_err() {
    let mut iter = "abcdef".chars();
    assert_eq!(
        expect("abc", "test_failure")
        .map(Result::Ok)
        .absorb_err()
        .parse(&mut iter),
        Ok("abc".into())
    );
    assert_eq!(
        expect("def", "test_failure")
        .map(|_| Err("err".into()))
        .absorb_err::<!>()
        .parse(&mut iter),
        Err("err".into())
    );
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter),
        Ok(())
    )
}

//* Functionality Testing

#[derive(Debug, PartialEq, Eq)]
enum TestRecExpr {
    Abc,
    Exprs(Vec<TestRecExpr>)
}

#[test]
fn test_recursive_capability() {
    use TestRecExpr::*;
    fn expr_parser() -> impl Parser<std::str::Chars<'static>, TestRecExpr, String> {
        recursive!( // required to create recursively defined lambda type
            expect("abc", "expected 'abc'").map(|_| Abc)
            .attempt_or_compose(
                expect("(", "expected '('")
                .and_compose(
                    Box::new(indirection::LazyParser::new(expr_parser))
                    .attempt_most_until(
                        expect(")", "expected ')'")
                    )
                    .map(|(exprs, _)| Exprs(exprs))
                )
            )
        )
    }
    let mut iter = "abc".chars();
    assert_eq!(
        expr_parser()
        .parse(&mut iter),
        Ok(
            Abc
        )
    );
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter),
        Ok(())
    );
    let mut iter = "(abcabcabc)".chars();
    assert_eq!(
        expr_parser()
        .parse(&mut iter),
        Ok(
            Exprs(vec![
                Abc,
                Abc,
                Abc
            ])
        )
    );
    assert_eq!(
        expect_end("test_failure")
        .parse(&mut iter),
        Ok(())
    );
    let mut iter = "(abcabc(abcabc)abc)".chars();
    assert_eq!(
        expr_parser()
        .parse(&mut iter),
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
        expect_end("test_failure")
        .parse(&mut iter),
        Ok(())
    )
}