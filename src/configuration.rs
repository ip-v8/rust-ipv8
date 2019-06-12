use crate::networking::address::Address;
use std::net::Ipv4Addr;
use crate::community::CommunityRegistry;
use std::time::Duration;

pub struct Config {
    /// the amount of space reserved for queueing up incoming messages (messages)
    pub queuesize: usize,
    /// the size of the buffer reserved for incoming messages (bytes)
    pub buffersize: usize,
    /// frequency at which polling times out and events are checked (ms)
    /// None is as fast as possible
    pub pollinterval: Option<Duration>,
    /// the max number of threads to use in the network manager. 0 is #cores
    pub threadcount: usize,

    /// Default list of host used for peer discovery
    pub default_hosts: Vec<Address>,
    /// from py-ipv8 configuration. UDP socket address.
    /// There split up in "address" and "port"
    pub socketaddress: Address,

    /// The registry containing all the communities
    pub communities: CommunityRegistry,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            queuesize: 100,
            buffersize: 2048,
            pollinterval: None,

            // zero means equal to number of cores
            threadcount: 0,

            socketaddress: Address {
                address: Ipv4Addr::new(0, 0, 0, 0),
                port: 8090,
            },

            communities: CommunityRegistry::default(),

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
