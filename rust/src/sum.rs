use expression::Expression;
use money::Money;
use bank::Bank;

pub struct Sum<'a> {
  pub augend: &'a (Expression + 'a),
  pub addend: &'a (Expression + 'a),
}

impl<'a> Sum<'a> {
  pub fn new(
    augend: &'a (Expression + 'a),
    addend: &'a (Expression + 'a),
  ) -> Sum<'a>
  {
    Self { augend, addend }
  }
}

impl<'a> Expression for Sum<'a> {
  fn plus<'b>(&'b self, addend: &'b (Expression + 'b)) -> Sum<'b> {
    Sum::new(self, addend.clone())
  }

  fn reduce(&self, bank: &Bank, to: &'static str) -> Money {
    let amount = self.augend.reduce(bank, to).amount() +
      self.addend.reduce(bank, to).amount();
    Money::new(amount, to)
  }
}
