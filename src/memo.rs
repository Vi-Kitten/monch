use super::*;

pub trait MemoHandler<I> where
    I: Iterator + Clone,
{
    type Value: Clone;
    type Error: Clone;

    fn learn(&self, iter: &mut I, res: ParseResult<Self::Value, Self::Error>);

    fn recall(&self, iter: &mut I) -> Option<ParseResult<Self::Value, Self::Error>>;
}

#[derive(Default)]
pub struct MemoSingular<C, T, E> {
    pair: std::cell::RefCell<Option<(C, ParseResult<T, E>)>>
}

impl<C, T, E> MemoSingular<C, T, E> {
    pub fn new(collection: C, p_res: ParseResult<T, E>) -> MemoSingular<C, T, E> {
        MemoSingular {
            pair: std::cell::RefCell::new(Some((collection, p_res)))
        }
    }
}

impl<I, C, T, E> MemoHandler<I> for MemoSingular<C, T, E> where
    I: Iterator + Clone,
    C: FromIterator<I::Item> + Eq,
    T: Clone,
    E: Clone
{
    type Value = T;
    type Error = E;

    fn learn(&self, iter: &mut I, p_res: ParseResult<T, E>) {
        *self.pair.borrow_mut() = Some((
            iter.take(p_res.info.read).collect::<C>(),
            p_res
        ))
    }

    fn recall(&self, iter: &mut I) -> Option<ParseResult<T, E>> {
        self.pair.borrow()
            .as_ref()
            .and_then(|(collection, p_res)| {
                Some(p_res)
                .filter(|_|
                    iter.take(p_res.info.read).collect::<C>() == *collection
                )
                .map(Clone::clone)
            })
    }
}

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
    H: MemoHandler<I, Value=T, Error=E>,
    T: Clone,
    E: Clone
{
    type Value = T;
    type Error = E;

    fn parse(&self, iter: &mut I) -> ParseResult<T, E> {
        if let Some(p_res) = self.handler.recall(&mut iter.clone()) {
            return p_res;
        }
        let mut start_iter = iter.clone();
        let p_res = self.parser.parse(iter);
        self.handler.learn(&mut start_iter, p_res.clone());
        p_res
    }
}