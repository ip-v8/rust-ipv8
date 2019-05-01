
use ipv8::networking::address;
use std::string::*;

fn main() {
  let _addr = address::IPV4Addr::new(String::from("127.0.0.1"), 8000);
  println!("{}", _addr);
}

