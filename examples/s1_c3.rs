#![feature(inclusive_range_syntax)]

extern crate matasano;
extern crate hex;

use matasano::cracks::minimise_etaoin_byte;

const STRANG: &'static str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

fn main() {
    let ct = hex::decode(STRANG).expect("PEBCAK");

    let key = minimise_etaoin_byte(&ct);
    
    let decrypted = String::from_utf8(matasano::ops::bxor(&ct,&[key]));

    println!("{}", decrypted.unwrap())

}
