
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