use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::fmt;
use std::marker::PhantomData;

/// This struct represents the bits inside an u8 by unpacking them into booleans.
///
/// Mostly here to achieve feature parity with py-ipv8 see
/// [py-ipv8 code](https://github.com/Tribler/py-ipv8/blob/57c1aa73eee8a3b7ee6ad48482fc2e0d5849415e/ipv8/messaging/serialization.py#L84.)
#[derive(Default, PartialEq, Debug)]
pub struct Bits {
    pub bit0: bool,
    pub bit1: bool,
    pub bit2: bool,
    pub bit3: bool,
    pub bit4: bool,
    pub bit5: bool,
    pub bit6: bool,
    pub bit7: bool,
}

fn bool_to_u8(data: bool) -> u8 {
    if data {
        1
    } else {
        0
    }
}

impl Bits {
    pub fn new() -> Self {
        Bits {
            bit0: false,
            bit1: false,
            bit2: false,
            bit3: false,
            bit4: false,
            bit5: false,
            bit6: false,
            bit7: false,
        }
    }

    pub fn from_bools(data: (bool, bool, bool, bool, bool, bool, bool, bool)) -> Self {
        Bits {
            bit0: data.0,
            bit1: data.1,
            bit2: data.2,
            bit3: data.3,
            bit4: data.4,
            bit5: data.5,
            bit6: data.6,
            bit7: data.7,
        }
    }

    /// convert an u8 into a bits struct.
    pub fn from_u8(num: u8) -> Self {
        Bits {
            bit0: ((num) & 1) > 0,
            bit1: ((num >> 1) & 1) > 0,
            bit2: ((num >> 2) & 1) > 0,
            bit3: ((num >> 3) & 1) > 0,
            bit4: ((num >> 4) & 1) > 0,
            bit5: ((num >> 5) & 1) > 0,
            bit6: ((num >> 6) & 1) > 0,
            bit7: ((num >> 7) & 1) > 0,
        }
    }

    pub fn to_u8(&self) -> u8 {
        (bool_to_u8(self.bit0))
            | (bool_to_u8(self.bit1) << 1)
            | (bool_to_u8(self.bit2) << 2)
            | (bool_to_u8(self.bit3) << 3)
            | (bool_to_u8(self.bit4) << 4)
            | (bool_to_u8(self.bit5) << 5)
            | (bool_to_u8(self.bit6) << 6)
            | (bool_to_u8(self.bit7) << 7)
    }
}

impl Serialize for Bits {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Bits", 1)?;
        state.serialize_field("value", &self.to_u8())?;
        state.end()
    }
}

/// used for deserializing bits
struct BitsVisitor {
    marker: PhantomData<fn() -> Bits>,
}

impl<'de> Visitor<'de> for BitsVisitor {
    type Value = Bits;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Bits")
    }

    fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Bits::from_u8(value))
    }
}

impl<'de> Deserialize<'de> for Bits {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_u8(BitsVisitor {
            marker: PhantomData,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bincode;

    #[test]
    fn test_serialization() {
        let b = Bits::from_bools((false, true, false, true, false, true, false, false));
        assert_eq!(vec![42], bincode::serialize(&b).unwrap());
    }

    #[test]
    fn test_deserialization() {
        let b = Bits::from_bools((false, true, false, true, false, true, false, false));
        assert_eq!(
            b,
            bincode::deserialize(&bincode::serialize(&b).unwrap()).unwrap()
        );
    }

    #[test]
    fn test_creation() {
        let b = Bits::from_bools((false, true, false, true, false, true, false, false));
        assert_eq!(b.bit0, false);
        assert_eq!(b.bit1, true);
        assert_eq!(b.bit2, false);
        assert_eq!(b.bit3, true);
        assert_eq!(b.bit4, false);
        assert_eq!(b.bit5, true);
        assert_eq!(b.bit6, false);
        assert_eq!(b.bit7, false);
    }

    #[test]
    fn test_tou8() {
        let b = Bits::from_bools((false, true, false, true, false, true, false, false));
        assert_eq!(b.to_u8(), 42);
    }

    #[test]
    fn test_fromu8() {
        let b = Bits::from_u8(42);
        assert_eq!(b.bit0, false);
        assert_eq!(b.bit1, true);
        assert_eq!(b.bit2, false);
        assert_eq!(b.bit3, true);
        assert_eq!(b.bit4, false);
        assert_eq!(b.bit5, true);
        assert_eq!(b.bit6, false);
        assert_eq!(b.bit7, false);
    }
}
