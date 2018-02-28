#[derive(Debug)]
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

impl PartialEq for Franc {
    fn eq(&self, other: &Franc) -> bool {
        self.amount == other.amount
    }
}
