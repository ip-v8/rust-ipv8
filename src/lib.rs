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
use crate::networking::NetworkManager;
use std::error::Error;
use crate::community::CommunityRegistry;

/// The IPv8 instance.
/// This struct is how you can interact with the network.
///
/// To create a new IPv8 instance with the default configuration do this:
/// ```
/// use ipv8::IPv8;
/// use ipv8::configuration::Config;
///
/// let ipv8 = IPv8::new(Config::default());
/// ```
pub struct IPv8 {
    pub config: Config,
    pub networkmanager: NetworkManager,

    /// The registry containing all the communities
    pub communities: CommunityRegistry,
}

impl IPv8 {
    pub fn new(config: configuration::Config) -> Result<Self, Box<dyn Error>> {
        let networkmanager = NetworkManager::new(
            &config.sending_address,
            &config.receiving_address,
            config.threadcount.to_owned(),
        )?;
        Ok(IPv8 {
            config,
            networkmanager,
            communities: CommunityRegistry::default(),
        })
    }

    pub fn start(self) {
        self.networkmanager.start(&self.config);
    }
}
