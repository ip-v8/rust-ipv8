use ipv8::networking::address;

fn main() {
  let addr = address::IPV4Addr {
    address: String::from("127.0.0.1"),
    port: 8000,
  };

  println!("{}", addr);
}
