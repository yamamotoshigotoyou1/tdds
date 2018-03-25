use money::Money;
use expression::Expression;

pub struct Bank {}

impl Bank {
  pub fn new() -> Self {
    Self {}
  }

  pub fn reduce<'a>(
    &self,
    source: &'a (Expression + 'a),
    to: &'static str,
  ) -> Money
  {
    source.reduce(&self, to)
  }

  pub fn rate(&self, from: &'static str, to: &'static str) -> u32 {
    match from == "USD" && to == "CHF" {
      true => 2,
      false => 1,
    }
  }

  pub fn add_rate(&self, _from: &'static str, _to: &'static str, _rate: u32) {}
}
