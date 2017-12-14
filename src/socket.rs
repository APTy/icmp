use std::io;
use std::cmp;
use std::mem;
use std::net::SocketAddr;

use libc;

type FileDesc = libc::c_int;

pub const IPPROTO_ICMP: libc::c_int = 1;

fn cvt<T: IsMinusOne>(t: T) -> io::Result<T> {
    if t.is_minus_one() {
        Err(io::Error::last_os_error())
    } else {
        Ok(t)
    }
}

trait IsMinusOne {
    fn is_minus_one(&self) -> bool;
}

macro_rules! impl_is_minus_one {
    ($($t:ident)*) => ($(impl IsMinusOne for $t {
        fn is_minus_one(&self) -> bool {
            *self == -1
        }
    })*)
}

impl_is_minus_one! { i8 i16 i32 i64 isize }

/// Sends and receives messages over a SOCK_RAW.
pub struct RawSocket(FileDesc);

impl RawSocket {

    /// Returns a new RawSocket for sending and receiving messages on
    /// IPPROTO_ICMP over a SOCK_RAW. Currently only supports IPv4.
    /// TODO: use libc::IPPROTO* when the const becomes available,
    ///       and add some config for IPv4/IPv6.
    pub fn new() -> io::Result<RawSocket> {
        unsafe {
            // try to open with SOCK_CLOEXEC, otherwise use a fallback
            if cfg!(target_os = "linux") {
                match cvt(libc::socket(libc::AF_INET, libc::SOCK_RAW | libc::SOCK_CLOEXEC, IPPROTO_ICMP)) {
                    Ok(fd) => return Ok(RawSocket(fd)),
                    Err(ref e) if e.raw_os_error() == Some(libc::EINVAL) => {}
                    Err(e) => return Err(e),
                }
            }

            let fd = cvt(libc::socket(libc::AF_INET, libc::SOCK_RAW, IPPROTO_ICMP))?;
            cvt(libc::ioctl(fd, libc::FIOCLEX))?;
            let socket = RawSocket(fd);
            Ok(socket)
        }
    }

    /// Returns the socket's underlying file descriptor.
    pub fn fd(&self) -> libc::c_int {
        self.0
    }

    /// Sets the socket to non-blocking mode so that reads return immediately.
    pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        let mut nonblocking = nonblocking as libc::c_int;
        cvt(unsafe { libc::ioctl(self.0, libc::FIONBIO, &mut nonblocking) }).map(|_| ())
    }

    /// Sends a set of bytes over the socket and returns the number of bytes written.
    pub fn send_to(&self, buf: &[u8], dst: &SocketAddr) -> io::Result<usize> {
        let len = cmp::min(buf.len(), <libc::size_t>::max_value() as usize) as libc::size_t;
        let (dstp, dstlen) = into_inner(dst);

        let ret = cvt(unsafe {
            libc::sendto(self.0, buf.as_ptr() as *const libc::c_void, len, 0, dstp, dstlen)
        })?;
        Ok(ret as usize)
    }

    /// Reads the next available packet into the buffer and returns the number
    /// of bytes read. The packet is completely consumed, even if it is only
    /// partially read.
    pub fn recv_from(&self, buf: &mut [u8]) -> io::Result<usize> {
        let mut storage: libc::sockaddr_storage = unsafe { mem::zeroed() };
        let mut addrlen = mem::size_of_val(&storage) as libc::socklen_t;

        let n = cvt(unsafe {
            libc::recvfrom(self.0, buf.as_mut_ptr() as *mut libc::c_void, buf.len(), 0, &mut storage as *mut _ as *mut _, &mut addrlen)
        })?;
        Ok(n as usize)
    }
}

impl Drop for RawSocket {
    fn drop(&mut self) {
        unsafe {
            libc::close(self.0);
        }
    }
}

fn into_inner(s: &SocketAddr) -> (*const libc::sockaddr, libc::socklen_t) {
    match *s {
        SocketAddr::V4(ref a) => {
            (a as *const _ as *const _, mem::size_of_val(a) as libc::socklen_t)
        }
        SocketAddr::V6(ref a) => {
            (a as *const _ as *const _, mem::size_of_val(a) as libc::socklen_t)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read_dir;
    use std::mem::drop;
    use super::RawSocket;

    #[test]
    #[cfg(target_os = "linux")]
    fn it_closes_sockets() {
        let initial_descriptors = read_dir("/proc/self/fd").unwrap().count();

        for _ in 0..5 {
            drop(RawSocket::new().unwrap());
        }

        let final_descriptors = read_dir("/proc/self/fd").unwrap().count();

        assert_eq!(initial_descriptors, final_descriptors);
    }
}
