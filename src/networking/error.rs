use std::fmt;

macro_rules! create_error {
 ( $name: expr, $message: expr) => {
  {
    pub struct $name;

    impl fmt::Display for KeyError {
      fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, $message)
      }
    }

    impl fmt::Debug for KeyError {
      fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
      }
    }
  }
 };
}


