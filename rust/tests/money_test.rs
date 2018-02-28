extern crate money;


#[cfg(test)]
mod money_test {
    use money::dollar::Dollar;

    #[test]
    fn test_multiplication() {
        let five = Dollar::new(5);
        // depends impl PartialEq
        assert_eq!(Dollar::new(10), five.times(2));
        assert_eq!(Dollar::new(15), five.times(3));
    }

    #[test]
    fn test_equality() {
        assert!(Dollar::new(5) == Dollar::new(5));
        assert!(Dollar::new(5) != Dollar::new(6));
    }
}
