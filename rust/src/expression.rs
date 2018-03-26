use std::any::Any;
use std::fmt;

use money::Money;
use bank::Bank;

pub trait Expression {
  fn reduce(&self, bank: &Bank, to: &'static str) -> Money;
}

impl fmt::Debug for Expression {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Expression: {:?}", self)
  }
}

pub trait ExpressionalObject {
  fn equals(&self, &Any) -> bool;
}

impl<T: Any + PartialEq + Expression> ExpressionalObject for T {
  fn equals(&self, other: &Any) -> bool {
    match other.downcast_ref::<T>() {
      None => false,
      Some(a) => self == a,
    }
  }
}
