
use super::*;
use std::collections::*;

pub trait MemoHandler<I> where
    I: Iterator + Clone,
{
    type Value;
    type Error;
    type Memory: Parser<I, Value=Self::Value, Error=Self::Error>;

    fn learn(&self, iter: I, info: ParseInfo, res: Result<Self::Value, Self::Error>);

    fn recall(&self) -> Option<Self::Memory>;
}

#[derive(Default)]
pub struct MemoSingular<C, T, E> {
    pair: std::cell::RefCell<Option<(C, ParseInfo, Result<T, E>)>>
}

impl<C, T, E> MemoSingular<C, T, E> {
    pub fn new(collection: C, p_res: Result<T, E>) -> MemoSingular<C, T, E> {
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

    fn learn(&self, iter: I, info: ParseInfo, res: Result<T, E>) {
        *self.pair.borrow_mut() = Some((
            iter.take(info.read).collect::<C>(),
            info,
            res
        ))
    }

    fn recall(&self, iter: &mut I, ) -> Option<Result<T, E>> {
        self.pair.borrow()
            .as_ref()
            .and_then(|(collection, inner_info, res)| {
                for col_token in collection {
                    iter.next()
                        .filter(|token| token == col_token)?;
                };
                Some(res.clone())
            })
    }
}

#[derive(Default)]
pub struct SyncMemoSingular<C, T, E> {
    pair: std::sync::RwLock<Option<(C, Result<T, E>)>>
}

impl<C, T, E> SyncMemoSingular<C, T, E> {
    pub fn new(collection: C, p_res: Result<T, E>) -> SyncMemoSingular<C, T, E> {
        SyncMemoSingular {
            pair: std::sync::RwLock::new(Some((collection, p_res)))
        }
    }
}

impl<I, C, T, E> MemoHandler<I> for SyncMemoSingular<C, T, E> where
    I: Iterator + Clone,
    C: FromIterator<I::Item>,
    for<'a> &'a C: IntoIterator<Item=&'a I::Item>,
    I::Item: Eq,
    T: Clone,
    E: Clone
{
    type Value = T;
    type Error = E;

    fn learn(&self, iter: I, p_res: Result<T, E>) {
        *self.pair.write().unwrap() = Some((
            iter.take(p_res.info.read).collect::<C>(),
            p_res
        ))
    }

    fn recall(&self, mut iter: I) -> Option<Result<T, E>> {
        self.pair.read().unwrap()
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
            handler_map: std::cell::RefCell::default(),
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

    fn learn(&self, iter: I, res: Result<Self::Value, Self::Error>) {
        let key = (self.key_func)(&iter);
        if let Some(handler) = self.handler_map.borrow().get(&key) {
            handler.learn(iter, res);
        } else {
            let handler = H::default();
            handler.learn(iter, res);
            self.handler_map.borrow_mut()
                .insert(key, handler)
                .ok_or(()).err().unwrap();
        }
    }

    fn recall(&self, iter: I) -> Option<Result<Self::Value, Self::Error>> {
        let key = (self.key_func)(&iter);
        self.handler_map.borrow()
            .get(&key)
            .and_then(|handler| {
                handler.recall(iter)
            })
    }
}

pub struct SyncMemoMap<F, K, H> {
    handler_map: std::sync::RwLock<HashMap<K, H>>,
    key_func: F,
}

impl<F, K, H> SyncMemoMap<F, K, H> {
    pub fn new(key_func: F) -> SyncMemoMap<F, K, H> {
        SyncMemoMap {
            handler_map: std::sync::RwLock::default(),
            key_func,
        }
    }
}

impl<I, F, K, H> MemoHandler<I> for SyncMemoMap<F, K, H> where
    I: Iterator + Clone,
    F: Fn(&I) -> K,
    K: Eq + std::hash::Hash,
    H: MemoHandler<I> + Default
{
    type Value = H::Value;
    type Error = H::Error;

    fn learn(&self, iter: I, res: Result<Self::Value, Self::Error>) {
        let key = (self.key_func)(&iter);
        if let Some(handler) = self.handler_map.read().unwrap().get(&key) {
            handler.learn(iter, res);
        } else {
            let handler = H::default();
            handler.learn(iter, res);
            self.handler_map.write().unwrap()
                .insert(key, handler)
                .ok_or(()).err().unwrap();
        }
    }

    fn recall(&self, iter: I) -> Option<Result<Self::Value, Self::Error>> {
        let key = (self.key_func)(&iter);
        self.handler_map.read().unwrap()
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

    fn parse(&self, iter: &mut I, info: &mut ParseInfo) -> Result<T, E> {
        if let Some(res) = self.handler.recall(iter, info) {
            return res;
        }
        let mut inner_info = ParseInfo::default();
        let start_iter = iter.clone();
        let res = self.parser.parse(iter, &mut inner_info);
        self.handler.learn(start_iter, inner_info, res.clone());
        res
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
    H: MemoHandler<I, Value=T, Error=E>,
    F: Fn(ParseInfo, &Result<T, E>) -> bool,
    T: Clone,
    E: Clone
{
    type Value = T;
    type Error = E;

    fn parse(&self, iter: &mut I, info: &mut ParseInfo) -> Result<T, E> {
        if let Some(res) = self.handler.recall(iter, info) {
            return res;
        }
        let mut inner_info = ParseInfo::default();
        let start_iter = iter.clone();
        let res = self.parser.parse(iter, &mut inner_info);
        if (self.predicate)(inner_info, &res) {
            self.handler.learn(start_iter, inner_info, res.clone());
        }
        res
    }
}