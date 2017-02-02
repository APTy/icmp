use std::io::Cursor;
use byteorder::{NetworkEndian, ReadBytesExt, WriteBytesExt};

use packet::header::{Header, HEADER_SIZE, ECHO_REQUEST_TYPE, ECHO_REQUEST_CODE};

#[derive(Debug)]
pub struct EchoRequest {
    header: Header,
    data: Vec<u8>,
}

impl EchoRequest {

    fn create_u32_data(id: u16, seq: u16) -> u32 {
        let mut w = vec![];
        w.write_u16::<NetworkEndian>(id).unwrap();
        w.write_u16::<NetworkEndian>(seq).unwrap();
        let mut r = Cursor::new(w);
        r.read_u32::<NetworkEndian>().unwrap()
    }

    fn read_u32_data(d: u32) -> (u16, u16) {
        let mut w = vec![];
        w.write_u32::<NetworkEndian>(d).unwrap();
        let mut r = Cursor::new(w);
        (r.read_u16::<NetworkEndian>().unwrap(), r.read_u16::<NetworkEndian>().unwrap())
    }

    /// Creates a new EchoRequest using the provided unique `id`.
    pub fn new(id: u16) -> EchoRequest {
        let data = EchoRequest::create_u32_data(id, 0);
        EchoRequest {
            header: Header::new(ECHO_REQUEST_TYPE, ECHO_REQUEST_CODE, data),
            data: Vec::new(),
        }
    }

    /// Creates a new EchoRequest from a byte slice.
    pub fn from(b: &[u8]) -> EchoRequest {
        EchoRequest {
            header: Header::from(&b[..HEADER_SIZE]),
            data: b[HEADER_SIZE..].to_vec(),
        }
    }

    /// Returns the total length of the packet, including the header.
    pub fn len(&self) -> usize {
        HEADER_SIZE + self.data.len()
    }

    /// Returns the unique id of the echo request.
    pub fn id(&self) -> u16 {
        let (id, _) = EchoRequest::read_u32_data(self.header.get_data());
        id
    }

    /// Returns the current sequence number of the echo request.
    pub fn seq(&self) -> u16 {
        let (_, seq) = EchoRequest::read_u32_data(self.header.get_data());
        seq
    }

    /// Returns the packet represented as a slice of bytes.
    pub fn as_bytes(&self) -> [u8; 8] {
        self.header.as_bytes()
    }

    /// Increment the packet's sequence number.
    pub fn inc_seq(&mut self) {
        let (id, seq) = EchoRequest::read_u32_data(self.header.get_data());
        self.header.set_data(EchoRequest::create_u32_data(id, seq + 1));
    }

    /// Calculates the packet's checksum and updates the header.
    pub fn do_checksum(&mut self) {
        self.header.do_checksum();
    }
}
