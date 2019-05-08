use super::packet::Packet;

pub trait Ipv8Payload {
  fn pack(&self) -> Packet;
  fn unpack(packet: Packet) -> Self;
}

