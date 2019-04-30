mod config;
mod ipv8;

pub use config::Config;
use ipv8::IPv8;

/**
 * Create a new instance of IPv8.
 */
pub fn new(config: Config) -> IPv8 {
  return IPv8::new(config);
}
