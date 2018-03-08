use std::any::Any;
use dollar::Dollar;
use franc::Franc;

#[derive(Debug)]
pub struct Money {
    amount: u32,
    currency: &'static str,
}

impl Money {
    fn new(amount: u32, currency: &'static str) -> Self {
        // pass
        Money{amount: amount, currency}
    }

    fn amount(&self) -> u32 {
        self.amount
    }

    fn currency(&self) -> &'static str {
        self.currency
    }

    pub fn dollar(amount: u32) -> Dollar {
        Dollar::new(amount, "USD")
    }

    pub fn franc(amount: u32) -> Franc {
        Franc::new(amount, "CHF")
    }
}

impl PartialEq for Money {
    fn eq(&self, other: &Money) -> bool {
        self.amount() == other.amount()
    }
}

pub trait MonetaryValue {
    fn new(amount: u32, currency: &'static str) -> Self;

    fn amount(&self) -> u32;
    fn currency(&self) -> &'static str;

    fn times(&self, multiplier: u32) -> Money {
        Money::new(self.amount() * multiplier, self.currency())
    }
}

impl<T: MonetaryValue> From<T> for Money {
    fn from(a: T) -> Money {
        Money{amount: a.amount(), currency: a.currency()}
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
