use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug)]
struct WasRun {
  was_run: i32,
  name: &'static str,
}

#[allow(dead_code)]
impl WasRun {
  pub fn new(name: &'static str) -> Self {
    WasRun { was_run: 0, name }
  }

  pub fn test_method(&mut self) {
    self.was_run = 1;
  }

  pub fn run(&mut self) {
    // TODO: alternative way for dynamic method invocation
    let mut methods: HashMap<&'static str, fn(&mut Self)> = HashMap::new();
    methods.insert("test_method", WasRun::test_method);

    match methods.get(self.name) {
      Some(f) => f(self),
      None => panic!("Not found"),
    };
  }
}

#[cfg(test)]
mod was_run_test {
  use super::*;

  #[test]
  fn test_was_run() {
    let mut test = WasRun::new("test_method");

    println!("{}", test.was_run);
    test.run();
    println!("{}", test.was_run);
  }
}
