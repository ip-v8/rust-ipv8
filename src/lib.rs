/// public modules
pub mod networking;
pub mod config;

/// private modules
mod ipv8;

use config::Config;
use ipv8::IPv8;

/**
 * Create a new instance of IPv8.
 */
pub fn new(config: Config) -> IPv8 {
  return IPv8::new(config);
}
