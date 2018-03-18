use money::Money;

pub trait Expression {
    fn reduce(&self, to: &'static str) -> Money;
}
