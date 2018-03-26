use std::any::Any;

use money::Money;
use bank::Bank;

pub trait Expression {
  fn reduce(&self, bank: &Bank, to: &'static str) -> Money;
}

pub trait ExpressionalObject {
  fn as_any(&self) -> &Any;
  fn equals(&self, &ExpressionalObject) -> bool;
}

impl<T: Any + PartialEq + Expression> ExpressionalObject for T {
  fn as_any(&self) -> &Any {
    self as &Any
  }

  fn equals(&self, other: &ExpressionalObject) -> bool {
    match other.as_any().downcast_ref::<T>() {
      None => false,
      Some(a) => self == a,
    }
  }
}
