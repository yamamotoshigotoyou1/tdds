extern crate money;


#[cfg(test)]
mod money_test {
    use money::money::Money;
    use money::money::MonetaryValue;
    use money::money::MonetaryUnit;

    #[test]
    fn test_multiplication() {
        let five = Money::dollar(5);
        assert_eq!(Money::from(Money::dollar(10)), five.times(2));
        assert_eq!(Money::from(Money::dollar(15)), five.times(3));
    }

    #[test]
    fn test_equality() {
        assert!(Money::dollar(5) == Money::dollar(5));
        assert!(Money::dollar(5) != Money::dollar(6));
        assert!(Money::franc(5) == Money::franc(5));
        assert!(Money::franc(5) != Money::franc(6));
        assert!(!Money::franc(5).equals(&Money::dollar(5)));
    }

    #[test]
    fn test_franc_multiplication() {
        let five = Money::franc(5);
        assert_eq!(Money::from(Money::franc(10)), five.times(2));
        assert_eq!(Money::from(Money::franc(15)), five.times(3));
    }

    #[test]
    fn test_currency() {
        assert_eq!("USD",  Money::dollar(1).currency());
        assert_eq!("CHF",  Money::franc(1).currency());
    }
}
