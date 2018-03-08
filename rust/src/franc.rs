use money::MonetaryValue;


pub struct Franc {
    amount: u32,
    currency: &'static str,
}

impl Franc {}

impl MonetaryValue for Franc {
    fn new(amount: u32, currency: &'static str) -> Self {
        Franc{amount, currency}
    }

    fn amount(&self) -> u32 {
        self.amount
    }

    fn currency(&self) -> &'static str {
        self.currency
    }
}

impl PartialEq for Franc {
    fn eq(&self, other: &Franc) -> bool {
        self.amount() == other.amount()
    }
}
