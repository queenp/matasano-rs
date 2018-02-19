extern crate hex;
extern crate base64;

const INPUT: &'static str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
const EXPECTED: &'static str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
fn main() {
    let somebytes = hex::decode(INPUT).expect("Hex decoding failed");

    let recoded = base64::encode(&somebytes);

    println!("Re-encoded string: {}", recoded);

    if recoded != EXPECTED {
        println!("Unexpected string");
    } else {
        println!("Result as expected!");
    }
}
