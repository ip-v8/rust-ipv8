use super::packet::Packet;





pub trait Ipv8Payload {

  // // fn serialize(&self) -> Vec<PackingIdentifier>;
  // fn deserialize(data:Vec<PackingIdentifier>) -> Self;


  fn pack(&self) -> Packet;
  fn unpack(packet:Packet) -> Self;

}

