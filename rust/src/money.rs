use expression::Expression;
use sum::Sum;
use bank::Bank;

#[derive(Debug)]
pub struct Money {
  amount: u32,
  currency: &'static str,
}

impl Money {
  pub fn new(amount: u32, currency: &'static str) -> Self {
    Self {
      amount,
      currency,
    }
  }

  pub fn amount(&self) -> u32 {
    self.amount
  }

  pub fn currency(&self) -> &'static str {
    self.currency
  }

  pub fn dollar(amount: u32) -> Self {
    Self {
      amount,
      currency: "USD",
    }
  }

  pub fn franc(amount: u32) -> Self {
    Self {
      amount,
      currency: "CHF",
    }
  }

  // TODO: Move to Expression
  pub fn times(&self, multiplier: u32) -> Self {
    Self {
      amount: self.amount * multiplier,
      currency: self.currency,
    }
  }
}

impl PartialEq for Money {
  fn eq(&self, other: &Money) -> bool {
    self.amount == other.amount && self.currency == other.currency
  }
}

impl Expression for Money {
  fn plus<'a>(&'a self, addend: &'a (Expression + 'a)) -> Sum<'a> {
    Sum::new(self, addend)
  }

  fn reduce(&self, bank: &Bank, to: &'static str) -> Self {
    let rate = bank.rate(self.currency, to);
    Self {
      amount: self.amount / rate,
      currency: to,
    }
  }
}
