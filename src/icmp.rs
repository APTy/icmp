use std::cmp;
use std::net::{SocketAddr, ToSocketAddrs};
use std::time::{Instant, Duration};
use std::collections::HashMap;

use rand;
use libc::c_int;
use nix::poll::{poll, EventFlags, PollFd, POLLIN};

use socket::RawSocket;
use packet::EchoRequest;

pub const POLL_TIMEOUT_MS: c_int = 1000;

/// An object for sending and receiving ICMP messages.
pub struct Icmp {
    socket: RawSocket,
    host: SocketAddr,
    echo: EchoRequest,
    echoes: HashMap<u16, Instant>,
}

impl Icmp {

    /// Creates a new object for sending and receiving ICMP messages between
    /// the local machine and a host machine identified by `ip`.
    pub fn new(ip: &str) -> Icmp {
        let host = (ip, 0).to_socket_addrs().unwrap().next().unwrap();
        let socket = RawSocket::new().unwrap();
        socket.set_nonblocking(true).unwrap();
        Icmp {
            socket: socket,
            host: host,
            echo: EchoRequest::new(rand::random()),
            echoes: HashMap::new(),
        }
    }

    /// Polls for new messages. Returns true if data becomes available before
    /// the timeout expires.
    pub fn poll(&self) -> Option<bool> {
        let icmpfd = PollFd::new(self.socket.fd(), POLLIN, EventFlags::empty());
        let rdy = poll(& mut[icmpfd], POLL_TIMEOUT_MS).unwrap();
        Some(rdy >= 0)
    }

    /// Send an echo request with an incrementing sequence number.
    pub fn echo_request(&mut self) {
        self.echo.inc_seq();
        self.echo.do_checksum();
        self.socket.send_to(&self.echo.as_bytes(), &self.host).unwrap();
        self.echoes.insert(self.echo.seq(), Instant::now());
    }

    /// Issue a non-blocking read for any available echo replies from the host.
    /// Returns an object representing the reply and the time elapsed between
    /// sending the request and receiving its reply.
    pub fn get_echo_reply(&self) -> Option<(EchoRequest, Duration)> {
        let mut buf = [0; 65536];
        match self.socket.recv_from(&mut buf) {
            Err(_) => None,
            Ok(recv_len) => {
                let len = cmp::min(recv_len, buf.len());
                let echo = EchoRequest::from(&buf[20..len]);
                // verify the echo reply is one of ours
                if echo.id() != self.echo.id() {
                    return None;
                }
                let time = Instant::now().duration_since(*self.echoes.get(&echo.seq()).unwrap());
                Some((echo, time))
            },
        }
    }
}
