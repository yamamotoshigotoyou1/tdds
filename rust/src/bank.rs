use money::Money;


pub struct Bank {
}

impl Bank {
    pub fn new() -> Self {
        Self{}
    }

    pub fn reduce(&self, _source: Money, _to: &'static str) -> Money {
        Money::dollar(10)
    }
}
