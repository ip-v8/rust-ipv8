pub mod error;
pub mod serialization;
pub mod configuration;
pub mod crypto;
pub mod event;
pub mod networking;
pub mod payloads;

use configuration::Config;

/// The IPv8 instance.
/// This struct is how you can interact with the network.
///
/// To create a new IPv8 instance with the default configuration do this:
/// ```
/// use ipv8::IPv8;
/// use ipv8::configuration::Config;
///
/// let ipv8_instance = IPv8::new(Config::default());
/// ```
pub struct IPv8 {
    config: Config,
}

impl IPv8 {
    pub fn new(config: configuration::Config) -> Self {
        IPv8 { config }
    }
}
