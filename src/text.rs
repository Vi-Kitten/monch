use super::*;
use super::primitives::*;

#[derive(Clone)]
pub struct ExpectString {
    expect: String,
}

impl ExpectString {
    pub fn new(expect: String) -> ExpectString {
        ExpectString {
            expect,
        }
    }
}

impl<I> Parser<I> for ExpectString where
    I: Iterator + Clone,
    I::Item: Into<char>
{
    type Value = String;
    type Error = String;

    fn parse(&self, iter: &mut I, info: &mut ParseInfo) -> Result<String, String> {
        let found = iter.take(self.expect.len()).map(Into::into).collect::<String>();
        *info += ParseInfo::new(found.len(), self.expect.len());
        if found == self.expect {
            Ok(found)
        } else {
            Err(self.expect.clone())
        }
    }
}

