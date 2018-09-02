use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug)]
struct WasRun {
  was_run: i32,
  name: &'static str,
}

#[allow(dead_code)]
impl WasRun {
  pub fn test_method(&mut self) {
    self.was_run = 1;
  }
}

trait TestCase {
  fn new(name: &'static str) -> Self;
  fn run(&mut self);
}

impl TestCase for WasRun {
  fn new(name: &'static str) -> Self {
    WasRun { was_run: 0, name }
  }

  fn run(&mut self) {
    // TODO: alternative way for dynamic method invocation
    let mut methods: HashMap<&'static str, fn(&mut Self)> = HashMap::new();
    methods.insert("test_method", WasRun::test_method);

    match methods.get(self.name) {
      Some(f) => f(self),
      None => panic!("Not found"),
    };
  }
}

struct TestCaseTest {
  test: WasRun,
}

impl TestCaseTest {
  pub fn test_running(&mut self) {
    assert_eq!(0, self.test.was_run);
    self.test.run();
    assert_eq!(1, self.test.was_run);
  }
}

impl TestCase for TestCaseTest {
  fn new(name: &'static str) -> Self {
    let test = WasRun { was_run: 0, name };
    TestCaseTest { test }
  }

  fn run(&mut self) {
    self.test_running();
  }
}

#[cfg(test)]
mod was_run_test {
  use super::*;

  #[test]
  fn test_was_run() {
    let mut test = WasRun::new("test_method");

    assert_eq!(0, test.was_run);
    test.run();
    assert_eq!(1, test.was_run);
  }

  #[test]
  fn test_case_running() {
    let mut test_case_test = TestCaseTest::new("test_method");
    test_case_test.run();
  }
}
