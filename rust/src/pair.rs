use std::hash::{Hash, Hasher};

pub struct Pair {
  from: &'static str,
  to: &'static str,
}

impl Pair {
  pub fn new(from: &'static str, to: &'static str) -> Self {
    Self { from, to }
  }
}

impl Hash for Pair {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.from.hash(state);
    self.to.hash(state);
  }
}

impl PartialEq for Pair {
  fn eq(&self, other: &Pair) -> bool {
    self.from == other.from && self.to == other.to
  }
}

impl Eq for Pair {}
