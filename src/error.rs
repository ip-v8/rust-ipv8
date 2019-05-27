macro_rules! create_error {
  ( $name: ident, $message: expr) => {
    #[derive(Debug)]
    pub struct $name;

    impl fmt::Display for $name {
      fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, $message)
      }
    }

    impl Error for $name{
      fn description(&self) -> &str {
        $message
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fmt;
  use std::error::Error;

  #[test]
  fn test_errors() {
    create_error!(TestError, "yeet");
    assert_eq!(TestError.description(), "yeet");
    assert_eq!(format!("{:?}",TestError), "TestError");
    assert_eq!(format!("{}", TestError), "yeet");
  }
}
