/**
 * In this example the program attempts to join the network.
 *
 * It does not do much, except for logging the output.
 */

use ipv8;

fn main() {
  let _ = ipv8::new(ipv8::Config::default());
}
