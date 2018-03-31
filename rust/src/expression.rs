use money::Money;
use bank::Bank;
use mul::Mul;
use sum::Sum;

pub trait Expression {
  fn plus<'a>(&'a self, addend: &'a (Expression + 'a)) -> Sum<'a>;
  fn times<'a>(&'a self, multiplier: u32) -> Mul<'a>;
  fn reduce(&self, bank: &Bank, to: &'static str) -> Money;
}
