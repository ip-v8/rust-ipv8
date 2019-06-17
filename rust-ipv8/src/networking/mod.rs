use crate::serialization::Packet;
use std::error::Error;
use mio::net::UdpSocket;
use std::thread;
use std::thread::JoinHandle;
use mio::{Poll, Token, Events, Ready, PollOpt};
use crate::configuration::Config;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::time::Duration;
use crate::networking::address::Address;
use rayon::scope_fifo;

pub mod address;

create_error!(SocketCreationError, "The socket creation failed");
create_error!(ListenError, "An error occured during the listening");

/// Any struct implementing this method can become a receiver of incoming network packets.
/// under normal operation, only the IPV8 struct should be a receiver of these and it should distribute it
/// through its CommunityRegistry to communities
pub trait Receiver {
    fn on_receive(&self, packet: Packet, address: Address);
}

pub struct NetworkSender {
    socket: UdpSocket,
}

impl NetworkSender {
    pub fn new(sending_address: &Address) -> Result<Self, Box<dyn Error>> {
        let socket = UdpSocket::bind(&sending_address.0)?;
        debug!("Starting, sending_address: {:?}", sending_address);

        Ok(Self { socket })
    }

    /// Sends a Packet to the specified address.
    pub fn send(&self, address: &Address, packet: Packet) -> Result<usize, Box<dyn Error>> {
        Ok(self.socket.send_to(packet.raw(), &address.0)?)
    }
}

pub struct NetworkReceiver {
    receivers: Vec<Box<dyn Receiver + Send + Sync>>,
    socket: UdpSocket,
}

impl NetworkReceiver {
    /// Creates a new networkmanager. This creates a receiver socket and builds a new threadpool on which
    /// all messages are distributed.
    pub fn new(receiving_address: &Address) -> Result<Self, Box<dyn Error>> {
        let socket = UdpSocket::bind(&receiving_address.0)?;

        debug!("Starting, receiving_address: {:?}", receiving_address);

        let nm = Self {
            receivers: vec![],
            socket,
        };
        Ok(nm)
    }

    /// Starts the networkmanager. This spawns a new thread in which it will listen for incoming messages.
    ///
    /// This method consumes self as it is transferred to the new thread. After this no receievers can be added to it.
    ///
    /// Returns a `JoinHandle<()>` which can be used to block until the networkmanager stops listening.
    /// Under normal operation this never happens so this marks the end of the program.
    pub fn start(self, configuration: &Config) -> JoinHandle<()> {
        let queuesize = configuration.queuesize.to_owned();
        let buffersize = configuration.buffersize.to_owned();
        let pollinterval = configuration.pollinterval.to_owned();

        // Start the I/O thread
        thread::spawn(move || {
            self.listen(queuesize, buffersize, pollinterval)
                .or_else(|i| {
                    error!("the listening thread crashed");
                    Err(i)
                })
                .unwrap(); // This only panics the I/O thread not the whole application
        })
    }

    fn listen(
        self,
        queuesize: usize,
        buffersize: usize,
        pollinterval: Option<Duration>,
    ) -> Result<(), Box<dyn Error>> {
        debug!("IPV8 is starting it's listener!");

        let poll = Poll::new()?;

        let mut events = Events::with_capacity(queuesize);

        let mut tmp_buf = vec![0; buffersize];
        let buffer = tmp_buf.as_mut_slice();

        const RECEIVER: Token = Token(0);
        poll.register(&self.socket, RECEIVER, Ready::readable(), PollOpt::edge())?;

        loop {
            poll.poll(&mut events, pollinterval)?;
            trace!("checking poll");
            for _ in events.iter() {
                trace!("handling event");

                let (recv_size, address) = self.socket.recv_from(buffer)?;

                let packet = Packet(buffer[..recv_size].to_vec()).clone();

                // We want a FIFO threadpool
                scope_fifo(|s| {
                    s.spawn_fifo(|_| {
                        // iterate over the receivers asynchronously and non blocking
                        self.receivers.par_iter().for_each(|r| {
                            r.on_receive(packet.clone(), Address(address));
                        });
                    })
                });
            }
        }
    }

    /// Adds a receiver to the networkmanager. Can only happen before the networkmanager is started.
    pub fn add_receiver(&mut self, receiver: Box<dyn Receiver + Send + Sync>) {
        self.receivers.push(receiver)
    }
}

/// Taken and adapted from the [mio testing suite](https://github.com/tokio-rs/mio/blob/master/test/mod.rs#L113)
#[cfg(test)]
pub mod test_helper {
    use std::net::{SocketAddr, Ipv4Addr, SocketAddrV4, IpAddr};
    use std::str::FromStr;
    use std::sync::atomic::Ordering::SeqCst;
    use std::sync::atomic::{AtomicU16};
    use crate::networking::address::Address;

    // Helper for getting a unique port for the task run
    // TODO: Reuse ports to not spam the system
    // If multiple test suites are ran at the same time, this _will_ fail as it will try to bind to the same FIRST_PORT
    const FIRST_PORT: u16 = 18080;
    static NEXT_PORT: AtomicU16 = AtomicU16::new(FIRST_PORT);
    pub const LOCALHOST_IP: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);

    fn next_port() -> u16 {
        // Get and increment the port list
        NEXT_PORT.fetch_add(1, SeqCst)
    }

    pub fn localhost() -> Address {
        Address(localhost_socket())
    }

    pub fn localhost_socket() -> SocketAddr {
        SocketAddr::new(IpAddr::V4(LOCALHOST_IP), next_port())
    }
}

#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;

    use crate::IPv8;
    use crate::configuration::Config;
    use mio::net::UdpSocket;

    use std::net::{Ipv4Addr, SocketAddr, IpAddr};
    use crate::serialization::Packet;
    use std::sync::Once;
    use std::time::Duration;
    use crate::networking::{Receiver, NetworkSender, NetworkReceiver};
    use std::thread;
    use std::sync::atomic::{AtomicUsize, Ordering, AtomicU16};
    use crate::networking::address::Address;
    use crate::serialization::header::HeaderVersion::PyIPV8Header;
    use crate::serialization::header::Header;
    use serde::private::ser::constrain;
    use crate::networking::test_helper::{localhost, localhost_socket, LOCALHOST_IP};

    static BEFORE: Once = Once::new();

    // A poor man's @Before
    fn before() {
        BEFORE.call_once(|| {
            simple_logger::init().unwrap();
        })
    }

    // `pacman -Syu networkmanager`
    #[test]
    fn test_networkmanager() {
        before();

        // start ipv8
        let mut config = Config::default();

        config.receiving_address = localhost();
        config.sending_address = localhost();
        config.buffersize = 2048;

        let mut ipv8 = IPv8::new(config).unwrap();

        let sender_socket = UdpSocket::bind(&localhost_socket()).unwrap();

        static SEND_PORT: AtomicU16 = AtomicU16::new(0);

        let recv_port: u16 = ipv8.network_receiver.socket.local_addr().unwrap().port();
        let send_port: u16 = sender_socket.local_addr().unwrap().port();

        SEND_PORT.store(send_port, Ordering::SeqCst);

        lazy_static! {
            static ref OGPACKET: Packet = Packet::new(create_test_header!()).unwrap();
        }

        static PACKET_COUNTER: AtomicUsize = AtomicUsize::new(0);

        //create receiver
        struct AReceiver;
        impl Receiver for AReceiver {
            fn on_receive(&self, packet: Packet, address: Address) {
                assert_eq!(OGPACKET.raw(), packet.raw());
                assert_eq!(SEND_PORT.load(Ordering::SeqCst), (address.0).port());

                // Count each packet
                PACKET_COUNTER.fetch_add(1, Ordering::SeqCst);
            }
        }

        ipv8.network_receiver.add_receiver(Box::new(AReceiver));

        ipv8.start();

        // now try to send ipv8 a message
        sender_socket
            .connect(SocketAddr::new(IpAddr::V4(LOCALHOST_IP), recv_port))
            .unwrap();

        let a = sender_socket.send(OGPACKET.raw()).unwrap();
        assert_eq!(a, OGPACKET.raw().len());

        thread::sleep(Duration::from_millis(20));

        let b = sender_socket.send(OGPACKET.raw()).unwrap();
        assert_eq!(b, OGPACKET.raw().len());

        thread::sleep(Duration::from_millis(20));

        // a poor man's `verify(AReceiver, times(2)).on_receiver();`
        assert_eq!(2, PACKET_COUNTER.load(std::sync::atomic::Ordering::SeqCst));
    }

    #[test]
    fn test_sending_networkmanager() {
        before();

        // start ipv8
        let mut config = Config::default();

        config.receiving_address = localhost();
        config.sending_address = localhost();
        config.buffersize = 2048;

        // let mut ipv8 = IPv8::new(config).unwrap();
        let ns = NetworkSender::new(&config.sending_address).unwrap();
        let mut nr = NetworkReceiver::new(&config.receiving_address).unwrap();

        static SEND_PORT: AtomicU16 = AtomicU16::new(0);

        let recv_port: u16 = nr.socket.local_addr().unwrap().port();
        let send_port: u16 = ns.socket.local_addr().unwrap().port();

        SEND_PORT.store(send_port, Ordering::SeqCst);

        lazy_static! {
            static ref OGPACKET: Packet = Packet::new(create_test_header!()).unwrap();
        }

        static PACKET_COUNTER: AtomicUsize = AtomicUsize::new(0);

        //create receiver
        struct AReceiver;
        impl Receiver for AReceiver {
            fn on_receive(&self, packet: Packet, address: Address) {
                assert_eq!(OGPACKET.raw(), packet.raw());
                assert_eq!(SEND_PORT.load(Ordering::SeqCst), (address.0).port());

                // Count each packet
                PACKET_COUNTER.fetch_add(1, Ordering::SeqCst);
            }
        }

        nr.add_receiver(Box::new(AReceiver));
        nr.start(&config);

        let addr = Address(SocketAddr::new(IpAddr::V4(LOCALHOST_IP), recv_port));

        ns.send(&addr, Packet(OGPACKET.raw().to_vec())).unwrap();

        thread::sleep(Duration::from_millis(20));

        // a poor man's `verify(AReceiver, times(2)).on_receiver();`
        assert_eq!(1, PACKET_COUNTER.load(std::sync::atomic::Ordering::SeqCst));
    }
}
