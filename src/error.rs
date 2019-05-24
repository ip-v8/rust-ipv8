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
