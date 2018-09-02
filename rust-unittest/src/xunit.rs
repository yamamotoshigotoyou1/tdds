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
    self.test_method();
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
