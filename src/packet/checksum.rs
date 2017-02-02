use std::io::Cursor;
use byteorder::{NetworkEndian, ReadBytesExt};

/// Compute the Internet Checksum for a slice of bytes
pub fn compute_checksum(data: &[u8]) -> u16 {
    let mut r = Cursor::new(data);
    let mut b = vec![];
    while let Ok(u) = r.read_u16::<NetworkEndian>() {
        b.push(u);
    }
    compute_checksum_u16(b.as_slice())
}

/// Compute the Internet Checksum for a slice of u16s
pub fn compute_checksum_u16(data: &[u16]) -> u16 {
    let mut sum = 0u32;
    for d in data {
        sum += *d as u32;
    }

    // Fold 32-bit sum to 16 bits
    while (sum >> 16) > 0 {
        sum = (sum & 0xffff) + (sum >> 16);
    }
    !sum as u16
}

#[test]
fn test_compute_checksum() {
    let data: [u16; 2] = [0x0000, 0xffff];
    let sum = 0x0000;
    let checksum = compute_checksum_u16(&data);
    assert_eq!(sum, checksum);
}

#[test]
fn test_compute_checksum_1() {
    let data: [u16; 10] = [
        0x4500, 0x0073, 0x0000, 0x4000, 0x4011,
        0xb861, 0xc0a8, 0x0001, 0xc0a8, 0x00c7,
    ];
    let sum = 0x0000;
    let checksum = compute_checksum_u16(&data);
    assert_eq!(sum, checksum);
}
