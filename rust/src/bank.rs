use money::Money;
use sum::Sum;


pub struct Bank {
}

impl Bank {
    pub fn new() -> Self {
        Self{}
    }

    pub fn reduce(&self, source: Sum, to: &'static str) -> Money {
        let amount = source.augend.amount() + source.addend.amount();
        Money::new(amount, to)
    }
}
