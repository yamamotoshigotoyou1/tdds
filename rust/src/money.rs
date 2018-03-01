use std::any::Any;

#[derive(Debug)]
pub struct Money {
    amount: u32,
}

impl Money {
    fn amount(&self) -> u32 {
        self.amount
    }
}

impl PartialEq for Money {
    fn eq(&self, other: &Money) -> bool {
        self.amount() == other.amount()
    }
}

pub trait MonetaryValue {
    fn amount(&self) -> u32;
}

impl<T: MonetaryValue> From<T> for Money {
    fn from(a: T) -> Money {
        Money{amount: a.amount()}
    }
}

pub trait MonetaryUnit {
    fn as_any(&self) -> &Any;
    fn equals(&self, &MonetaryUnit) -> bool;
}

impl<T: Any + PartialEq> MonetaryUnit for T {
    fn as_any(&self) -> &Any {
        self as &Any
    }

    fn equals(&self, other: &MonetaryUnit) -> bool {
        match other.as_any().downcast_ref::<T>() {
            None => false,
            Some(a) => self == a,
        }
    }
}
