use openssl;
use rust_sodium::crypto::sign::ed25519;
use std::fmt;

/// gives the signature length of a nid in bytes
///
/// When adding new curves their length should be added here.

pub enum PublicKey{
  OpenSSLVeryLow(openssl::pkey::PKey<openssl::pkey::Public>),
  OpenSSLLow(openssl::pkey::PKey<openssl::pkey::Public>),
  OpenSSLMedium(openssl::pkey::PKey<openssl::pkey::Public>),
  OpenSSLHigh(openssl::pkey::PKey<openssl::pkey::Public>),
  Ed25519(ed25519::PublicKey, ed25519::PublicKey), // First key is encryption key, second is verification key
}

impl PublicKey{
  /// Basically a way to map curves to their OpenSSL curve datatype
  fn get_curve(&self) -> Option<openssl::nid::Nid>{
    Some(match self{
      PublicKey::OpenSSLVeryLow(_) => openssl::nid::Nid::SECT163K1,
      PublicKey::OpenSSLLow(_) => openssl::nid::Nid::SECT233K1,
      PublicKey::OpenSSLMedium(_) => openssl::nid::Nid::SECT409K1,
      PublicKey::OpenSSLHigh(_) => openssl::nid::Nid::SECT571R1,
      _ => return None,
    })
  }

  pub fn size(&self) -> usize {
    match self {
      PublicKey::Ed25519(_,_) => 64,
      PublicKey::OpenSSLHigh(i) => i.size() - 6, // we have no fucking clue why this is needed
      PublicKey::OpenSSLMedium(i) |
      PublicKey::OpenSSLLow(i) |
      PublicKey::OpenSSLVeryLow(i) => i.size()
    }
  }

  pub fn to_vec(&self) -> Option<Vec<u8>>{
    Some(match self{
      PublicKey::Ed25519(key_encryption, key_verification) => {
        //translates to "LibNaCLPK:" which is the (very silly) prefix used by py-ipv8
        let mut res = vec![76, 105, 98, 78, 97, 67, 76, 80, 75, 58];
        res.extend_from_slice(key_encryption.as_ref());
        res.extend_from_slice(key_verification.as_ref());
        res
      },
      PublicKey::OpenSSLHigh(i) |
      PublicKey::OpenSSLMedium(i) |
      PublicKey::OpenSSLLow(i) |
      PublicKey::OpenSSLVeryLow(i) => match i.public_key_to_der(){
        Ok(i) => i,
        Err(_) => return None
      },
      _ => return None,
    })
  }

  pub fn from_vec(data: Vec<u8>) -> Option<Self>{
    // literally "LibNaCLPK:"
    let ed25519prefix = &[76, 105, 98, 78, 97, 67, 76, 80, 75, 58];

    if data.starts_with(ed25519prefix){ // libnacl
      // divide by two to get the cutoff point for the two keys (encryption,verify)
      let key_length = (data.len() - ed25519prefix.len()) / 2;

      let key_encryption = ed25519::PublicKey::from_slice(&data[ed25519prefix.len()..ed25519prefix.len()+key_length])?;
      let key_verification = ed25519::PublicKey::from_slice(&data[ed25519prefix.len()+key_length as usize..])?;

      Some(PublicKey::Ed25519(key_encryption, key_verification))
    }else{

      // openssl DER encoded. Pem always is base64 encoded DER with a header and trailer. py-ipv8 appends these headers and trailers
      // which is very inefficient. We just keep it as DER as that's basically how we get it from the deserialization process.
      let pkey = match openssl::pkey::PKey::public_key_from_der(&*data){
        Ok(i) => i,
        Err(_) => return None
      };

      let eckey = match (*pkey).ec_key(){
        Ok(i) => i,
        Err(_) => return None
      };

      let m = match eckey.group().curve_name()?{
        openssl::nid::Nid::SECT163K1 => PublicKey::OpenSSLVeryLow(pkey),
        openssl::nid::Nid::SECT233K1 => PublicKey::OpenSSLLow(pkey),
        openssl::nid::Nid::SECT409K1 => PublicKey::OpenSSLMedium(pkey),
        openssl::nid::Nid::SECT571R1 => PublicKey::OpenSSLHigh(pkey),
        _ => return None
      };

      dbg!(&m);


      //get the type of key and convert it to a PublicKey enum type
      Some(m)
    }
  }
}

pub enum PrivateKey{
  OpenSSLVeryLow(openssl::pkey::PKey<openssl::pkey::Private>),
  OpenSSLLow(openssl::pkey::PKey<openssl::pkey::Private>),
  OpenSSLMedium(openssl::pkey::PKey<openssl::pkey::Private>),
  OpenSSLHigh(openssl::pkey::PKey<openssl::pkey::Private>),
  OpenSSLVeryHigh(openssl::pkey::PKey<openssl::pkey::Private>),
  Ed25519(ed25519::SecretKey, ed25519::SecretKey), // First key is encryption key, second is verification key
}

impl PartialEq for PublicKey{
  fn eq(&self, other: &Self) -> bool{
    self.to_vec() == other.to_vec()
  }

}

impl fmt::Debug for PrivateKey{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
    write!(f, "PrivateKey <...secret...>")
  }
}


impl fmt::Debug for PublicKey{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
    let t = match self {
      PublicKey::Ed25519(_,_) => "Ed25519",
      PublicKey::OpenSSLHigh(_) =>"OpenSSLHigh",
      PublicKey::OpenSSLMedium(_) =>"OpenSSLMedium",
      PublicKey::OpenSSLLow(_) =>"OpenSSLLow",
      PublicKey::OpenSSLVeryLow(_) =>"OpenSSLVeryLow",
    };
    write!(f, "PublicKey of type {}", t)
  }
}

#[cfg(test)]
mod tests{
  use super::*;

  #[test]
  fn test_from_vec_verylow(){
    let keyvec = vec![48,64,48,16,6,7,42,134,72,206,61,2,1,6,5,43,129,4,0,1,3,44,0,4,0,80,239,172,104,165,76,172,6,229,136,156,105,23,249,46,30,148,87,105,57,6,105,134,2,229,115,169,44,162,41,190,228,56,20,100,64,79,167,224,118,14];
    match PublicKey::from_vec(keyvec.clone()).unwrap(){
      PublicKey::OpenSSLVeryLow(i) => assert_eq!(keyvec,i.public_key_to_der().unwrap()),
      _ => assert!(false)
    }
  }

  #[test]
  fn test_from_vec_low(){
    let keyvec = vec![48,82,48,16,6,7,42,134,72,206,61,2,1,6,5,43,129,4,0,26,3,62,0,4,1,237,162,144,126,249,63,251,228,118,93,65,187,203,79,253,104,206,120,30,139,71,21,181,214,161,144,53,73,148,1,161,113,67,188,223,127,151,153,202,154,76,191,176,244,246,196,92,18,228,141,142,78,103,81,8,19,123,172,213];
    match PublicKey::from_vec(keyvec.clone()).unwrap(){
      PublicKey::OpenSSLLow(i) => assert_eq!(keyvec,i.public_key_to_der().unwrap()),
      _ => assert!(false)
    }
  }

  #[test]
  fn test_from_vec_medium(){
    let keyvec = vec![48,126,48,16,6,7,42,134,72,206,61,2,1,6,5,43,129,4,0,36,3,106,0,4,0,63,154,250,137,139,50,78,67,59,29,230,182,254,215,61,230,33,151,87,122,2,92,194,241,73,104,145,254,241,127,204,168,199,144,240,165,4,223,1,64,22,193,8,200,233,121,45,113,45,147,20,1,72,182,70,130,77,235,216,208,122,244,198,250,247,225,204,19,106,196,129,52,117,172,241,108,179,47,199,124,25,232,26,37,247,95,7,242,128,26,223,86,177,28,165,90,207,116,155,40,182,195,79];
    match PublicKey::from_vec(keyvec.clone()).unwrap(){
      PublicKey::OpenSSLMedium(i) => assert_eq!(keyvec,i.public_key_to_der().unwrap()),
      _ => assert!(false)
    }
  }

  #[test]
  fn test_from_vec_high(){
    let keyvec = vec![48,129,167,48,16,6,7,42,134,72,206,61,2,1,6,5,43,129,4,0,39,3,129,146,0,4,2,86,251,75,206,159,133,120,63,176,235,178,14,8,197,59,107,51,179,139,3,155,20,194,112,113,15,40,67,115,37,223,152,7,102,154,214,90,110,180,226,5,190,99,163,54,116,173,121,40,80,129,142,82,118,154,96,127,164,248,217,91,13,80,91,94,210,16,110,108,41,57,4,243,49,52,194,254,130,98,229,50,84,21,206,134,223,157,189,133,50,210,181,93,229,32,179,228,179,132,143,147,96,207,68,48,184,160,47,227,70,147,23,159,213,105,134,60,211,226,8,235,186,20,241,85,170,4,3,40,183,98,103,80,164,128,87,205,101,67,254,83,142,133];
    match PublicKey::from_vec(keyvec.clone()).unwrap(){
      PublicKey::OpenSSLHigh(i) => assert_eq!(keyvec,i.public_key_to_der().unwrap()),
      _ => assert!(false)
    }
  }

  #[test]
  fn test_from_vec_ed25519(){
    let seed = ed25519::Seed::from_slice(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,]).unwrap();
    let (e_pkey,e_skey) = ed25519::keypair_from_seed(&seed);

    let seed = ed25519::Seed::from_slice(&[1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,]).unwrap();
    let (s_pkey,s_skey) = ed25519::keypair_from_seed(&seed);

    let mut keyvec = vec![76, 105, 98, 78, 97, 67, 76, 80, 75, 58]; // libnaclPK:
    keyvec.extend_from_slice(&e_pkey.as_ref()); // Encryption key
    keyvec.extend_from_slice(&s_pkey.as_ref()); // Verification key

    match PublicKey::from_vec(keyvec).unwrap(){
      PublicKey::Ed25519(key_enc,key_verification) => {
        assert_eq!(key_enc, e_pkey);
        assert_eq!(key_verification,s_pkey);
      },
      _ => assert!(false)
    }
  }

  #[test]
  fn test_to_vec_verylow(){
    let keyvec = vec![48,64,48,16,6,7,42,134,72,206,61,2,1,6,5,43,129,4,0,1,3,44,0,4,0,80,239,172,104,165,76,172,6,229,136,156,105,23,249,46,30,148,87,105,57,6,105,134,2,229,115,169,44,162,41,190,228,56,20,100,64,79,167,224,118,14];
    assert_eq!(PublicKey::from_vec(keyvec.clone()).unwrap().to_vec().unwrap(), keyvec);
  }

  #[test]
  fn test_to_vec_low(){
    let keyvec = vec![48,82,48,16,6,7,42,134,72,206,61,2,1,6,5,43,129,4,0,26,3,62,0,4,1,237,162,144,126,249,63,251,228,118,93,65,187,203,79,253,104,206,120,30,139,71,21,181,214,161,144,53,73,148,1,161,113,67,188,223,127,151,153,202,154,76,191,176,244,246,196,92,18,228,141,142,78,103,81,8,19,123,172,213];
    assert_eq!(PublicKey::from_vec(keyvec.clone()).unwrap().to_vec().unwrap(), keyvec);
  }

  #[test]
  fn test_to_vec_medium(){
    let keyvec = vec![48,126,48,16,6,7,42,134,72,206,61,2,1,6,5,43,129,4,0,36,3,106,0,4,0,63,154,250,137,139,50,78,67,59,29,230,182,254,215,61,230,33,151,87,122,2,92,194,241,73,104,145,254,241,127,204,168,199,144,240,165,4,223,1,64,22,193,8,200,233,121,45,113,45,147,20,1,72,182,70,130,77,235,216,208,122,244,198,250,247,225,204,19,106,196,129,52,117,172,241,108,179,47,199,124,25,232,26,37,247,95,7,242,128,26,223,86,177,28,165,90,207,116,155,40,182,195,79];
    assert_eq!(PublicKey::from_vec(keyvec.clone()).unwrap().to_vec().unwrap(), keyvec);
  }

  #[test]
  fn test_to_vec_high(){
    let keyvec = vec![48,129,167,48,16,6,7,42,134,72,206,61,2,1,6,5,43,129,4,0,39,3,129,146,0,4,2,86,251,75,206,159,133,120,63,176,235,178,14,8,197,59,107,51,179,139,3,155,20,194,112,113,15,40,67,115,37,223,152,7,102,154,214,90,110,180,226,5,190,99,163,54,116,173,121,40,80,129,142,82,118,154,96,127,164,248,217,91,13,80,91,94,210,16,110,108,41,57,4,243,49,52,194,254,130,98,229,50,84,21,206,134,223,157,189,133,50,210,181,93,229,32,179,228,179,132,143,147,96,207,68,48,184,160,47,227,70,147,23,159,213,105,134,60,211,226,8,235,186,20,241,85,170,4,3,40,183,98,103,80,164,128,87,205,101,67,254,83,142,133];
    assert_eq!(PublicKey::from_vec(keyvec.clone()).unwrap().to_vec().unwrap(), keyvec);
  }

  #[test]
  fn test_to_vec_ed25519(){
    let seed = ed25519::Seed::from_slice(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,]).unwrap();
    let (pkey,skey) = ed25519::keypair_from_seed(&seed);
    let mut keyvec = vec![76, 105, 98, 78, 97, 67, 76, 80, 75, 58]; // libnaclPK:
    keyvec.extend_from_slice(&pkey.as_ref());
    keyvec.extend_from_slice(&pkey.as_ref());
    assert_eq!(PublicKey::from_vec(keyvec.clone()).unwrap().to_vec().unwrap(), keyvec);
  }
}
