#![allow(warnings)]
#![feature(test)]

extern crate test;
#[macro_use]
extern crate jetscii;

use test::Bencher;
use jetscii::Bytes;
use test::black_box;


static EXAMPLE_LIPSUM_EMPH: &str = include_str!("lipsum-emph.md");

#[bench]
fn find_from_set_jetscii_bytes_lipsum_emph(b: &mut Bencher) {
    let bytes = bytes!(b'#', b'_', b'*', b'=', b'-', b'~', b'|', b'[', b'\\', b'>', b'^', b'`', b'&', b'/', b':', b'@');
    b.iter(|| {
        let r = bytes.find(EXAMPLE_LIPSUM_EMPH.as_bytes());
        assert_eq!(r, Some(419));
    });
}
