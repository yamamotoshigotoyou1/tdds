use money::Money;
use expression::Expression;


pub struct Bank {
}

impl Bank {
    pub fn new() -> Self {
        Self{}
    }

    pub fn reduce<'a>(&self,
        source: &'a (Expression + 'a), to: &'static str) -> Money {
        source.reduce(to)
    }
}
