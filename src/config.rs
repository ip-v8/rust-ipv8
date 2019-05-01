use crate::networking::address::*;

pub struct Config {
  /// Default list of host used for peer discovery
  pub default_hosts: Vec<IPAddress>,
}

impl Default for Config {
  fn default() -> Self {
    Config {
      default_hosts: vec![
        // Dispersy
        IPAddress {
          address: String::from("130.161.119.206"),
          port: 6421,
          version: IPVersion::IPV4,
        },
        IPAddress {
          address: String::from("130.161.119.206"),
          port: 6422,
          version: IPVersion::IPV4,
        },
        IPAddress {
          address: String::from("131.180.27.155"),
          port: 6423,
          version: IPVersion::IPV4,
        },
        IPAddress {
          address: String::from("131.180.27.156"),
          port: 6424,
          version: IPVersion::IPV4,
        },
        IPAddress {
          address: String::from("131.180.27.161"),
          port: 6427,
          version: IPVersion::IPV4,
        },
        // IPv8
        IPAddress {
          address: String::from("131.180.27.161"),
          port: 6521,
          version: IPVersion::IPV4,
        },
        IPAddress {
          address: String::from("131.180.27.161"),
          port: 6522,
          version: IPVersion::IPV4,
        },
        IPAddress {
          address: String::from("131.180.27.162"),
          port: 6523,
          version: IPVersion::IPV4,
        },
        IPAddress {
          address: String::from("131.180.27.162"),
          port: 6524,
          version: IPVersion::IPV4,
        },
        IPAddress {
          address: String::from("130.161.119.215"),
          port: 6525,
          version: IPVersion::IPV4,
        },
        IPAddress {
          address: String::from("130.161.119.215"),
          port: 6526,
          version: IPVersion::IPV4,
        },
        IPAddress {
          address: String::from("81.171.27.194"),
          port: 6527,
          version: IPVersion::IPV4,
        },
        IPAddress {
          address: String::from("81.171.27.194"),
          port: 6528,
          version: IPVersion::IPV4,
        },
      ],
    }
  }
}
