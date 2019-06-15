use crate::serialization::Packet;
use std::error::Error;
use mio::net::UdpSocket;
use rayon::{ThreadPool, ThreadPoolBuilder};
use std::thread;
use std::thread::JoinHandle;
use mio::{Poll, Token, Events, Ready, PollOpt};
use crate::configuration::Config;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::time::Duration;
use crate::networking::address::Address;

pub mod address;

create_error!(SocketCreationError, "The socket creation failed");
create_error!(ListenError, "An error occured during the listening");

/// Any struct implementing this method can become a receiver of incoming network packets.
/// under normal operation, only the IPV8 struct should be a receiver of these and it should distribute it
/// through its CommunityRegistry to communities
pub trait Receiver {
    fn on_receive(&self, packet: Packet, address: Address);
}

/// This struct manages the sockets and receives incoming messages.
pub struct NetworkManager {
    receivers: Vec<Box<dyn Receiver + Send + Sync>>,
    receiving_socket: UdpSocket,
    sending_socket: UdpSocket,
    threadpool: ThreadPool,
}

impl NetworkManager {
    /// Creates a new networkmanager. This creates a receiver socket and builds a new threadpool on which
    /// all messages are distributed.
    pub fn new(
        sending_address: &Address,
        receiving_address: &Address,
        threadcount: usize,
    ) -> Result<Self, Box<dyn Error>> {
        let receiving_socket =
            UdpSocket::bind(&receiving_address.0).or(Err(SocketCreationError))?;

        let sending_socket = UdpSocket::bind(&sending_address.0).or(Err(SocketCreationError))?;

        trace!(
            "Starting, sending_address: {:?}, receiving_address: {:?}",
            sending_address,
            receiving_address
        );

        let pool = ThreadPoolBuilder::new().num_threads(threadcount).build()?;

        let nm = Self {
            threadpool: pool,
            receivers: vec![],
            receiving_socket,
            sending_socket,
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

        thread::spawn(move || {
            self.listen(queuesize, buffersize, pollinterval)
                .or_else(|i| {
                    error!("the listening thread crashed");
                    Err(i)
                })
                .unwrap(); // :gasp: <-- here it's allowed as it will only crash this thread
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
        // this is basically a generated magic number we can later check for
        const RECEIVER: Token = Token(0);
        let mut events = Events::with_capacity(queuesize);

        let mut tmp_buf = vec![0; buffersize];
        let buffer = tmp_buf.as_mut_slice();

        poll.register(
            &self.receiving_socket,
            RECEIVER,
            Ready::readable(),
            PollOpt::edge(),
        )?;

        loop {
            poll.poll(&mut events, pollinterval)?;
            trace!("checking poll");
            for _ in events.iter() {
                debug!("handling event");

                let (recv_size, address) = self.receiving_socket.recv_from(buffer)?;

                let packet = Packet(buffer[..recv_size].to_vec()).clone();

                // use our own threadpool
                self.threadpool.scope_fifo(|s| {
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

    /// Sends a Packet to the specified address.
    pub fn send(&self, address: &Address, packet: Packet) -> Result<(), Box<dyn Error>> {
        self.sending_socket.send_to(packet.raw(), &address.0)?;
        Ok(())
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
    use crate::networking::{Receiver, NetworkManager};
    use std::thread;
    use std::sync::atomic::{AtomicUsize, Ordering, AtomicU16};
    use crate::networking::address::Address;

    static BEFORE: Once = Once::new();

    // A poor man's @Before
    fn before() {
        BEFORE.call_once(|| {
            simple_logger::init().unwrap();
        })
    }

    #[test]
    fn test_socket_creation_error() {
        let address = Ipv4Addr::new(127, 0, 0, 1);

        let addr1 = Address(SocketAddr::new(IpAddr::V4(address), 0));
        let receiving_socket = UdpSocket::bind(&addr1.0).unwrap();
        let addr1 = Address(SocketAddr::new(
            IpAddr::V4(address),
            receiving_socket.local_addr().unwrap().port(),
        ));
        let addr2 = Address(SocketAddr::new(IpAddr::V4(address), 0));
        let addr3 = Address(SocketAddr::new(IpAddr::V4(address), 0));
        let addr4 = Address(SocketAddr::new(IpAddr::V4(address), 0));

        // should report an error as the address is already in use (for the sending socket)
        match NetworkManager::new(&addr1, &addr2, 0) {
            Err(_) => assert!(true),
            Ok(_) => assert!(false),
        };
        // now it shouldnt have made a receiving socket so making a new sending socket with that address should work
        match NetworkManager::new(&addr2, &addr3, 0) {
            Err(_) => assert!(false),
            Ok(_) => assert!(true),
        };
        //or when using a sending socket that was already in use
        match NetworkManager::new(&addr4, &addr1, 0) {
            Err(_) => assert!(true),
            Ok(_) => assert!(false),
        };
    }

    // `pacman -Syu networkmanager`
    #[test]
    fn test_networkmanager() {
        before();

        // start ipv8
        let mut config = Config::default();
        let address = Ipv4Addr::new(0, 0, 0, 0);

        config.receiving_address = Address(SocketAddr::new(IpAddr::V4(address), 8090));
        config.sending_address = Address(SocketAddr::new(IpAddr::V4(address), 0));
        config.buffersize = 2048;

        let mut ipv8 = IPv8::new(config).unwrap();

        let sender_socket = UdpSocket::bind(&SocketAddr::new(IpAddr::V4(address), 0)).unwrap();

        static SEND_PORT: AtomicU16 = AtomicU16::new(0);

        let recv_port: u16 = ipv8
            .networkmanager
            .receiving_socket
            .local_addr()
            .unwrap()
            .port();
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

        ipv8.networkmanager.add_receiver(Box::new(AReceiver));

        ipv8.start();
        // wait for it to start up
        thread::sleep(Duration::from_millis(300));

        // now try to send ipv8 a message
        sender_socket
            .connect(SocketAddr::new(IpAddr::V4(address), recv_port))
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
}
