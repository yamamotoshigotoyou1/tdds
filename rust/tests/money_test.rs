extern crate money;


#[cfg(test)]
mod money_test {
    use money::dollar::Dollar;

    #[test]
    fn test_multiplication() {
        let five = Dollar{amount: 5};
        let mut product = five.times(2);
        assert_eq!(10, product.amount);
        product = five.times(3);
        assert_eq!(15, product.amount);
    }

    #[test]
    fn test_equality() {
        assert!(Dollar{amount: 5} == Dollar{amount: 5});
        assert!(Dollar{amount: 5} != Dollar{amount: 6});
    }
}
