extern crate matasano;
extern crate hex;

use std::fs::File;
use std::io::prelude::*;
use std::io;

fn load_cts() -> Vec<Vec<u8>> {
    let f = io::BufReader::new(
        File::open("examples/8.txt").expect("Couldn't open challenge file")
        );
    f.lines()
        .map(|line| hex::decode(line.unwrap()).expect("Couldn't decode hex"))
        .collect::<Vec<Vec<u8>>>()
}

fn main() {
    let cts = load_cts();

    let (line, count) = 
        cts.iter()
        .map(|ct| matasano::cracks::find_dup_blocks(&ct, 16))
        .enumerate()
        .max_by(|a,b| a.1.cmp(&b.1)).expect("Somehow couldn't find an ordering on integers!");

    println!("MaxDupCount: {}\nLine: {}", count, line);
}
