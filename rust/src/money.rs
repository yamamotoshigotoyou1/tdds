use std::any::Any;
use dollar::Dollar;
use franc::Franc;

#[derive(Debug)]
pub struct Money {
    amount: u32,
}

impl Money {
    fn new(amount: u32) -> Self {
        // pass
        Money{amount: amount}
    }

    fn amount(&self) -> u32 {
        self.amount
    }

    pub fn dollar(amount: u32) -> Dollar {
        Dollar::new(amount)
    }

    pub fn franc(amount: u32) -> Franc {
        Franc::new(amount)
    }
}

impl PartialEq for Money {
    fn eq(&self, other: &Money) -> bool {
        self.amount() == other.amount()
    }
}

pub trait MonetaryValue {
    fn new(amount: u32) -> Self;

    fn amount(&self) -> u32;

    fn times(&self, multiplier: u32) -> Money {
        Money::new(self.amount() * multiplier)
    }
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
