use ipv8::networking::address::Address;
use std::net::Ipv4Addr;

fn main() {
  let addr = Address {
    address: Ipv4Addr::new(127, 0, 0, 1),
    port: 8000,
  };

  println!("{}", addr);
}
