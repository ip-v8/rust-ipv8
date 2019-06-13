use std::net::{SocketAddr, IpAddr};
use crate::networking::address::Address;
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

pub mod address;

create_error!(SocketCreationError, "The socket creation failed");
create_error!(ListenError, "An error occured during the listening");

pub trait Receiver {
    fn on_receive(&self, packet: Packet, address: Address);
}

pub struct NetworkManager {
    receivers: Vec<Box<dyn Receiver + Send + Sync>>,
    socket: UdpSocket,
    threadpool: ThreadPool,
}

impl NetworkManager {
    pub fn new(address: &Address, threadcount: usize) -> Result<Self, Box<dyn Error>> {
        let socket = UdpSocket::bind(&SocketAddr::new(IpAddr::V4(address.address), address.port))
            .or(Err(SocketCreationError))?;

        trace!("Starting on {}", address);

        let pool = ThreadPoolBuilder::new()
            .num_threads(threadcount)
            .breadth_first()
            .build()?;

        let nm = Self {
            threadpool: pool,
            receivers: vec![],
            socket,
        };
        Ok(nm)
    }

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

    pub fn listen(
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

        poll.register(&self.socket, RECEIVER, Ready::readable(), PollOpt::edge())?;

        loop {
            poll.poll(&mut events, pollinterval)?;
            trace!("checking poll");
            for _ in events.iter() {
                debug!("handling event");

                let (recv_size, address) = self.socket.recv_from(buffer)?;

                let packet = Packet(buffer[..recv_size].to_vec()).clone();

                let ip = match address.ip() {
                    IpAddr::V4(a) => a,
                    IpAddr::V6(_) => {
                        warn!("Unexpectedly received ipv6 packet");
                        continue;
                    }
                };

                // use our own threadpool
                self.threadpool.install(|| {
                    // iterate over the receivers asynchronously and non blocking
                    self.receivers.par_iter().for_each(|r| {
                        r.on_receive(
                            packet.clone(),
                            Address {
                                address: ip.to_owned(),
                                port: address.port(),
                            },
                        );
                    });
                });
            }
        }
    }

    pub fn add_receiver(&mut self, receiver: Box<dyn Receiver + Send + Sync>) {
        self.receivers.push(receiver)
    }

    pub fn send(address: Address, packet: Packet) -> Result<(), Box<dyn Error>> {
        unimplemented!(
            "Trying to send {:?} to {:?} but sending is not implemented",
            packet,
            address
        )
    }
}

#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;

    use crate::IPv8;
    use crate::configuration::Config;
    use mio::net::UdpSocket;
    use crate::networking::address::Address;
    use std::net::{Ipv4Addr, SocketAddr, IpAddr};
    use crate::serialization::Packet;
    use std::sync::Once;
    use std::time::Duration;
    use crate::networking::Receiver;
    use std::thread;
    use std::sync::atomic::{AtomicUsize, Ordering, AtomicU16};

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
        let address = Ipv4Addr::new(0, 0, 0, 0);

        config.socketaddress = Address { address, port: 0 };
        config.buffersize = 2048;

        let mut ipv8 = IPv8::new(config).unwrap();

        let sender_socket = UdpSocket::bind(&SocketAddr::new(IpAddr::V4(address), 0)).unwrap();

        static SEND_PORT: AtomicU16 = AtomicU16::new(0);

        let recv_port: u16 = ipv8.networkmanager.socket.local_addr().unwrap().port();
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
                assert_eq!(SEND_PORT.load(Ordering::SeqCst), address.port);

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
