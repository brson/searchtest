#![allow(warnings)]
#![feature(test)]

extern crate test;
#[macro_use]
extern crate jetscii;
extern crate faster;
extern crate memchr;

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

mod find_set_of_bytes {

    use super::*;

    #[bench]
    fn find_from_set_jetscii_bytes_lipsum_emph(b: &mut Bencher) {
        let bytes = bytes!(b'#', b'_', b'*', b'=', b'-', b'~', b'|', b'[', b'\\', b'>', b'^', b'`', b'&', b'/', b':', b'@');
        b.iter(|| {
            let r = bytes.find(EXAMPLE_LIPSUM_EMPH.as_bytes());
            assert_eq!(r, Some(419));
            black_box(r);
        });
    }

    #[bench]
    fn find_from_set_jetscii_ascii_lipsum_emph(b: &mut Bencher) {
        let chars = ascii_chars!('#', '_', '*', '=', '-', '~', '|', '[', '\\', '>', '^', '`', '&', '/', ':', '@');
        b.iter(|| {
            let r = chars.find(EXAMPLE_LIPSUM_EMPH);
            assert_eq!(r, Some(419));
            black_box(r);
        });
    }

    #[bench]
    fn find_from_set_open_table_lipsum_emph(b: &mut Bencher) {
        let mut table: [bool; 256] = [false; 256];
        for ch in FORBIDDEN_CHARS {
            table[*ch as usize] = true;
        }
        let table = table;
        b.iter(|| {
            let mut r = None;
            for (i, byte) in EXAMPLE_LIPSUM_EMPH.as_bytes().iter().enumerate() {
                if table[*byte as usize] {
                    r = Some(i);
                    break;
                }
            }
            assert_eq!(r, Some(419));
            black_box(r);
        });
    }

    #[bench]
    fn find_from_set_position_table_lipsum_emph(b: &mut Bencher) {
        let mut table: [bool; 256] = [false; 256];
        for ch in FORBIDDEN_CHARS {
            table[*ch as usize] = true;
        }
        let table = table;
        b.iter(|| {
            let r = EXAMPLE_LIPSUM_EMPH.as_bytes().iter().position(|byte| {
                if table[*byte as usize] {
                    true
                } else {
                    false
                }
            });
            assert_eq!(r, Some(419));
            black_box(r);
        });
    }


    #[bench]
    fn find_from_set_multi_memchr_table_lipsum_emph(b: &mut Bencher) {
    }
}

// TODO: benchmark chars creation
#[bench]
fn jetscii_setup(b: &mut Bencher) {
    b.iter(|| {
        let bytes = bytes!(b'#', b'_', b'*', b'=', b'-', b'~', b'|', b'[', b'\\', b'>', b'^', b'`', b'&', b'/', b':', b'@');
        black_box(bytes);
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

