#[derive(Debug)]
pub struct Money {
    amount: u32,
}

pub trait MonetaryValue {
    fn amount(&self) -> u32;
}

impl PartialEq for Money {
    fn eq(&self, other: &Money) -> bool {
        self.amount == other.amount
    }
}

impl<T: MonetaryValue> From<T> for Money {
    fn from(a: T) -> Money {
        Money{amount: a.amount()}
    }
}
