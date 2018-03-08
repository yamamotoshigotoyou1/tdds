use money::MonetaryValue;


pub struct Franc {
    amount: u32,
    pub currency: &'static str,
}

impl Franc {}

impl MonetaryValue for Franc {
    fn new(amount: u32) -> Self {
        Franc{amount: amount, currency: "CHF"}
    }

    fn amount(&self) -> u32 {
        self.amount
    }
}

impl PartialEq for Franc {
    fn eq(&self, other: &Franc) -> bool {
        self.amount() == other.amount()
    }
}
