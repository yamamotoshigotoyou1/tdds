extern crate money;


#[cfg(test)]
mod money_test {
    use money::money::Money;
    use money::money::MonetaryUnit;

    use money::dollar::Dollar;
    use money::franc::Franc;

    #[test]
    fn test_multiplication() {
        let five = Dollar::new(5);
        assert_eq!(Money::from(Dollar::new(10)), Money::from(five.times(2)));
        assert_eq!(Money::from(Dollar::new(15)), Money::from(five.times(3)));
    }

    #[test]
    fn test_equality() {
        assert!(Money::from(Dollar::new(5)) == Money::from(Dollar::new(5)));
        assert!(Money::from(Dollar::new(5)) != Money::from(Dollar::new(6)));
        assert!(Money::from(Franc::new(5)) == Money::from(Franc::new(5)));
        assert!(Money::from(Franc::new(5)) != Money::from(Franc::new(6)));
        assert!(!Franc::new(5).equals(&Dollar::new(5)));
        assert!(!Franc::new(5).equals(&Money::from(Dollar::new(5))));
    }

    #[test]
    fn test_franc_multiplication() {
        let five = Franc::new(5);
        assert_eq!(Money::from(Franc::new(10)), Money::from(five.times(2)));
        assert_eq!(Money::from(Franc::new(15)), Money::from(five.times(3)));
    }
}
