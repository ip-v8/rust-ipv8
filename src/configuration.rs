use crate::networking::address::Address;
use std::net::Ipv4Addr;

pub struct Config {
    /// Default list of host used for peer discovery
    pub default_hosts: Vec<Address>,
    /// from py-ipv8 configuration. UDP socket address.
    /// There split up in "address" and "port"
    pub socketaddress: Address,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            socketaddress: Address {
                address: Ipv4Addr::new(0, 0, 0, 0),
                port: 8090,
            },

            default_hosts: vec![
                // Dispersy
                Address {
                    address: Ipv4Addr::new(130, 161, 119, 206),
                    port: 6421,
                },
                Address {
                    address: Ipv4Addr::new(130, 161, 119, 206),
                    port: 6422,
                },
                Address {
                    address: Ipv4Addr::new(131, 180, 27, 155),
                    port: 6423,
                },
                Address {
                    address: Ipv4Addr::new(131, 180, 27, 156),
                    port: 6424,
                },
                Address {
                    address: Ipv4Addr::new(131, 180, 27, 161),
                    port: 6427,
                },
                // IPv8
                Address {
                    address: Ipv4Addr::new(131, 180, 27, 161),
                    port: 6521,
                },
                Address {
                    address: Ipv4Addr::new(131, 180, 27, 161),
                    port: 6522,
                },
                Address {
                    address: Ipv4Addr::new(131, 180, 27, 162),
                    port: 6523,
                },
                Address {
                    address: Ipv4Addr::new(131, 180, 27, 162),
                    port: 6524,
                },
                Address {
                    address: Ipv4Addr::new(130, 161, 119, 215),
                    port: 6525,
                },
                Address {
                    address: Ipv4Addr::new(130, 161, 119, 215),
                    port: 6526,
                },
                Address {
                    address: Ipv4Addr::new(81, 171, 27, 194),
                    port: 6527,
                },
                Address {
                    address: Ipv4Addr::new(81, 171, 27, 194),
                    port: 6528,
                },
            ],
        }
    }
}
