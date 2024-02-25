use dyn_clone::DynClone;


pub trait StatefulIterator: Iterator {
    type State;

    fn get_state(&mut self) -> &mut Self::State;
}

pub trait DynCloneStatefulIterator: StatefulIterator + DynClone {}

impl<I> DynCloneStatefulIterator for I where
    I: StatefulIterator + DynClone {}

#[derive(Clone)]
pub struct WithState<I, S> {
    iter: I,
    state: S,
}

impl<I, S> WithState<I, S> {
    pub fn new(iter: I, state: S) -> WithState<I, S> {
        WithState {
            iter,
            state,
        }
    }
}

impl<I, S> Iterator for WithState<I, S> where
    I: Iterator
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<I, S> StatefulIterator for WithState<I, S> where
    I: Iterator + Clone
{
    type State = S;

    fn get_state(&mut self) -> &mut S {
        &mut self.state
    }
}

#[derive(Clone)]
pub struct SubState<I, F> {
    iter: I,
    lense: F,
}

impl<I, F> SubState<I, F> {
    pub fn new(iter: I, lense: F) -> SubState<I, F> {
        SubState {
            iter,
            lense,
        }
    }

    pub fn inner(self) -> I {
        self.iter
    }
}

impl<I, F> Iterator for SubState<I, F> where
    I: Iterator
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        self.iter.next()
    }
}

impl<I, F, S> StatefulIterator for SubState<I, F> where
    I: StatefulIterator,
    F: for<'s> Fn(&'s mut I::State) -> &'s mut S
{
    type State = S;

    fn get_state(&mut self) -> &mut S {
        (self.lense)(self.iter.get_state())
    }
}

// #[derive(Clone)]
// pub struct PrepDyn<I> {
//     iter: I,
// }

// impl<I> Iterator for PrepDyn<I> where
//     I: Iterator
// {
//     type Item = I::Item;

//     fn next(&mut self) -> Option<I::Item> {
//         self.iter.next()
//     }
// }