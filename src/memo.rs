
use super::*;
use std::{cell::RefCell, collections::*};

pub trait MemoHandler<I> where
    I: Iterator + Clone,
{
    type Value: Clone;
    type Error: Clone;

    fn learn(&self, iter: I, res: ParseResult<Self::Value, Self::Error>);

    fn recall(&self, iter: I) -> Option<ParseResult<Self::Value, Self::Error>>;
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
    C: FromIterator<I::Item>,
    for<'a> &'a C: IntoIterator<Item=&'a I::Item>,
    I::Item: Eq,
    T: Clone,
    E: Clone
{
    type Value = T;
    type Error = E;

    fn learn(&self, iter: I, p_res: ParseResult<T, E>) {
        *self.pair.borrow_mut() = Some((
            iter.take(p_res.info.read).collect::<C>(),
            p_res
        ))
    }

    fn recall(&self, mut iter: I) -> Option<ParseResult<T, E>> {
        self.pair.borrow()
            .as_ref()
            .and_then(|(collection, p_res)| {
                for col_token in collection {
                    iter.next()
                        .filter(|token| token == col_token)?;
                };
                Some(p_res.clone())
            })
    }
}

pub struct MemoMap<F, K, H> {
    handler_map: std::cell::RefCell<HashMap<K, H>>,
    key_func: F,
}

impl<F, K, H> MemoMap<F, K, H> {
    pub fn new(key_func: F) -> MemoMap<F, K, H> {
        MemoMap {
            handler_map: RefCell::default(),
            key_func,
        }
    }
}

impl<I, F, K, H> MemoHandler<I> for MemoMap<F, K, H> where
    I: Iterator + Clone,
    F: Fn(&I) -> K,
    K: Eq + std::hash::Hash,
    H: MemoHandler<I> + Default
{
    type Value = H::Value;
    type Error = H::Error;

    fn learn(&self, iter: I, res: ParseResult<Self::Value, Self::Error>) {
        let key = (self.key_func)(&iter);
        if let Some(handler) = self.handler_map.borrow().get(&key) {
            handler.learn(iter, res);
        } else {
            let handler = H::default();
            handler.learn(iter, res);
            self.handler_map.borrow_mut().insert(key, handler).ok_or(()).err().unwrap();
        }
    }

    fn recall(&self, iter: I) -> Option<ParseResult<Self::Value, Self::Error>> {
        let key = (self.key_func)(&iter);
        self.handler_map.borrow()
            .get(&key)
            .and_then(|handler| {
                handler.recall(iter)
            })
    }
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
    H: MemoHandler<I, Value=T, Error=E>,
    T: Clone,
    E: Clone
{
    type Value = T;
    type Error = E;

    fn parse(&self, iter: &mut I) -> ParseResult<T, E> {
        if let Some(p_res) = self.handler.recall(iter.clone()) {
            return p_res;
        }
        let start_iter = iter.clone();
        let p_res = self.parser.parse(iter);
        self.handler.learn(start_iter, p_res.clone());
        p_res
    }
}