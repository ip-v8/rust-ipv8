pub mod configuration;

pub mod event;
pub mod networking;

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

