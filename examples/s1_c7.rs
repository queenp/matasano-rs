extern crate base64;
extern crate crypto;
extern crate matasano;

use std::fs::File;
use std::io::Read;

use matasano::ops::{aes_d};

fn main() {
    let mut x = File::open("examples/7.txt").expect("Challenge file not found!");
    let mut bytes = Vec::new();
    x.read_to_end(&mut bytes);
    let decoded = base64::decode_config(&bytes, base64::MIME).expect("Invalid base64");
    let blocks = decoded.chunks(16);
    let key = b"YELLOW SUBMARINE";
    let result = blocks.fold(Vec::<u8>::new(),
        |mut acc, block| {
            let out = aes_d(key, &block);
            acc.extend(&out);
            acc});
    println!("{}", String::from_utf8(result).expect("Not valid utf-8"));
}
