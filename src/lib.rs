#![feature(inclusive_range_syntax)]
#![feature(conservative_impl_trait)]
#![feature(entry_and_modify)]

extern crate lazysort;
extern crate slice_stripes;
extern crate crypto;
#[cfg(test)]
mod helpers;
pub mod cracks;
pub mod ops;
pub mod models;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
