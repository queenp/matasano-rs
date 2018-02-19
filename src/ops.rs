use std::iter::Iterator;

use crypto;
use crypto::buffer;
use crypto::buffer::{ReadBuffer, WriteBuffer, BufferResult};
use crypto::aes;

/// String Xor operator.
///
/// Note this operation is not transitive. For ergonomic reasons, the RHS is presumed to be shorter
/// than the LHS, and is cycled, fixed only by the length of the LHS (acting like a keystream).
#[inline]
pub fn sxor(lhs: &str, rhs: &str) -> Vec<u8> {
    lhs.bytes().zip(rhs.bytes().cycle())
        .map(|(a,b)| a^b)
        .collect()
}

/// Byte Xor into lhs variable.
pub fn bxor_in_place(lhs: &mut [u8], rhs: &[u8]) {
    for (a,b) in  lhs.iter_mut().zip(rhs.iter().cycle()) {
        *a ^= b;
    }
}

/// Byte xor operator.
///
/// Not transitive. LHS determines length, rhs is cycled.
#[inline]
pub fn bxor(lhs: &[u8], rhs: &[u8]) -> Vec<u8> {
    lhs.iter().zip(rhs.iter().cycle())
        .map(|(a,b)| a^b)
        .collect()
}

/// Byte xor byte iterator
///
/// Not necessarily the best implementation returning a `Vec`, would be nice to return an
/// `impl Iterator<Item=u8>`
//pub fn bxor_iter<T, W>(lhs: &T, rhs: &[u8]) -> Vec<u8> 
//where T: Iterator<Item=u8> {
//    lhs.zip(rhs.iter().cycle()).map(|(a,b)| a^b).collect::<Vec<u8>>()
//}

/// Count all the ones in a given string of bytes.
#[inline]
pub fn hamming(some: &[u8]) -> u32 {
    some.iter()
        .fold(0,|mut acc,i| {
            acc += i.count_ones(); 
            acc})
}

pub fn hamming_d(lhs: &[u8], rhs: &[u8]) -> u32 {
    hamming(&bxor(lhs, rhs))
}

/// Horrifically extracting the block cipher from rust-crypto and removing the nice stream handling
/// features for the challenge
pub fn aes_e(key: &[u8], pt: &[u8]) -> Vec<u8> {
    let mut e = aes::ecb_encryptor(aes::KeySize::KeySize128, key, crypto::blockmodes::NoPadding);
    let mut out = Vec::<u8>::new();
    let mut buf = [0u8;16];
    let mut reader = buffer::RefReadBuffer::new(pt);
    let mut writer = buffer::RefWriteBuffer::new(&mut buf);

    e.encrypt(&mut reader, &mut writer, true).expect("Failed to encrypt");
    out.extend(writer.take_read_buffer().take_remaining());
    out
}

/// Doing horrific unrusty stuff to avoid reimplementing dumb AES.
pub fn aes_d(key: &[u8], ct: &[u8]) -> Vec<u8> {
    let mut d = aes::ecb_decryptor(aes::KeySize::KeySize128, key, crypto::blockmodes::NoPadding);
    let mut out = Vec::<u8>::new();
    let mut buf = [0u8;16];
    let mut reader = buffer::RefReadBuffer::new(ct);
    let mut writer = buffer::RefWriteBuffer::new(&mut buf);

    d.decrypt(&mut reader, &mut writer, true).expect("Failed to decrypt");
    out.extend(writer.take_read_buffer().take_remaining());
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hamming_dist() {
        let s1 = b"this is a test";
        let s2 = b"wokka wokka!!!";

        assert_eq!(hamming_d(s1,s2),37);
    }
}
