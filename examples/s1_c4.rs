extern crate matasano;
extern crate hex;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Error,BufReader};

use matasano::cracks::{minimise_etaoin_byte,etaoin_score};
use matasano::ops::bxor;

// Not going for severe efficiency with this solution. TBH, native code is fast enough readability
// and manageability are better goals. Especially since the cost of repeating a string xor is very
// low.
//
// May be better off in practice rolling analysis in to the parsing phase but it's less clean.

/// Just read the file to lines and unpack the hex.
fn load_sts() -> Result<Vec<Vec<u8>>,Error> {
    let f = File::open("examples/4.txt")?;
    let reader = BufReader::new(f);

    Ok(reader
        .lines()
        .map(|line| hex::decode(line.unwrap()).unwrap())
        .collect())
}

fn main() {
    let cts = load_sts().unwrap();
    let best = cts.iter().map(|ct| {
        let minb = minimise_etaoin_byte(&ct);
        let pt = bxor(&ct,&[minb]);
        let score = etaoin_score(&pt);
        (pt,minb,score)
    }).min_by(|res1,res2| res1.2.cmp(&res2.2)).unwrap();

    println!("Result: {}", String::from_utf8(best.0).unwrap());
}
