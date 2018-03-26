use expression::Expression;
use money::Money;
use bank::Bank;

pub struct Sum<'a> {
  pub augend: &'a (Expression<'a> + 'a),
  pub addend: &'a (Expression<'a> + 'a),
}

impl<'a> Sum<'a> {
  pub fn new(
    augend: &'a (Expression<'a> + 'a),
    addend: &'a (Expression<'a> + 'a),
  ) -> Sum<'a>
  {
    Self { augend, addend }
  }
}

impl<'a> Expression<'a> for Sum<'a> {
  fn plus(&'a self, addend: &'a (Expression<'a> + 'a)) -> Sum<'a> {
    Sum::new(self, addend)
  }

  fn reduce(&self, bank: &Bank, to: &'static str) -> Money {
    let amount = self.augend.reduce(bank, to).amount() +
      self.addend.reduce(bank, to).amount();
    Money::new(amount, to)
  }
}
