extern crate base64;
extern crate matasano;

use std::fs::File;
use std::io::prelude::*;
use std::io::{Error,ErrorKind};

use matasano::cracks::{find_keysize, find_vig_xor_key};
use matasano::ops::bxor;

fn load_b64_ct() -> Result<Vec<u8>,Error> {
    let mut f = File::open("examples/6.txt")?;
    let mut b = Vec::new();
    f.read_to_end(&mut b);

    if let Ok(r) = base64::decode_config(&b,base64::MIME) {
        Ok(r)
    } else {
        Err(Error::new(ErrorKind::Other, "Failed to decode"))
    }
}

fn main() {
    let ct = load_b64_ct().expect("Failed to load ciphertext!");
    
    let k_sz = find_keysize(&ct, 2..40);

    assert_eq!(k_sz[0].0, 29); // Checking nothing has changed behaviourally

    let key = find_vig_xor_key(&ct, k_sz[0].0);

    println!("{}", String::from_utf8(bxor(&ct, &key)).unwrap());
}
