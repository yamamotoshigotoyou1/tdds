#![feature(core_intrinsics)]

extern crate money;

#[cfg(test)]
mod money_test {
  use std::intrinsics::type_name;

  use money::expression::Expression;
  use money::money::Money;
  use money::bank::Bank;
  use money::sum::Sum;

  fn get_type<T>(_: &T) -> &'static str {
    unsafe { type_name::<T>() }
  }

  #[test]
  fn test_multiplication() {
    let five = Money::dollar(5);
    assert_eq!(Money::dollar(10), five.times(2));
    assert_eq!(Money::dollar(15), five.times(3));
  }

  #[test]
  fn test_equality() {
    assert!(Money::dollar(5) == Money::dollar(5));
    assert!(Money::dollar(5) != Money::dollar(6));
    assert!(Money::franc(5) != Money::dollar(5));
  }

  #[test]
  fn test_currency() {
    assert_eq!("USD", Money::dollar(1).currency());
    assert_eq!("CHF", Money::franc(1).currency());
  }

  #[test]
  fn test_simple_addition() {
    let five = Money::dollar(5);
    let sum = five.plus(&five);

    let bank = Bank::new();
    let reduced = bank.reduce(&sum, "USD");
    assert_eq!(Money::dollar(10), reduced);
  }

  #[test]
  fn test_plus_returns_sum() {
    let five = Money::dollar(5);
    let sum = five.plus(&five);

    // TODO: Consider more efficient way :'(
    // Should `augend` and `addend` be really checked?
    assert_eq!("money::sum::Sum", get_type(&sum));
  }

  #[test]
  fn test_reduce_sum() {
    let three = Money::dollar(3);
    let four = Money::dollar(4);
    let sum = Sum::new(&three, &four);

    let bank = Bank::new();
    let reduced = bank.reduce(&sum, "USD");
    assert_eq!(Money::dollar(7), reduced);
  }

  #[test]
  fn test_reduce_money() {
    let bank = Bank::new();
    let result = bank.reduce(&Money::dollar(1), "USD");
    assert_eq!(Money::dollar(1), result);
  }

  #[test]
  fn test_reduce_money_different_currency() {
    // 1 CHF == 2 USD
    let mut bank = Bank::new();
    bank.add_rate("USD", "CHF", 2);
    let result = bank.reduce(&Money::dollar(2), "CHF");
    assert_eq!(Money::franc(1), result);
  }

  #[test]
  fn test_identity_rate() {
    assert_eq!(1, Bank::new().rate("CHF", "CHF"));
  }

  #[test]
  fn test_mixed_addition() {
    // 1 CHF == 2 USD
    let ten_bucks = Money::dollar(10);
    let five_francs = Money::franc(5);
    let mut bank = Bank::new();
    bank.add_rate("USD", "CHF", 2);
    let result = bank.reduce(&ten_bucks.plus(&five_francs), "CHF");
    assert_eq!(Money::franc(10), result);
  }

  #[test]
  fn test_sum_plus_money() {
    // 1 CHF == 2 USD
    let ten_bucks = Money::dollar(10);
    let five_francs = Money::franc(5);
    let mut bank = Bank::new();
    bank.add_rate("USD", "CHF", 2);

    let sum = Sum::new(&ten_bucks, &five_francs);
    let result = bank.reduce(&sum.plus(&five_francs), "CHF");
    assert_eq!(Money::franc(15), result);
  }

  //#[test]
  // fn test_sum_times() { // 1 CHF == 2 USD
  //   let ten_bucks = Money::dollar(10);
  //   let five_francs = Money::franc(5);
  //   let mut bank = Bank::new();
  //   bank.add_rate("USD", "CHF", 2);

  //   let sum = Sum::new(&ten_bucks, &five_francs);
  //   let result = bank.reduce(&sum.times(2), "CHF");
  //   assert_eq!(Money::franc(20), result);
  // }
}
