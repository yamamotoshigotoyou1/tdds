extern crate money;


#[cfg(test)]
mod money_test {
    use money::dollar::Dollar;

    #[test]
    fn test_multiplication() {
        let mut five = Dollar{amount: 5};
        five.times(2);
        assert_eq!(10, five.amount);
    }
}
