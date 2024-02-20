use super::*;

pub struct LazyParser<P, F> where
    F: FnOnce() -> P
{
    parser: std::cell::RefCell<Result<P, Option<F>>> // option will always be Some
}

impl<P, F, I, T, E> UnsizedParser<I, T, E> for LazyParser<P, F> where
    F: FnOnce() -> P,
    I: Iterator + Clone,
    P: UnsizedParser<I, T, E>
{
    fn parse(&self, iter: &mut I) -> Result<T, E> {
        if let Ok(p) = &*self.parser.borrow() {
            return p.parse(iter)
        }

        {
            let parser = &mut *self.parser.borrow_mut();
            match parser {
                Ok(_) => panic!(),
                Err(f) => {
                    let mut g = None;
                    std::mem::swap(f, &mut g);
                    let p = g.unwrap()();
                    *parser = Ok(p);
                }
            }
        }

        let Ok(p) = &*self.parser.borrow() else {
            panic!()
        };

        p.parse(iter)
    }
}

impl<P, F> LazyParser<P, F> where
    F: FnOnce() -> P
{
    pub fn new(f: F) -> LazyParser<P, F> {
        LazyParser {
            parser: std::cell::RefCell::new(Err(Some(f)))
        }
    }
}