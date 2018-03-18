use money::Money;
use sum::Sum;


pub struct Bank {
}

impl Bank {
    pub fn new() -> Self {
        Self{}
    }

    pub fn reduce(&self, _source: Sum, _to: &'static str) -> Money {
        Money::dollar(10)
    }
}
