use money::MonetaryValue;


pub struct Dollar {
    amount: u32,
    currency: &'static str,
}

impl Dollar {}

impl MonetaryValue for Dollar {
    fn new(amount: u32, currency: &'static str) -> Self {
        Dollar{amount, currency}
    }

    fn amount(&self) -> u32 {
        self.amount
    }

    fn currency(&self) -> &'static str {
        self.currency
    }
}

impl PartialEq for Dollar {
    fn eq(&self, other: &Dollar) -> bool {
        self.amount() == other.amount()
    }
}
