use money::MonetaryValue;


pub struct Dollar {
    amount: u32,
    pub currency: &'static str,
}

impl Dollar {}

impl MonetaryValue for Dollar {
    fn new(amount: u32) -> Self {
        Dollar{amount: amount, currency: "USD"}
    }

    fn amount(&self) -> u32 {
        self.amount
    }
}

impl PartialEq for Dollar {
    fn eq(&self, other: &Dollar) -> bool {
        self.amount() == other.amount()
    }
}
