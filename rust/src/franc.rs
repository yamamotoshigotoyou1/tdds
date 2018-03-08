use money::{Money,MonetaryValue};


pub struct Franc {
    amount: u32,
    pub currency: &'static str,
}

impl Franc {
    pub fn new(amount: u32) -> Franc {
        Franc{amount: amount, currency: "CHF"}
    }

    pub fn times(&self, multiplier: u32) -> Money {
        Money::from(Self::new(self.amount * multiplier))
    }
}

impl MonetaryValue for Franc {
    fn amount(&self) -> u32 {
        self.amount
    }
}

impl PartialEq for Franc {
    fn eq(&self, other: &Franc) -> bool {
        self.amount() == other.amount()
    }
}
