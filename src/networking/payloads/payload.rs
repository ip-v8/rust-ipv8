use crate::networking::serialization::rawend::RawEnd;

pub trait Ipv8Payload {
  fn set_rawend(&mut self, _bytes: RawEnd){}
}

