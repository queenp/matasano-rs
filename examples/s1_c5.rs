extern crate matasano;
extern crate hex;

use matasano::ops::bxor;

const PT: &'static [u8] = b"Burning 'em, if you ain't quick and nimble\n\
                            I go crazy when I hear a cymbal";

const CT: &'static str = "0b3637272a2b2e63622c2e69692a23693a2a3c63242\
                            02d623d63343c2a26226324272765272a282b2f204\
                            30a652e2c652a3124333a653e2b2027630c692b202\
                            83165286326302e27282f";

const KEY: &'static [u8] = b"ICE";
fn main() {
    let result = bxor(PT,KEY);
    if result == hex::decode(CT).unwrap() {
        println!("Qa'pla!");
    } else {
        println!("Sad trombone");
    }
}
