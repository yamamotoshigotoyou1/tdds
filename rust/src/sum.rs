use expression::Expression;
use money::Money;
use bank::Bank;

pub struct Sum<'a> {
  pub augend: &'a Money,
  pub addend: &'a Money,
}

impl<'a> Sum<'a> {
  pub fn new(augend: &'a Money, addend: &'a Money) -> Self {
    Self { augend, addend }
  }
}

impl<'a> Expression for Sum<'a> {
  fn reduce(&self, bank: &Bank, to: &'static str) -> Money {
    let amount = self.augend.reduce(bank, to).amount() +
      self.addend.reduce(bank, to).amount();
    Money::new(amount, to)
  }
}
