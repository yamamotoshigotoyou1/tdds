use money::Money;
use bank::Bank;

pub trait Expression {
  fn reduce(&self, bank: &Bank, to: &'static str) -> Money;
}
