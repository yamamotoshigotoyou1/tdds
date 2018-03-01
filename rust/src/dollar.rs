use money::MonetaryValue;


pub struct Dollar {
    amount: u32,

}

impl Dollar {
    pub fn new(amount: u32) -> Dollar {
        Dollar{amount: amount}
    }

    pub fn times(&self, multiplier: u32) -> Dollar {
        Dollar{amount: self.amount * multiplier}
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
