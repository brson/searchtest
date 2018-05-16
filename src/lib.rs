#![allow(warnings)]
#![feature(test)]

extern crate test;
#[macro_use]
extern crate jetscii;
//extern crate faster;

use test::Bencher;
use jetscii::Bytes;
use test::black_box;


static FORBIDDEN_CHARS: &[u8] = &[b'#', b'_', b'*', b'=', b'-', b'~', b'|', b'[', b'\\', b'>', b'^', b'`', b'&', b'/', b':', b'@'];

static EXAMPLE_BIG: &str = include_str!("comrak-readme.md");
static EXAMPLE_SIMPLE: &str = include_str!("simple.md");
static EXAMPLE_WWW: &str = include_str!("www.md");
static EXAMPLE_LIPSUM: &str = include_str!("lipsum.md");
static EXAMPLE_LIPSUM_BR: &str = include_str!("lipsum-linebreaks.md");
static EXAMPLE_LIPSUM_EMPH: &str = include_str!("lipsum-emph.md");

/*#[test]
fn find_from_set_jetscii_bytes_lipsum_emph() {
    let bytes = bytes!(b'#', b'_', b'*', b'=', b'-', b'~', b'|', b'[', b'\\', b'>', b'^', b'`', b'&', b'/', b':', b'@');
    for i in 0..100 {
    let r = bytes.find(EXAMPLE_LIPSUM_EMPH.as_bytes());
        assert_eq!(r, Some(419));
    }
}

#[test]
fn find_from_set_jetscii_ascii_lipsum_emph() {
    let chars = ascii_chars!(b'#', b'_', b'*', b'=', b'-', b'~', b'|', b'[', b'\\', b'>', b'^', b'`', b'&', b'/', b':', b'@');
    let r = chars.find(EXAMPLE_LIPSUM_EMPH);
    assert_eq!(r, Some(419));
}*/

#[bench]
fn find_from_set_jetscii_bytes_lipsum_emph(b: &mut Bencher) {
    let bytes = bytes!(b'#', b'_', b'*', b'=', b'-', b'~', b'|', b'[', b'\\', b'>', b'^', b'`', b'&', b'/', b':', b'@');
    b.iter(|| {
        let r = bytes.find(EXAMPLE_LIPSUM_EMPH.as_bytes());
        assert_eq!(r, Some(419));
        black_box(r);
    });
}

/*#[bench]
fn find_from_set_jetscii_ascii_lipsum_emph(b: &mut Bencher) {
    let chars = ascii_chars!(b'#', b'_', b'*', b'=', b'-', b'~', b'|', b'[', b'\\', b'>', b'^', b'`', b'&', b'/', b':', b'@');
    b.iter(|| {
        let r = chars.find(EXAMPLE_LIPSUM_EMPH);
        assert_eq!(r, Some(419));
        black_box(r);
    });
}

// TODO: benchmark chars creation

#[bench]
fn find_from_set_table_lipsum_emph(b: &mut Bencher) {
    let table: [u8; 256] = [0; 256];
    b.iter(|| {
        for byte in EXAMPLE_LIPSUM_EMPH.as_bytes() {

        }
    });
}

#[bench]
fn find_substring_std(b: &mut Bencher) {
}

#[bench]
fn is_ascii_std(b: &mut Bencher) {
}

#[bench]
fn is_ascii_faster(b: &mut Bencher) {
}

#[bench]
fn line_split_std(b: &mut Bencher) {
}
*/
