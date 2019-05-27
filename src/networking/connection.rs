use super::{super::event::EventGenerator, address::Address};
use std::net::{SocketAddr, UdpSocket};

// NOTE: i am really unhappy with how this connection class works as of now.
// please improve

pub struct Connection {
  socket: UdpSocket,

  on_message: EventGenerator,
}

impl Connection {
  fn new(address: Address) -> Result<Self, String> {
    let socketaddress = SocketAddr::from((address.address, address.port));
    let socket = match UdpSocket::bind(socketaddress) {
      Ok(i) => i,
      Err(i) => return Err(format!("{:?}", i)),
    };

    Ok(Connection {
      socket,
      on_message: EventGenerator::new(),
    })
  }

  /**
   * Send some data to an address.
   * @param _address The address to send to.
   * @param _data A vector with data to send.
   */
  fn send(_address: Address, _data: Vec<u8>) {}
}

