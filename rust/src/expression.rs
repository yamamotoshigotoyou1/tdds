use std::any::Any;
use std::fmt;

use money::Money;
use bank::Bank;
use sum::Sum;

pub trait Expression<'a> {
  fn plus(&'a self, addend: &'a (Expression<'a> + 'a)) -> Sum<'a>;
  fn reduce(&self, bank: &Bank, to: &'static str) -> Money;
}

impl<'a> fmt::Debug for Expression<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Expression: {:?}", self)
  }
}

pub trait ExpressionalObject {
  fn equals(&self, &Any) -> bool;
}

impl<'a, T: Any + PartialEq + Expression<'a>> ExpressionalObject for T {
  fn equals(&self, other: &Any) -> bool {
    match other.downcast_ref::<T>() {
      None => false,
      Some(a) => self == a,
    }
  }
}
