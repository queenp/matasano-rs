use std::collections::BTreeMap;
use std::ops::Range;
use std::iter::Iterator;

use lazysort::{SortedBy,LazySortIteratorBy};

use slice_stripes::Striped;

use ops;
/// Const borrowed outright from https://github.com/nccgroup/featherduster
const ETAOIN: &'static [u8;256] = b" etaonihrsdlu\r\nmw,cfgypb.\"vkI-TM';D!?ASHLCxqWBPYFjNEGOJ:Rz_()V*U1XKQ/098273546@\xc3$]\xa9[%\xef\xbb#\xbf\x00\xed\\\x8a\xb8\xe6\x83\xb1\xdf|\xaa\xd8\x19\xa3\xd1\x12\xff\x9c\xca\x0b\xf8\xf0\x95\x04\xf1`\x8e\xbc\xea+\x87\xb5\xe3\x80\xdc\x1d\x9d\xa7\xd5\xa0\xce\x0f\xfc=\x99\xc7\xf5\xc0\x01\xee\x8b\xb9\xe7\x84\x16\xb2\xe0\xb4}\xab\xd9\x1a\x93\xa4\xd2\x13\xf9\xcb\x0c\x02\x96\xc4\x05\xf2\x8f\xebZ\x88\xb6\x81\xaf\xdd\x1e\xa8\xd6\x17\xa1\xcf\x10\xfd>\x9a\xc8\t\xf6\xc1\x08^\x8c\xba\xe8\xbd\x85\xb3\xe1~\xac\xda\x1b\xa5\xd3\x14\x92\x9e\xcc\xfa\x97\xc5\x06\xf3\xd7\x90\xbe\xec\x89\xb7\xe5&\x0e\x82\xb0\xe4\xde\x1f{\x18\xa2\xd0\x11\xfe\x9b\xc9\xae\xf7\x94\xc2\x03\x8d\xe9\x86\xe2\x7f\xad\xdb\x1c\xa6\xd4\x15\x9f\xcd\xfb<\x98\xc6\x07\xf4\x91";

/// Cheap and lazy scoring function. Lower is closer to english.
///
/// Return value: a score based on position in a typical letter frequency chart.
///
/// Designed to assist in identifying plaintext (by minimising this score). This
/// doesn't always work and the Chi Squared scoring is arguably more sophisticated and capable
/// of maximising error in decryption attempts (which arguably is more effective than minimising
/// difference from English).
pub fn etaoin_score(val: &[u8]) -> usize {
    val.iter()
        .fold(0, |acc, &byte| {
            if let Some(unrank) = ETAOIN.iter().position(|&z| z == byte ) {
                acc+unrank
            } else {
                acc + 256
            }
        })
}

pub fn minimise_etaoin_byte(ct: &[u8]) -> u8 {
    (0..=255u8)
        .map(|mask| 
            (mask, etaoin_score(&ops::bxor(ct, &[mask]) ) )
        ).min_by(|a,b| a.1.cmp(&b.1)).unwrap().0
}

/// Find the minimal etaoin byte for a given iterator over u8.
///
/// It would be nice to do this lazily rather than caching a collection from the input
/// perhaps using the new "impl Trait" return type feature once it's stabilised.
pub fn minimise_etaoin_byte_iter<T>(ct: T) -> u8
where T: Iterator<Item = u8> {
    let ctcached = ct.collect::<Vec<u8>>();
    (0..=255u8)
        .map(|mask|
             (mask, etaoin_score(&ops::bxor(&ctcached, &[mask] ) ))
             ).min_by(|a,b| a.1.cmp(&b.1)).unwrap().0
}

/// Return a list of key sizes ordered by hamming score candidacy
pub fn find_keysize(text: &[u8], range: Range<usize>) -> Vec<(usize,f64)> {
    let mut results = range.
        map(|size| {
            let mut chunks = text.chunks(size);
            let fst = chunks.next().unwrap();
            let hds = chunks.filter_map(|chunk| {
                if chunk.len() == fst.len() {
                    Some(ops::hamming_d(&fst,&chunk) )
                } else {
                    None
                }
            }).fold((0,0), |acc, i| {
                (acc.0+i, acc.1+1)});
            // println!("{}, {}, {}", hds.0, hds.1, size);
            (size, f64::from(hds.0)/f64::from(hds.1 * size as u32))
        }).collect::<Vec<(usize,f64)>>();

    // This is a use case which would massively benefit from using lazysort, however getting the
    // trait to be asserted to the previous `Map` is pretty awkward.
    //
    // We are only really interested in, perhaps, the first few best candidates at most, so a full
    // sorting of the results is not necessary or efficient compared with successively pulling the
    // minimum by a given quantity for a few out of a much larger set. Lazysort conveniently
    // produces such an iterator of ordered results, which would allow us to be returning a much
    // simpler result and avoiding unnecessary memory allocations.
    results.sort_by(|a,b| a.1.partial_cmp(&b.1).unwrap());
    results
}

pub fn find_vig_xor_key(ct: &[u8], keysize: usize) -> Vec<u8> {
    ct.stripes(keysize)
        .map(|stripe| minimise_etaoin_byte_iter(stripe))
        .collect::<Vec<u8>>()
}

/// Find the counts of all blocks
pub fn count_blocks(ct: &[u8], b_sz: usize) -> BTreeMap<Vec<u8>,usize> {
    let mut counts = BTreeMap::new();
    for block in ct.chunks(b_sz) {
        counts.entry(block.to_owned())
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }
    counts
}

/// Return the maximum count for duplicated blocks found
pub fn find_dup_blocks(ct: &[u8], b_sz: usize) -> usize {
    *count_blocks(ct,b_sz)
        .iter()
        .max_by(|a,b|a.1.cmp(b.1)).expect("Couldn't compare two integer values. Something is very wrong!")
        .1
}

#[cfg(test)]
mod tests {
    use helpers::{load_ct_b64};
    use super::find_keysize;
    #[test]
    fn test_keysize_hamming_search() {
        // Must implement this test;
        let ct = load_ct_b64("examples/6.txt").unwrap();
        let ksz = find_keysize(&ct, 2..40);
        assert_eq!(ksz[0].0, 29);
    }
}
