//! [README](https://github.com/ip-v8/rust-ipv8/blob/develop/README.md)
#![deny(clippy::missing_docs_in_private_items)]
#[macro_use]
extern crate log;

pub mod error;
pub mod serialization;

pub mod community;
pub mod configuration;
pub mod crypto;
pub mod networking;
pub mod payloads;

use configuration::Config;
use crate::networking::{NetworkSender, NetworkReceiver};
use std::error::Error;
use crate::community::CommunityRegistry;
use rayon::{ThreadPoolBuilder};
use std::sync::Once;

/// The IPv8 instance.
/// This struct is how you can interact with the network.
///
/// To create a new IPv8 instance with the default configuration do this:
/// ```
/// use rust_ipv8::IPv8;
/// use rust_ipv8::configuration::Config;
///
/// let ipv8 = IPv8::new(Config::default());
/// ```
pub struct IPv8 {
    /// Defines the config used forIPv8
    pub config: Config,
    /// The network receiver which forwards the packets to the correct communities
    pub network_receiver: NetworkReceiver,
    /// The sender used for sending packets over the network
    pub network_sender: NetworkSender,

    /// The registry containing all the communities
    pub communities: CommunityRegistry,
}

// To keep track if the threadpool is already started
#[doc(hidden)]
static THREADPOOL_START: Once = Once::new();

impl IPv8 {
    /// Creates a new instance of the ipv8 struct
    pub fn new(config: configuration::Config) -> Result<Self, Box<dyn Error>> {
        // Setup the global threadpool
        {
            let mut started = None;

            THREADPOOL_START.call_once(|| {
                started = Some(
                    ThreadPoolBuilder::new()
                        .num_threads(config.threadcount)
                        .build_global(),
                )
            });

            if let Some(s) = started {
                s?
            }
        }

        let network_receiver = NetworkReceiver::new(&config.receiving_address)?;
        let network_sender = NetworkSender::new(&config.sending_address)?;
        Ok(IPv8 {
            config,
            network_receiver,
            network_sender,
            communities: CommunityRegistry::default(),
        })
    }

    /// Starts ipv8 to actually listen for packets
    pub fn start(self) {
        self.network_receiver.start(&self.config);
    }
}
