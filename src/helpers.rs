extern crate base64;

use std::fs::File;
use std::io::prelude::*;
use std::io;

pub fn load_ct_b64(filename: &str) -> Result<Vec<u8>,io::Error> {
    let mut f = File::open(filename)?;
    let mut b = Vec::new();
    let _sz = f.read_to_end(&mut b);

    if let Ok(r) = base64::decode_config(&b, base64::MIME) {
        Ok(r)
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "Failed to decode"))
    }
}
