use std::fmt;

use money::Money;
use bank::Bank;
use sum::Sum;

pub trait Expression {
  fn plus<'a>(&'a self, addend: &'a (Expression + 'a)) -> Sum<'a>;
  fn reduce(&self, bank: &Bank, to: &'static str) -> Money;
}

impl fmt::Debug for Expression {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Expression: {:?}", self)
  }
}
