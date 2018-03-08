use money::{Money,MonetaryValue};


pub struct Dollar {
    amount: u32,
    pub currency: &'static str,
}

impl Dollar {
    pub fn new(amount: u32) -> Dollar {
        Dollar{amount: amount, currency: "USD"}
    }

    pub fn times(&self, multiplier: u32) -> Money {
        Money::from(Self::new(self.amount * multiplier))
    }
}

impl MonetaryValue for Dollar {
    fn amount(&self) -> u32 {
        self.amount
    }
}

impl PartialEq for Dollar {
    fn eq(&self, other: &Dollar) -> bool {
        self.amount() == other.amount()
    }
}
