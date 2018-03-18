use std::any::Any;

use expression::Expression;
use sum::Sum;

#[derive(Debug)]
pub struct Money {
    amount: u32,
    currency: &'static str,
}

impl Money {
    pub fn new(amount: u32, currency: &'static str) -> Self {
        Self{amount: amount, currency}
    }

    fn amount(&self) -> u32 {
        self.amount
    }

    pub fn currency(&self) -> &'static str {
        self.currency
    }

    pub fn times(&self, multiplier: u32) -> Self {
        Self::new(self.amount() * multiplier, self.currency())
    }

    pub fn plus<'a>(&'a self, addend: &'a Self) -> Sum {
        let s = Sum::new(self, addend);
        s
    }

    pub fn dollar(amount: u32) -> Self {
        Self::new(amount, "USD")
    }

    pub fn franc(amount: u32) -> Self {
        Self::new(amount, "CHF")
    }
}

impl PartialEq for Money {
    fn eq(&self, other: &Money) -> bool {
        self.amount() == other.amount() && self.currency() == other.currency()
    }
}

pub trait MonetaryObject {
    fn as_any(&self) -> &Any;
    fn equals(&self, &MonetaryObject) -> bool;
}

impl<T: Any + PartialEq> MonetaryObject for T {
    fn as_any(&self) -> &Any {
        self as &Any
    }

    fn equals(&self, other: &MonetaryObject) -> bool {
        match other.as_any().downcast_ref::<T>() {
            None => false,
            Some(a) => self == a
        }
    }
}

impl Expression for Money {
}
