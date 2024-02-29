use std::marker::PhantomData;
use super::*;

pub struct BranchInternalError<Pos, Tokens>(pub String, pub ParseError<Pos, Tokens>);
pub struct BranchEntryError<Tokens>(pub Option<String>, pub Tokens);

pub enum BranchError<Pos, Tokens> {
    Internal(BranchInternalError<Pos, Tokens>),
    Entry(BranchEntryError<Tokens>)
}

impl<Pos, Tokens> From<BranchInternalError<Pos, Tokens>> for BranchError<Pos, Tokens> {
    fn from(value: BranchInternalError<Pos, Tokens>) -> Self {
        BranchError::Internal(value)
    }
}

impl<Pos, Tokens> From<BranchEntryError<Tokens>> for BranchError<Pos, Tokens> {
    fn from(value: BranchEntryError<Tokens>) -> Self {
        BranchError::Entry(value)
    }
}

// when branching the latest internal error will be given else a collection of branch entry errors will be given

pub enum ParseError<Pos, Tokens> {
    ContextualisedError(Box<BranchInternalError<Pos, Tokens>>, Pos), // in region foo at pos: ...
    BranchingError(Vec<BranchEntryError<Tokens>>, Tokens, Pos), // expected xs... found x at pos
    Bundle(Vec<ParseError<Pos, Tokens>>) // a collection of errors
}

impl<Pos, Tokens> ParseError<Pos, Tokens> where {
    pub fn collect_branch<I>(iter: &I, errs: Vec<BranchError<Pos, Tokens>>, get_pos: impl Fn(&I) -> Pos) -> ParseError<Pos, Tokens> where
        I: Iterator + Clone,
        Tokens: FromIterator<I::Item>,
        for<'a> &'a Tokens: IntoIterator<Item=I::Item>
    {
        let mut starts: Vec<BranchEntryError<Tokens>> = vec![];
        for err in errs.into_iter().rev() {
            match err {
                BranchError::Internal(inner_err) => {
                    return ParseError::ContextualisedError(Box::new(inner_err), get_pos(iter))
                },
                BranchError::Entry(inner_err) => {
                    starts.push(inner_err);
                },
            }
        };
        let n = starts.iter()
            .map(|tokens| tokens.1.into_iter().count())
            .max();
        let found: Tokens = iter.clone().take(n.unwrap_or(0)).collect();
        ParseError::BranchingError(starts, found, get_pos(iter))
    }
}

#[derive(Clone)]
pub struct Region<S, B>{
    name: String,
    start: S,
    body: B,
}

impl<S, B> Region<S, B> {
    pub fn new(name: String, start: S, body: B) -> Region<S, B> {
        Region {
            name,
            start,
            body,
        }
    }
}

impl<I, S, B, Pos, Tokens> Parser<I> for Region<S, B> where
    I: Iterator + Clone,
    S: Parser<I, Error=Tokens>,
    B: Parser<I, Error=ParseError<Pos, Tokens>>
{
    type Value = B::Value;
    type Error = BranchError<Pos, Tokens>;

    fn parse(&self, iter: &mut I, info: &mut ParseInfo) -> Result<Self::Value, Self::Error> {
        self.start
            .parse(iter, info)
            .map_err(|err|
                BranchEntryError(Some(self.name.clone()), err).into()
            )
            .and_then(|_| self.body
                .parse(iter, info)
                .map_err(|err|
                    BranchInternalError(self.name.clone(), err).into()
                )
            )
    }
}

#[derive(Clone)]
struct Stub<S, Pos> {
    name: Option<String>,
    start: S,
    _p: PhantomData<Pos>,
}

impl<S, Tok> Stub<S, Tok> {
    pub fn new(name: Option<String>, start: S) -> Stub<S, Tok> {
        Stub {
            name,
            start,
            _p: PhantomData,
        }
    }
}

impl<I, S, Pos, Tokens> Parser<I> for Stub<S, Pos> where
    I: Iterator + Clone,
    S: Parser<I, Error=Tokens>
{
    type Value = S::Value;
    type Error = BranchError<Pos, Tokens>;

    fn parse(&self, iter: &mut I, info: &mut ParseInfo) -> Result<Self::Value, Self::Error> {
        self.start
            .parse(iter, info)
            .map_err(|err|
                BranchEntryError(self.name.clone(), err).into()
            )
    }
}

// Branching

pub macro branch {
    ($g:expr) => {
        Branch0::new($g)
    },
    ($g:expr, $b1:expr) => {
        Branch1::new($g, $b1)
    },
    ($g:expr, $b1:expr, $b2:expr) => {
        Branch2::new($g, $b1, $b2)
    },
    ($g:expr, $b1:expr, $b2:expr, $b3:expr) => {
        Branch4::new($g, $b1, $b2, $b3)
    },
    ($g:expr, $b1:expr, $b2:expr, $b3:expr, $b4:expr) => {
        Branch4::new($g, $b1, $b2, $b3, $b4)
    }
}

#[derive(Clone)]
pub struct Branch0<F, T, Tokens> {
    get_pos: F,
    _t: PhantomData<T>,
    _tk: PhantomData<Tokens>,
}

impl<F, T, Tokens> Branch0<F, T, Tokens> {
    pub fn new(get_pos: F) -> Branch0<F, T, Tokens> {
        Branch0 {
            get_pos,
            _t: PhantomData,
            _tk: PhantomData,
        }
    }
}

impl<I, F, T, Pos, Tokens> Parser<I> for Branch0<F, T, Tokens> where
    I: Iterator + Clone,
    F: Fn(&I) -> Pos,
    Tokens: FromIterator<I::Item>,
    for<'a> &'a Tokens: IntoIterator<Item=I::Item>
{
    type Value = T;
    type Error = ParseError<Pos, Tokens>;

    fn parse(&self, iter: &mut I, _info: &mut ParseInfo) -> Result<Self::Value, Self::Error> {
        let branch_errors = vec![];
        Err(ParseError::collect_branch(iter, branch_errors, &self.get_pos))
    }
}

#[derive(Clone)]
pub struct Branch1<F, B1> {
    get_pos: F,
    b1: B1,
}

impl<F, B1> Branch1<F, B1> {
    pub fn new(get_pos: F, b1: B1) -> Branch1<F, B1> {
        Branch1 {
            get_pos,
            b1,
        }
    }
}

impl<I, F, B1, T, Pos, Tokens> Parser<I> for Branch1<F, B1> where
    I: Iterator + Clone,
    F: Fn(&I) -> Pos,
    B1: Parser<I, Value=T, Error=BranchError<Pos, Tokens>>,
    Tokens: FromIterator<I::Item>,
    for<'a> &'a Tokens: IntoIterator<Item=I::Item>
{
    type Value = T;
    type Error = ParseError<Pos, Tokens>;

    fn parse(&self, iter: &mut I, info: &mut ParseInfo) -> Result<Self::Value, Self::Error> {
        let mut branch_errors = vec![];
        match self.b1.parse(iter, info) {
            Ok(val) => return Ok(val),
            Err(err) => branch_errors.push(err),
        }
        Err(ParseError::collect_branch(iter, branch_errors, &self.get_pos))
    }
}

#[derive(Clone)]
pub struct Branch2<F, B1, B2> {
    get_pos: F,
    b1: B1,
    b2: B2,
}

impl<F, B1, B2> Branch2<F, B1, B2> {
    pub fn new(get_pos: F, b1: B1, b2: B2) -> Branch2<F, B1, B2> {
        Branch2 {
            get_pos,
            b1,
            b2,
        }
    }
}

impl<I, F, B1, B2, T, Pos, Tokens> Parser<I> for Branch2<F, B1, B2> where
    I: Iterator + Clone,
    F: Fn(&I) -> Pos,
    B1: Parser<I, Value=T, Error=BranchError<Pos, Tokens>>,
    B2: Parser<I, Value=T, Error=BranchError<Pos, Tokens>>,
    Tokens: FromIterator<I::Item>,
    for<'a> &'a Tokens: IntoIterator<Item=I::Item>
{
    type Value = T;
    type Error = ParseError<Pos, Tokens>;

    fn parse(&self, iter: &mut I, info: &mut ParseInfo) -> Result<Self::Value, Self::Error> {
        let mut branch_errors = vec![];
        match self.b1.parse(iter, info) {
            Ok(val) => return Ok(val),
            Err(err) => branch_errors.push(err),
        }
        match self.b2.parse(iter, info) {
            Ok(val) => return Ok(val),
            Err(err) => branch_errors.push(err),
        }
        Err(ParseError::collect_branch(iter, branch_errors, &self.get_pos))
    }
}

#[derive(Clone)]
pub struct Branch3<F, B1, B2, B3> {
    get_pos: F,
    b1: B1,
    b2: B2,
    b3: B3,
}

impl<F, B1, B2, B3> Branch3<F, B1, B2, B3> {
    pub fn new(get_pos: F, b1: B1, b2: B2, b3: B3) -> Branch3<F, B1, B2, B3> {
        Branch3 {
            get_pos,
            b1,
            b2,
            b3,
        }
    }
}

impl<I, F, B1, B2, B3, T, Pos, Tokens> Parser<I> for Branch3<F, B1, B2, B3> where
    I: Iterator + Clone,
    F: Fn(&I) -> Pos,
    B1: Parser<I, Value=T, Error=BranchError<Pos, Tokens>>,
    B2: Parser<I, Value=T, Error=BranchError<Pos, Tokens>>,
    B3: Parser<I, Value=T, Error=BranchError<Pos, Tokens>>,
    Tokens: FromIterator<I::Item>,
    for<'a> &'a Tokens: IntoIterator<Item=I::Item>
{
    type Value = T;
    type Error = ParseError<Pos, Tokens>;

    fn parse(&self, iter: &mut I, info: &mut ParseInfo) -> Result<Self::Value, Self::Error> {
        let mut branch_errors = vec![];
        match self.b1.parse(iter, info) {
            Ok(val) => return Ok(val),
            Err(err) => branch_errors.push(err),
        }
        match self.b2.parse(iter, info) {
            Ok(val) => return Ok(val),
            Err(err) => branch_errors.push(err),
        }
        match self.b3.parse(iter, info) {
            Ok(val) => return Ok(val),
            Err(err) => branch_errors.push(err),
        }
        Err(ParseError::collect_branch(iter, branch_errors, &self.get_pos))
    }
}

#[derive(Clone)]
pub struct Branch4<F, B1, B2, B3, B4> {
    get_pos: F,
    b1: B1,
    b2: B2,
    b3: B3,
    b4: B4,
}

impl<F, B1, B2, B3, B4> Branch4<F, B1, B2, B3, B4> {
    pub fn new(get_pos: F, b1: B1, b2: B2, b3: B3, b4: B4) -> Branch4<F, B1, B2, B3, B4> {
        Branch4 {
            get_pos,
            b1,
            b2,
            b3,
            b4,
        }
    }
}

impl<I, F, B1, B2, B3, B4, T, Pos, Tokens> Parser<I> for Branch4<F, B1, B2, B3, B4> where
    I: Iterator + Clone,
    F: Fn(&I) -> Pos,
    B1: Parser<I, Value=T, Error=BranchError<Pos, Tokens>>,
    B2: Parser<I, Value=T, Error=BranchError<Pos, Tokens>>,
    B3: Parser<I, Value=T, Error=BranchError<Pos, Tokens>>,
    B4: Parser<I, Value=T, Error=BranchError<Pos, Tokens>>,
    Tokens: FromIterator<I::Item>,
    for<'a> &'a Tokens: IntoIterator<Item=I::Item>
{
    type Value = T;
    type Error = ParseError<Pos, Tokens>;

    fn parse(&self, iter: &mut I, info: &mut ParseInfo) -> Result<Self::Value, Self::Error> {
        let mut branch_errors = vec![];
        match self.b1.parse(iter, info) {
            Ok(val) => return Ok(val),
            Err(err) => branch_errors.push(err),
        }
        match self.b2.parse(iter, info) {
            Ok(val) => return Ok(val),
            Err(err) => branch_errors.push(err),
        }
        match self.b3.parse(iter, info) {
            Ok(val) => return Ok(val),
            Err(err) => branch_errors.push(err),
        }
        match self.b4.parse(iter, info) {
            Ok(val) => return Ok(val),
            Err(err) => branch_errors.push(err),
        }
        Err(ParseError::collect_branch(iter, branch_errors, &self.get_pos))
    }
}