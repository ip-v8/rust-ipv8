#![macro_use]
pub mod bits;
pub mod header;
pub mod nestedpayload;
pub mod rawend;
pub mod varlen;

use crate::crypto::signature::{Signature, KeyPair, sign_packet, verify_packet, Ed25519PublicKey};
use crate::payloads::binmemberauthenticationpayload::BinMemberAuthenticationPayload;
use crate::payloads::Ipv8Payload;
use crate::serialization::header::Header;
use bincode;
use bincode::ErrorKind;
use serde::{Deserialize, Serialize};
use std::error::Error;

create_error!(HeaderError, "The supplied header was invalid");

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Packet(pub Vec<u8>);

impl Clone for Packet {
    fn clone(&self) -> Packet {
        Packet(self.0.to_vec())
    }
}

#[derive(Debug, PartialEq)]
pub struct PacketDeserializer {
    pub pntr: Packet,
    pub index: usize,
}

/// iterates over a packet to extract it's possibly multiple payloads
impl PacketDeserializer {
    /// Deserializes a stream of bytes into an ipv8 payload. Which payload is inferred by the type of T which is generic.
    /// T has to be deserializable and implement the Ipv8Payload trait.
    pub fn next_payload<T>(&mut self) -> Result<T, Box<ErrorKind>>
    where
        for<'de> T: Deserialize<'de> + Ipv8Payload + Serialize,
    {
        let res: T = bincode::config()
            .big_endian()
            .deserialize(&self.pntr.0[self.index..])?;

        // the old solution was: self.index += size_of::<T>();
        // this doesnt work as it is not uncommon to return less bytes than was actually in the bytecode (lengths etc)
        // the code below works but is inefficient. TODO: create a more efficient way to do this.
        // tried this:
        /*
          let mut value = &self.pntr.0[self.index ..];
          let oldsize = value.len();
          let res: T = bincode::config().big_endian().deserialize_from(&mut value)?;
          self.index += (oldsize - value.to_owned().len());
        */
        // apparently it is less efficient than recalculating the size as below.
        // on the bench_deserialize_multiple benchmark in the tests section below
        // it got 17,584,554 ns per iteration (where each iteration is 100000 serialize/deserializations
        // while the recalculation takes 11,965,294ns
        self.index += bincode::config().big_endian().serialized_size(&res)? as usize;
        Ok(res)
    }

    pub fn peek_header(&self) -> Result<Header, Box<ErrorKind>> {
        let res: Header = bincode::config()
            .big_endian()
            .deserialize(&self.pntr.0[self.index..])?;
        Ok(res)
    }

    pub fn pop_header(&mut self) -> Result<Header, Box<ErrorKind>> {
        let res = self.peek_header()?;
        self.index += res.size;
        Ok(res)
    }

    pub fn skip_header(mut self) -> Result<Self, Box<ErrorKind>> {
        self.pop_header()?;
        Ok(self)
    }

    fn len(&self) -> usize {
        self.pntr.0.len()
    }

    /// This should be in most cases the first method to be called when receiving a packet. It **assumes** there is a
    /// BinMemberAuthenticationPayload at the start of the message (AND DOES NOT CHECK IF IT IS OR NOT). It extracts it and the
    /// with the sign put at the end by the sender by calling Packet.sign() verifies that the packet is still inyhtact.
    ///
    /// If the public key has been acquired in any other way (i.e. there is no BinMemberAuthenticationPayload at the start)
    /// use the Packet.verify_with() function instead.
    pub fn verify(&mut self) -> bool {
        let authpayload: BinMemberAuthenticationPayload = match self.next_payload() {
            Ok(i) => i,
            Err(_) => return false, // when an error occurred the signature is certainly not right.
        };
        self.verify_with(authpayload.public_key_bin)
    }

    /// Does the same thing as the Packet. verify method. Takes a public key as second argument instead of extracting it from the packet itself
    /// through a BinMemberAuthenticationPayload
    pub fn verify_with(&mut self, pkey: Ed25519PublicKey) -> bool {
        let keylength = Signature::ED25519_SIGNATURE_BYTES;

        let datalen = self.len();

        let signature = self.pntr.0[datalen - keylength..].to_owned();

        self.pntr.0.truncate(datalen - keylength);
        verify_packet(&pkey, &mut self.pntr, &*signature)
    }
}

impl Packet {
    /// Creates a new packet with a given header.
    pub fn new(header: Header) -> Result<Self, Box<dyn Error>> {
        let mut res = Self(vec![]);
        res.0
            .extend(match bincode::config().big_endian().serialize(&header) {
                Ok(i) => i,
                Err(_) => return Err(Box::new(HeaderError)),
            });
        Ok(res)
    }

    pub fn raw(&self) -> &[u8] {
        &*self.0
    }

    /// Signs a packet. After this, new payloads must under no circumstances be added as this will
    /// break the verification process on the receiving end. There is no check for this by design for a speed boost
    /// (though this may or may not be revisited later). Sign deliberately consumes self and returns it again so it can
    /// be assigned to a new, immutable variable to never be changed again.
    ///
    /// Verification of this signature can only happen when a public key is known at the receiving end. The user of the rust-ipv8 library
    /// is responsible for this to already have been packed into the final packet or alternatively already known by the receiver.
    ///
    /// To verify signatures first transform the Packet into a PacketIterator with Packet.deserialize_multiple and then use the PacketIterator.verify() or
    /// PacketIterator.verify_with() method.
    pub fn sign(mut self, keypair: KeyPair) -> Result<Self, Box<dyn Error>> {
        //        let signature = Signature::from_bytes(&*self.0, skey)?;
        let signature = sign_packet(keypair, &self)?;
        self.add(&signature)?;

        // now this packet *must not* be modified anymore
        Ok(self)
    }

    /// Deserializes a stream of bytes into ipv8 payloads.
    pub fn start_deserialize(self) -> PacketDeserializer {
        PacketDeserializer {
            pntr: self,
            index: 0,
        }
    }

    pub fn add<T>(&mut self, obj: &T) -> Result<(), Box<ErrorKind>>
    where
        T: Ipv8Payload + Serialize,
    {
        self.0
            .extend(bincode::config().big_endian().serialize(&obj)?);
        Ok(())
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct TestPayload1 {
        test: u16,
    }

    impl Ipv8Payload for TestPayload1 {
        // doesnt have anything but needed for the default implementation (as of right now)
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct TestPayload2 {
        test: u32,
    }

    impl Ipv8Payload for TestPayload2 {
        // doesnt have anything but needed for the default implementation (as of right now)
    }
    //
    //  // only works with feature(test) and with `extern crate test; use test::Bencher;`
    //  extern crate test;
    //  use test::Bencher;
    //  use crate::serialization::varlen::VarLen16;
    //  #[derive(Debug, PartialEq, Serialize, Deserialize)]
    //  struct TestPayload3 {
    //    test:VarLen16,
    //  }
    //
    //  impl Ipv8Payload for TestPayload3 {
    //    // doesnt have anything but needed for the default implementation (as of right now)
    //  }
    //
    //  #[bench]
    //  fn bench_deserialize_multiple(b: &mut Bencher){
    //    let mut tst = vec![];
    //    for i in 0..10000{
    //      tst.push((i%255) as u8);
    //    }
    //
    //    b.iter(move || {
    //      let n = test::black_box(100000);
    //      for _i in 0..n{
    //
    //        let a = TestPayload1{test:42};
    //        let b = TestPayload2{test:43};
    //        let c = TestPayload1{test:10};
    ////        let c = TestPayload3{test:VarLen16(tst.to_owned())};
    //
    //        let mut ser_tmp = Packet::serialize(&a).unwrap();
    //        ser_tmp.add(&b).unwrap();
    //        ser_tmp.add(&c).unwrap();
    //
    //        let mut deser_iterator = ser_tmp.deserialize_multiple();
    //        assert_eq!(a,deser_iterator.next().unwrap());
    //        assert_eq!(b,deser_iterator.next().unwrap());
    //        assert_eq!(c,deser_iterator.next().unwrap());
    //      }
    //    });
    //  }

    #[test]
    fn test_raw() {
        let header = create_test_header!();
        let packet = Packet::new(header).unwrap();
        let raw = packet.raw();
        assert_eq!(
            raw,
            &[0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 42,]
        )
    }

    #[test]
    fn test_peek_header() {
        let packet = Packet::new(create_test_header!()).unwrap();
        let deserializer = packet.start_deserialize();
        let header1 = deserializer.peek_header().unwrap();
        let header2 = deserializer.peek_header().unwrap();

        assert_eq!(header1, header2);
    }

    #[test]
    fn test_sign_verify_ed25519() {
        let a = TestPayload1 { test: 42 };
        let mut packet = Packet::new(create_test_header!()).unwrap();
        packet.add(&a).unwrap();

        let pk = KeyPair::from_seed_unchecked([
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31,
        ])
        .unwrap();

        let publickey = pk.public_key().unwrap();

        let signed = packet.sign(pk).unwrap();

        let mut deser_iterator = signed.start_deserialize();
        let valid = deser_iterator.verify_with(publickey);
        assert!(valid);
    }

    #[test]
    fn test_serialize_multiple() {
        let a = TestPayload1 { test: 42 };
        let b = TestPayload2 { test: 43 };
        let c = TestPayload1 { test: 44 };

        let mut packet = Packet::new(create_test_header!()).unwrap();

        packet.add(&a).unwrap();
        packet.add(&b).unwrap();
        packet.add(&c).unwrap();

        assert_eq!(
            Packet(vec![
                0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 42, 0, 42, 0, 0,
                0, 43, 0, 44
            ]),
            packet
        );
    }

    #[test]
    fn test_deserialize_multiple() {
        let a = TestPayload1 { test: 42 };
        let b = TestPayload2 { test: 43 };
        let c = TestPayload1 { test: 44 };

        let mut packet = Packet::new(create_test_header!()).unwrap();
        packet.add(&a).unwrap();
        packet.add(&b).unwrap();
        packet.add(&c).unwrap();

        let mut deser_iterator = packet.start_deserialize().skip_header().unwrap();
        assert_eq!(a, deser_iterator.next_payload().unwrap());
        assert_eq!(b, deser_iterator.next_payload().unwrap());
        assert_eq!(c, deser_iterator.next_payload().unwrap());
    }

    #[test]
    fn test_deserialize_multiple_more() {
        let a = TestPayload1 { test: 42 };
        let b = TestPayload2 { test: 43 };
        let c = TestPayload1 { test: 44 };

        let mut ser_tmp = Packet::new(create_test_header!()).unwrap();
        ser_tmp.add(&a).unwrap();
        ser_tmp.add(&b).unwrap();
        ser_tmp.add(&c).unwrap();

        let mut deser_iterator = ser_tmp.start_deserialize().skip_header().unwrap();
        assert_eq!(a, deser_iterator.next_payload().unwrap());
        assert_eq!(b, deser_iterator.next_payload().unwrap());
        assert_eq!(c, deser_iterator.next_payload().unwrap());

        let last: Result<TestPayload1, Box<ErrorKind>> = deser_iterator.next_payload();
        match last {
            Ok(_) => assert!(false, "this should throw an error as there is no next"),
            Err(_) => assert!(true),
        };
    }
}
