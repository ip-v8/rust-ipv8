//#![allow(warnings)]
#[macro_use]
pub mod error;
pub mod configuration;
pub mod crypto;
pub mod event;
pub mod networking;
pub mod payloads;
pub mod serialization;

use configuration::Config;

/**
 * The IPv8 instance.
 *
 * This struct is how you can interact with the network.
 */
pub struct IPv8 {
    config: Config,
}

impl IPv8 {
    pub fn new(config: configuration::Config) -> Self {
        IPv8 { config }
    }
}
