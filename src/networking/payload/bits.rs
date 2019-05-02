/// This struct represents the bits inside an u8 by unpacking them into booleans.
/// Mostly here to achieve feature parity with py-ipv8
/// see https://github.com/Tribler/py-ipv8/blob/57c1aa73eee8a3b7ee6ad48482fc2e0d5849415e/ipv8/messaging/serialization.py#L84.
#[derive(PartialEq, Debug)]
pub struct Bits {
  bit0: bool,
  bit1: bool,
  bit2: bool,
  bit3: bool,
  bit4: bool,
  bit5: bool,
  bit6: bool,
  bit7: bool,
}

impl Bits {
  /// convert an u8 into a bits struct.
  pub fn from(num: u8) -> Self {
    Bits {
      bit0: ((num >> 0) & 1) > 0,
      bit1: ((num >> 1) & 1) > 0,
      bit2: ((num >> 2) & 1) > 0,
      bit3: ((num >> 3) & 1) > 0,
      bit4: ((num >> 4) & 1) > 0,
      bit5: ((num >> 5) & 1) > 0,
      bit6: ((num >> 6) & 1) > 0,
      bit7: ((num >> 7) & 1) > 0,
    }
  }
}
