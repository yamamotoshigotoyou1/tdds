use money::MonetaryValue;

pub struct Franc {
    amount: u32,

}

impl Franc {
    pub fn new(amount: u32) -> Franc {
        Franc{amount: amount}
    }

    pub fn times(&self, multiplier: u32) -> Franc {
        Franc{amount: self.amount * multiplier}
    }
}

impl MonetaryValue for Franc {
    fn amount(&self) -> u32 {
        self.amount
    }
}
