extern crate hex;
extern crate matasano;

use std::str;

use matasano::ops::bxor;

const FIRST: &'static str = "1c0111001f010100061a024b53535009181c";
const SECOND: &'static str = "686974207468652062756c6c277320657965";
const THIRD: &'static str = "746865206b696420646f6e277420706c6179";

fn main() {
    let first = hex::decode(FIRST).expect("Panicked trying to unwrap first hex string");
    let second = hex::decode(SECOND).expect("Panicked trying to unwrap second hex string");
    let third = hex::decode(THIRD).expect("Panicked trying to unwrap third hex string");

    let result = bxor(&first, &second);

    println!("Result: {}", hex::encode(&result));
    if third == result {

        println!("Decoded: {}", String::from_utf8(result).expect("Expected a valid text"))

    }
}
