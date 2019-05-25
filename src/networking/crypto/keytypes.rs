
use openssl;
use rust_sodium::crypto::sign::ed25519;

pub fn get_signature_length(curve : openssl::nid::Nid) -> Option<u16>{

  let i = match curve{
    openssl::nid::Nid::SECT163K1 => 163u16,
    openssl::nid::Nid::SECT233K1 => 233u16,
    openssl::nid::Nid::SECT409K1 => 409u16,
    openssl::nid::Nid::SECT571R1 => 570u16,
    _ => return None
  };
  Some((i as f32 / 8.0).ceil() as u16 * 2)
}

pub enum PublicKey{
  OpenSSLVeryLow(openssl::ec::EcKey<openssl::pkey::Public>),
  OpenSSLLow(openssl::ec::EcKey<openssl::pkey::Public>),
  OpenSSLMedium(openssl::ec::EcKey<openssl::pkey::Public>),
  OpenSSLHigh(openssl::ec::EcKey<openssl::pkey::Public>),
  Ed25519(ed25519::PublicKey),
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
}

pub enum PrivateKey{
  OpenSSLVeryLow(openssl::ec::EcKey<openssl::pkey::Private>),
  OpenSSLLow(openssl::ec::EcKey<openssl::pkey::Private>),
  OpenSSLMedium(openssl::ec::EcKey<openssl::pkey::Private>),
  OpenSSLHigh(openssl::ec::EcKey<openssl::pkey::Private>),
  OpenSSLVeryHigh(openssl::ec::EcKey<openssl::pkey::Private>),
  Ed25519(ed25519::SecretKey),
}
