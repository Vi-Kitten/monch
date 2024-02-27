use super::*;

pub struct BranchInternalError<Error>(pub String, pub Error);
pub struct BranchEntryError<Tokens>(pub String, pub Tokens);

pub type BranchError<Pos, Tokens> = Result<BranchInternalError<ParseError<Pos, Tokens>>, BranchEntryError<Tokens>>;

// when branching the latest internal error will be given else a collection of branch entry errors will be given

pub enum ParseError<Pos, Tokens> {
    ContextualisedError(Box<BranchInternalError<ParseError<Pos, Tokens>>>, Pos),
    BranchingError(Vec<BranchEntryError<Tokens>>, Tokens, Pos),
    ExpectedFound(Tokens, Tokens, Pos),
    Bundle(Vec<ParseError<Pos, Tokens>>)
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

impl<I, S, B> Parser<I> for Region<S, B> where
    I: Iterator + Clone,
    S: Parser<I>,
    B: Parser<I>
{
    type Value = B::Value;
    type Error = Result<BranchInternalError<B::Error>, BranchEntryError<S::Error>>;

    fn parse(&self, iter: &mut I, info: &mut ParseInfo) -> Result<Self::Value, Self::Error> {
        self.start
            .parse(iter, info)
            .map_err(|err|
                Result::Err(BranchEntryError(self.name.clone(), err))
            )
            .and_then(|_| self.body
                .parse(iter, info)
                .map_err(|err|
                    Result::Ok(BranchInternalError(self.name.clone(), err))
                )
            )
    }
}