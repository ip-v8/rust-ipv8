use ipv8::networking::address::*;

fn main() {
  let addr = IPAddress {
    address: String::from("127.0.0.1"),
    port: 8000,
    version: IPVersion::IPV4,
  };

  println!("{}", addr);
}
