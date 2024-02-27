use super::*;

pub trait MemoHandler<I, T, E>: Parser<I, Value=Result<T, E>> where
    I: Iterator + Clone,
{
    fn learn(&self, iter: I, info: ParseInfo, res: Result<T, E>);
}

#[derive(Clone)]
pub struct Memo<P, H> {
    parser: P,
    handler: H,
}

impl<P, H> Memo<P, H> {
    pub fn new(parser: P, handler: H) -> Memo<P, H> {
        Memo {
            parser,
            handler,
        }
    }
}

impl<I, P, H, T, E> Parser<I> for Memo<P, H> where
    I: Iterator + Clone,
    P: Parser<I, Value=T, Error=E>,
    H: MemoHandler<I, T, E>,
    T: Clone,
    E: Clone
{
    type Value = T;
    type Error = E;

    fn parse(&self, iter: &mut I, info: &mut ParseInfo) -> Result<T, E> {
        self.handler
            .parse(iter, info)
            .or_else(|_| {
                let mut inner_info = ParseInfo::default();
                let start_iter = iter.clone();
                let res = self.parser.parse(iter, &mut inner_info);
                *info += inner_info;
                self.handler.learn(start_iter, inner_info, res.clone());
                Ok::<_, !>(res)
            })
            .unwrap()
    }
}

#[derive(Clone)]
pub struct MemoIf<P, H, F> {
    parser: P,
    handler: H,
    predicate: F,
}

impl<P, H, F> MemoIf<P, H, F> {
    pub fn new(parser: P, handler: H, predicate: F) -> MemoIf<P, H, F> {
        MemoIf {
            parser,
            handler,
            predicate,
        }
    }
}

impl<I, P, H, F, T, E> Parser<I> for MemoIf<P, H, F> where
    I: Iterator + Clone,
    P: Parser<I, Value=T, Error=E>,
    H: MemoHandler<I, T, E>,
    F: Fn(ParseInfo, &Result<T, E>) -> bool,
    T: Clone,
    E: Clone
{
    type Value = T;
    type Error = E;

    fn parse(&self, iter: &mut I, info: &mut ParseInfo) -> Result<T, E> {
        self.handler
            .parse(iter, info)
            .or_else(|_| {
                let mut inner_info = ParseInfo::default();
                let start_iter = iter.clone();
                let res = self.parser.parse(iter, &mut inner_info);
                *info += inner_info;
                if (self.predicate)(inner_info, &res) {
                    self.handler.learn(start_iter, inner_info, res.clone());
                }
                Ok::<_, !>(res)
            })
            .unwrap()
    }
}