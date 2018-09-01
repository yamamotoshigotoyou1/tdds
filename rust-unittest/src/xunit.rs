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

#[cfg(test)]
mod was_run_test {
  use super::*;

  #[test]
  fn test_was_run() {
    let mut test: WasRun = WasRun {
      was_run: 0,
      name: "test_method",
    };

    println!("{}", test.was_run);
    test.test_method();
    println!("{}", test.was_run);
  }
}
