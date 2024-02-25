
pub enum Found<Tokens> {
    Tokens(Vec<Tokens>)
}

pub enum ParseError<Tokens, Msg = String> {
    UnexpectedEnd,
    Expected(Vec<Tokens>, Found<Tokens>),
    Context(Msg, Box<ParseError<Tokens, Msg>>),
    Bundle(Vec<ParseError<Tokens, Msg>>)
}