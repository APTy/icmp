use std::io::Cursor;
use byteorder::{NetworkEndian, ReadBytesExt, WriteBytesExt};

use packet::checksum::compute_checksum;

pub static HEADER_SIZE: usize = 8;

pub type Type = u8;

pub static ECHO_REPLY_TYPE: Type = 0;
pub static ECHO_REQUEST_TYPE: Type = 8;

pub type TypeCode = u8;

pub static ECHO_REPLY_CODE: TypeCode = 0u8;
pub static ECHO_REQUEST_CODE: TypeCode = 0u8;

/// The 8-byte header for an ICMP packet
#[derive(Debug)]
pub struct Header {
    ttype: Type,
    code: TypeCode,
    checksum: u16,
    data: u32,
}

impl Header {
    /// Creates a new Header using a type, code, and arbitrary
    /// header data. A checksum is automatically calculated.
    pub fn new(ttype: Type, code: TypeCode, data: u32) -> Header {
        Header {
            ttype: ttype,
            code: code,
            checksum: 0,
            data: data,
        }
    }

    /// Creates a new Header using a slice of bytes. This method will
    /// use the first 8 bytes from the slice, and ignore the rest.
    pub fn from(b: &[u8]) -> Header {
        let mut r = Cursor::new(b);
        Header {
            ttype: r.read_u8().unwrap() as Type,
            code: r.read_u8().unwrap() as TypeCode,
            checksum: r.read_u16::<NetworkEndian>().unwrap(),
            data: r.read_u32::<NetworkEndian>().unwrap(),
        }
    }

    /// Calculate the checksum for the header.
    pub fn do_checksum(&mut self) {
        // set the current checksum to 0
        self.checksum = 0u16;
        self.checksum = compute_checksum(&self.as_bytes());
    }

    /// Returns the header represented as a sized array of bytes.
    pub fn as_bytes(&self) -> [u8; 8] {
        let mut w = vec![];
        w.write_u8(self.ttype).unwrap();
        w.write_u8(self.code).unwrap();
        w.write_u16::<NetworkEndian>(self.checksum).unwrap();
        w.write_u32::<NetworkEndian>(self.data).unwrap();

        let mut header = [0u8; 8];
        for i in 0..header.len() {
            header[i] = w[i];
        }
        header
    }

    /// Sets the 4-byte header data as a u32.
    pub fn set_data(&mut self, data: u32) {
        self.data = data
    }

    /// Gets the 4-byte header data as a u32.
    pub fn get_data(&self) -> u32 {
        self.data
    }
}
