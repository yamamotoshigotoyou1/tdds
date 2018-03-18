use expression::Expression;
use money::Money;

pub struct Sum<'a> {
    pub augend: &'a Money,
    pub addend: &'a Money,
}

impl<'a> Sum<'a> {
    pub fn new(augend: &'a Money, addend: &'a Money) -> Self {
        Self{augend, addend}
    }
}

impl<'a> Expression for Sum<'a> {
}
