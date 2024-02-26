use super::*;

pub trait MemoHandler<I> where
    I: Iterator + Clone,
{
    type Value: Clone;

    fn remember(&self, iter: &mut I, res: Self::Value, info: ParseInfo);

    fn recall(&self, iter: &mut I) -> Option<Self::Value>;
}