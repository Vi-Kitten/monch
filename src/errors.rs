
pub struct BranchInternalError<Pos, Tokens>(String, ParseError<Pos, Tokens>);
pub struct BranchEntryError<Tokens>(String, Tokens);

pub type BranchError<Pos, Tokens> = Result<BranchInternalError<Pos, Tokens>, BranchEntryError<Tokens>>;

// when branching the latest internal error will be given else a collection of branch entry errors will be given

pub enum ParseError<Pos, Tokens> {
    ContextualisedError(Box<BranchInternalError<Pos, Tokens>>, Pos),
    BranchingError(Vec<BranchEntryError<Tokens>>, Tokens, Pos),
    ExpectedFound(Tokens, Tokens, Pos),
    Bundle(Vec<ParseError<Pos, Tokens>>)
}