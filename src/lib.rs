#[macro_use]
extern crate log;

pub mod error;
pub mod serialization;

pub mod community;
pub mod configuration;
pub mod crypto;
pub mod event;
pub mod networking;
pub mod payloads;

use configuration::Config;
use crate::networking::NetworkManager;
use std::error::Error;

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
}

impl IPv8 {
    pub fn new(config: configuration::Config) -> Result<Self, Box<dyn Error>> {
        let networkmanager =
            NetworkManager::new(&config.socketaddress, config.threadcount.to_owned())?;
        Ok(IPv8 {
            config,
            networkmanager,
        })
    }

    pub fn start(self) {
        self.networkmanager.start(&self.config);
    }
}
