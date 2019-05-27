pub mod bits;
pub mod rawend;
pub mod varlen;
pub mod nestedpayload;
pub mod header;

use bincode;
use crate::networking::payloads::Ipv8Payload;
use bincode::ErrorKind;
use serde::{Deserialize,Serialize};
use crate::networking::crypto::keytypes::{PrivateKey, PublicKey};
use crate::networking::payloads::binmemberauthenticationpayload::BinMemberAuthenticationPayload;
use crate::networking::crypto::signature::Signature;
use std::error::Error;
use crate::networking::serialization::header::Header;
use std::fmt;

create_error!(HeaderError, "The supplied header was invalid");

#[derive(Debug,Serialize,Deserialize, PartialEq)]
pub struct Packet(
  pub Vec<u8>,
);

#[derive(Debug, PartialEq)]
pub struct PacketIterator{
  pub pntr: Packet,
  pub index: usize,
}

/// iterates over a packet to extract it's possibly multiple payloads
impl PacketIterator{
  /// Deserializes a stream of bytes into an ipv8 payload. Which payload is inferred by the type of T which is generic.
  /// T has to be deserializable and implement the Ipv8Payload trait.
  pub fn next_payload<T>(&mut self) -> Result<T, Box<ErrorKind>>
    where for<'de> T: Deserialize<'de> + Ipv8Payload + Serialize
  {
    let res: T = bincode::config().big_endian().deserialize(&self.pntr.0[self.index ..])?;
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

  pub fn get_header<T>(&mut self) -> Result<T, Box<ErrorKind>>
    where for<'de> T: Header + Serialize + Deserialize<'de>{
    let res: T = bincode::config().big_endian().deserialize(&self.pntr.0[self.index ..])?;
    self.index += bincode::config().big_endian().serialized_size(&res)? as usize;
    Ok(res)
  }

  pub fn skip_header<T>(mut self) -> Self
    where T: Header{
    self.index += T::size();
    self
  }

  fn len(&self) -> usize {
    self.pntr.0.len()
  }

  /// This should be in most cases the first method to be called when receiving a packet. It **assumes** there is a
  /// BinMemberAuthenticationPayload at the start of the message (AND DOES NOT CHECK IF IT IS OR NOT). It extracts it and the
  /// with the sign put at the end by the sender by calling Packet.sign() verifies that the packet is still intact.
  ///
  /// If the public key has been acquired in any other way (i.e. there is no BinMemberAuthenticationPayload at the start)
  /// use the Packet.verify_with() function instead.
  pub fn verify(&mut self) -> bool{
    let authpayload: BinMemberAuthenticationPayload = match self.next_payload(){
      Ok(i) => i,
      Err(_) => return false // when an error occurred the signature is certainly not right.
    };
    self.verify_with(authpayload.public_key_bin)
  }

  /// Does the same thing as the Packet. verify method. Takes a public key as second argument instead of extracting it from the packet itself
  /// through a BinMemberAuthenticationPayload
  pub fn verify_with(&mut self, pkey: PublicKey) -> bool{
    let signaturelength = pkey.size();

    let datalen = self.pntr.0.len();
    let signature = Signature{signature:self.pntr.0[datalen-signaturelength..].to_vec()};
    self.pntr.0.truncate(datalen - signaturelength);
    signature.verify(&*self.pntr.0,pkey)
  }
}

impl Packet{
  pub fn new<T>(header: T) -> Result<Self, Box<Error>>
    where T: Header + Serialize {
    let mut res = Self(vec![]);
    res.0.extend(match bincode::config().big_endian().serialize(&header){
      Ok(i)=>i,
      Err(_) => return Err(Box::new(HeaderError))
    });
    Ok(res)
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
  pub fn sign(mut self, skey: PrivateKey) -> Result<Self, Box<Error>>{
    let signature = Signature::from_bytes(&*self.0, skey)?;
    self.add(&signature)?;
    // now this packet *must not* be modified anymore
    Ok(self)
  }

  /// Deserializes a stream of bytes into ipv8 payloads.
  pub fn start_deserialize(self) -> PacketIterator {
    PacketIterator{
      pntr : self,
      index : 0,
    }
  }

  pub fn add<T>(&mut self, obj: &T) -> Result<(), Box<ErrorKind>>
    where T: Ipv8Payload + Serialize {

    self.0.extend(bincode::config().big_endian().serialize(&obj)?);
    Ok(())
  }

  fn len(&self) -> usize {
    self.0.len()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use serde::{Serialize,Deserialize};
  use rust_sodium::crypto::sign::ed25519;
  use crate::networking::serialization::header::{TEST_HEADER, DefaultHeader};

  #[derive(Debug, PartialEq, Serialize, Deserialize)]
  struct TestPayload1 {
    test:u16,
  }

  impl Ipv8Payload for TestPayload1 {
    // doesnt have anything but needed for the default implementation (as of right now)
  }

  #[derive(Debug, PartialEq, Serialize, Deserialize)]
  struct TestPayload2 {
    test:u32,
  }

  impl Ipv8Payload for TestPayload2 {
    // doesnt have anything but needed for the default implementation (as of right now)
  }
//
//  // only works with feature(test) and with `extern crate test; use test::Bencher;`
//  extern crate test;
//  use test::Bencher;
//  use crate::networking::serialization::varlen::VarLen16;
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
  fn test_sign_verify_verylow(){
    let a = TestPayload1{test:42};
    let mut packet = Packet::new(TEST_HEADER).unwrap();
    packet.add(&a).unwrap();

    let skey = openssl::pkey::PKey::private_key_from_pem("-----BEGIN EC PRIVATE KEY-----\nMFMCAQEEFQKu4aaDxyTSj92iquQP5CIdbagLP6AHBgUrgQQAAaEuAywABABQ76xopUysBuWInGkX+S4elFdpOQZphgLlc6ksoim+5DgUZEBPp+B2Dg==\n-----END EC PRIVATE KEY-----".as_bytes()).unwrap();
    let pkey = openssl::pkey::PKey::public_key_from_pem("-----BEGIN PUBLIC KEY-----\nMEAwEAYHKoZIzj0CAQYFK4EEAAEDLAAEAFDvrGilTKwG5YicaRf5Lh6UV2k5BmmGAuVzqSyiKb7kOBRkQE+n4HYO\n-----END PUBLIC KEY-----".as_bytes()).unwrap();

    let signed = packet.sign(PrivateKey::OpenSSLVeryLow(skey)).unwrap();

    let mut deser_iterator = signed.start_deserialize();
    let valid = deser_iterator.verify_with(PublicKey::OpenSSLVeryLow(pkey));
    assert!(valid);
  }

  #[test]
  fn test_sign_verify_low(){
    let a = TestPayload1{test:42};
    let mut packet = Packet::new(TEST_HEADER).unwrap();
    packet.add(&a).unwrap();

    let skey = openssl::pkey::PKey::private_key_from_pem("-----BEGIN EC PRIVATE KEY-----\nMG0CAQEEHQ7vns0bhePCngPc4WeP3wnglzSrml0HdQ+jcpfAoAcGBSuBBAAaoUAD\nPgAEAe2ikH75P/vkdl1Bu8tP/WjOeB6LRxW11qGQNUmUAaFxQ7zff5eZyppMv7D0\n9sRcEuSNjk5nUQgTe6zV\n-----END EC PRIVATE KEY-----".as_bytes()).unwrap();
    let pkey = openssl::pkey::PKey::public_key_from_pem("-----BEGIN PUBLIC KEY-----\nMFIwEAYHKoZIzj0CAQYFK4EEABoDPgAEAe2ikH75P/vkdl1Bu8tP/WjOeB6LRxW11qGQNUmUAaFxQ7zff5eZyppMv7D09sRcEuSNjk5nUQgTe6zV\n-----END PUBLIC KEY-----".as_bytes()).unwrap();

    let signed = packet.sign(PrivateKey::OpenSSLLow(skey)).unwrap();

    let mut deser_iterator = signed.start_deserialize();
    let valid = deser_iterator.verify_with(PublicKey::OpenSSLLow(pkey));
    assert!(valid);
  }

  #[test]
  fn test_sign_verify_medium(){
    let a = TestPayload1{test:42};
    let mut packet = Packet::new(TEST_HEADER).unwrap();
    packet.add(&a).unwrap();

    let skey = openssl::pkey::PKey::private_key_from_pem("-----BEGIN EC PRIVATE KEY-----\nMIGvAgEBBDNDkh1KSwaBgRj5GGcbYm2qWI5TyBVkOeMVkWWX5+8Dmd44OoSzmR5xCmc1DWuEsasIhhagBwYFK4EEACShbANqAAQAP5r6iYsyTkM7Hea2/tc95iGXV3oCXMLxSWiR/vF/zKjHkPClBN8BQBbBCMjpeS1xLZMUAUi2RoJN69jQevTG+vfhzBNqxIE0dazxbLMvx3wZ6Bol918H8oAa31axHKVaz3SbKLbDTw==\n-----END EC PRIVATE KEY-----".as_bytes()).unwrap();
    let pkey = openssl::pkey::PKey::public_key_from_pem("-----BEGIN PUBLIC KEY-----\nMH4wEAYHKoZIzj0CAQYFK4EEACQDagAEAD+a+omLMk5DOx3mtv7XPeYhl1d6AlzC8Ulokf7xf8yox5DwpQTfAUAWwQjI6XktcS2TFAFItkaCTevY0Hr0xvr34cwTasSBNHWs8WyzL8d8GegaJfdfB/KAGt9WsRylWs90myi2w08=\n-----END PUBLIC KEY-----".as_bytes()).unwrap();

    let signed = packet.sign(PrivateKey::OpenSSLMedium(skey)).unwrap();

    let mut deser_iterator = signed.start_deserialize();
    let valid = deser_iterator.verify_with(PublicKey::OpenSSLMedium(pkey));
    assert!(valid);
  }

  #[test]
  fn test_sign_verify_high(){
    let a = TestPayload1{test:42};
    let mut packet = Packet::new(TEST_HEADER).unwrap();
    packet.add(&a).unwrap();

    let skey = openssl::pkey::PKey::private_key_from_pem("-----BEGIN EC PRIVATE KEY-----\nMIHuAgEBBEgCQPcwiTfJz3T0/fDqAgvtTO3fvCobbxvJAnsDKQwjJbK9Ak2njemFanI8BOGp/1Mi6nrjfJs9+8h9LhUIYsrJ2j7piRxo2SygBwYFK4EEACehgZUDgZIABAJW+0vOn4V4P7Drsg4IxTtrM7OLA5sUwnBxDyhDcyXfmAdmmtZabrTiBb5jozZ0rXkoUIGOUnaaYH+k+NlbDVBbXtIQbmwpOQTzMTTC/oJi5TJUFc6G3529hTLStV3lILPks4SPk2DPRDC4oC/jRpMXn9VphjzT4gjruhTxVaoEAyi3YmdQpIBXzWVD/lOOhQ==\n-----END EC PRIVATE KEY-----".as_bytes()).unwrap();
    let pkey = openssl::pkey::PKey::public_key_from_pem("-----BEGIN PUBLIC KEY-----\nMIGnMBAGByqGSM49AgEGBSuBBAAnA4GSAAQCVvtLzp+FeD+w67IOCMU7azOziwObFMJwcQ8oQ3Ml35gHZprWWm604gW+Y6M2dK15KFCBjlJ2mmB/pPjZWw1QW17SEG5sKTkE8zE0wv6CYuUyVBXOht+dvYUy0rVd5SCz5LOEj5Ngz0QwuKAv40aTF5/VaYY80+II67oU8VWqBAMot2JnUKSAV81lQ/5TjoU=\n-----END PUBLIC KEY-----".as_bytes()).unwrap();

    let signed = packet.sign(PrivateKey::OpenSSLHigh(skey)).unwrap();

    let mut deser_iterator = signed.start_deserialize();
    let valid = deser_iterator.verify_with(PublicKey::OpenSSLHigh(pkey));
    assert!(valid);
  }

  #[test]
  fn test_sign_verify_ed25519(){
    let a = TestPayload1{test:42};
    let mut packet = Packet::new(TEST_HEADER).unwrap();
    packet.add(&a).unwrap();

    let seed = ed25519::Seed::from_slice(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,]).unwrap();
    let (pkey_tmp,skey_tmp) = ed25519::keypair_from_seed(&seed);

    let seed = ed25519::Seed::from_slice(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,]).unwrap();
    let (e_pkey_tmp,e_skey_tmp) = ed25519::keypair_from_seed(&seed);


    let skey = PrivateKey::Ed25519(e_skey_tmp, skey_tmp);
    let pkey = PublicKey::Ed25519(e_pkey_tmp,pkey_tmp);

    let signed = packet.sign(skey).unwrap();

    let mut deser_iterator = signed.start_deserialize();
    let valid = deser_iterator.verify_with(pkey);
    assert!(valid);
  }

  # [test]
  fn test_serialize_multiple(){
    let a = TestPayload1{test:42};
    let b = TestPayload2{test:43};
    let c = TestPayload1{test:44};

    let mut packet = Packet::new(TEST_HEADER).unwrap();

    packet.add(&a).unwrap();
    packet.add(&b).unwrap();
    packet.add(&c).unwrap();

    assert_eq!(Packet(vec![0,42,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,42,0, 42, 0, 0, 0, 43, 0, 44]),packet);
  }

  # [test]
  fn test_deserialize_multiple(){
    let a = TestPayload1{test:42};
    let b = TestPayload2{test:43};
    let c = TestPayload1{test:44};

    let mut packet = Packet::new(TEST_HEADER).unwrap();
    packet.add(&a).unwrap();
    packet.add(&b).unwrap();
    packet.add(&c).unwrap();

    let mut deser_iterator = packet.start_deserialize().skip_header::<DefaultHeader>();
    assert_eq!(a,deser_iterator.next_payload().unwrap());
    assert_eq!(b,deser_iterator.next_payload().unwrap());
    assert_eq!(c,deser_iterator.next_payload().unwrap());
  }

  # [test]
  fn test_deserialize_multiple_more(){
    let a = TestPayload1{test:42};
    let b = TestPayload2{test:43};
    let c = TestPayload1{test:44};

    let mut ser_tmp = Packet::new(TEST_HEADER).unwrap();
    ser_tmp.add(&a).unwrap();
    ser_tmp.add(&b).unwrap();
    ser_tmp.add(&c).unwrap();


    let mut deser_iterator = ser_tmp.start_deserialize().skip_header::<DefaultHeader>();
    assert_eq!(a,deser_iterator.next_payload().unwrap());
    assert_eq!(b,deser_iterator.next_payload().unwrap());
    assert_eq!(c,deser_iterator.next_payload().unwrap());

    let last:Result<TestPayload1,Box<ErrorKind>> = deser_iterator.next_payload();
    match last {
      Ok(_) => assert!(false, "this should throw an error as there is no next"),
      Err(_) => assert!(true)
    };
  }
}
