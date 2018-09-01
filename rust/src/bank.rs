use std::collections::HashMap;

use money::Money;
use expression::Expression;
use pair::Pair;

pub struct Bank {
  rates: HashMap<Pair, u32>,
}

impl Bank {
  pub fn new() -> Self {
    Self {
      rates: HashMap::new(),
    }
  }

  pub fn reduce<'a>(
    &self,
    source: &'a (Expression + 'a),
    to: &'static str,
  ) -> Money
  {
    source.reduce(&self, to)
  }

  pub fn rate(&self, from: &'static str, to: &'static str) -> u32 {
    if from == to {
      return 1;
    }
    match self.rates.get(&Pair::new(from, to)) {
      Some(rate) => *rate,
      None => panic!("Unknown Pair is given, from: {}, to: {}", from, to),
    }
  }

  pub fn add_rate(&mut self, from: &'static str, to: &'static str, rate: u32) {
    self.rates.insert(Pair::new(from, to), rate);
  }
}
