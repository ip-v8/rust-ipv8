//! In this module, the IPv8 configuration can be specified.
//!
//! Also contains the default IPv8 configuration.

use std::net::{Ipv4Addr, SocketAddr, IpAddr};
use std::time::Duration;
use crate::networking::address::Address;

/// This struct configures the IPv8 instance. It contains all the options
/// you can change when using rust_ipv8 in your code.
pub struct Config {
    /// the amount of space reserved for queueing up incoming messages (messages)
    pub queuesize: usize,
    /// the size of the buffer reserved for incoming messages (bytes)
    pub buffersize: usize,
    /// frequency at which polling times out and events are checked (ms)
    /// None is as fast as possible
    pub pollinterval: Option<Duration>,
    /// the max number of threads to use in the network manager. 0 is \#cores.
    pub threadcount: usize,

    /// Default list of host used for peer discovery and bootstrapping
    pub default_hosts: Vec<Address>,
    /// UDP Socket address used for sending packets
    pub sending_address: Address,
    /// UDP Socket address used for receiving packets
    pub receiving_address: Address,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            queuesize: 100,
            buffersize: 2048,
            pollinterval: None,

            // zero means equal to number of cores
            threadcount: 0,

            sending_address: Address(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 8000)),
            receiving_address: Address(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0)),

            default_hosts: vec![
                // Dispersy
                Address(SocketAddr::new(
                    IpAddr::V4(Ipv4Addr::new(130, 161, 119, 206)),
                    6421,
                )),
                Address(SocketAddr::new(
                    IpAddr::V4(Ipv4Addr::new(130, 161, 119, 206)),
                    6422,
                )),
                Address(SocketAddr::new(
                    IpAddr::V4(Ipv4Addr::new(131, 180, 27, 155)),
                    6423,
                )),
                Address(SocketAddr::new(
                    IpAddr::V4(Ipv4Addr::new(131, 180, 27, 156)),
                    6424,
                )),
                Address(SocketAddr::new(
                    IpAddr::V4(Ipv4Addr::new(131, 180, 27, 161)),
                    6427,
                )),
                Address(SocketAddr::new(
                    IpAddr::V4(Ipv4Addr::new(131, 180, 27, 161)),
                    6427,
                )),
                // IPv8
                Address(SocketAddr::new(
                    IpAddr::V4(Ipv4Addr::new(131, 180, 27, 161)),
                    6521,
                )),
                Address(SocketAddr::new(
                    IpAddr::V4(Ipv4Addr::new(131, 180, 27, 161)),
                    6522,
                )),
                Address(SocketAddr::new(
                    IpAddr::V4(Ipv4Addr::new(131, 180, 27, 162)),
                    6523,
                )),
                Address(SocketAddr::new(
                    IpAddr::V4(Ipv4Addr::new(131, 180, 27, 162)),
                    6524,
                )),
                Address(SocketAddr::new(
                    IpAddr::V4(Ipv4Addr::new(130, 161, 119, 215)),
                    6525,
                )),
                Address(SocketAddr::new(
                    IpAddr::V4(Ipv4Addr::new(130, 161, 119, 215)),
                    6526,
                )),
                Address(SocketAddr::new(
                    IpAddr::V4(Ipv4Addr::new(81, 171, 27, 194)),
                    6527,
                )),
                Address(SocketAddr::new(
                    IpAddr::V4(Ipv4Addr::new(81, 171, 27, 194)),
                    6528,
                )),
            ],
        }
    }
}
