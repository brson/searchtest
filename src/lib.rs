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
use memchr::*;
use jetscii::ByteSubstring;


static FORBIDDEN_CHARS: &[u8] = &[b'#', b'_', b'*', b'=', b'-', b'~', b'|', b'[', b'\\', b'>', b'^', b'`', b'&', b'/', b':', b'@'];

static EXAMPLE_BIG: &str = include_str!("comrak-readme.md");
static EXAMPLE_SIMPLE: &str = include_str!("simple.md");
static EXAMPLE_WWW: &str = include_str!("www.md");
static EXAMPLE_LIPSUM: &str = include_str!("lipsum.md");
static EXAMPLE_LIPSUM_BR: &str = include_str!("lipsum-linebreaks.md");
static EXAMPLE_LIPSUM_EMPH: &str = include_str!("lipsum-emph.md");
static EXAMPLE_LIPSUM_AT: &str = include_str!("lipsum-at.md");

mod find_set_of_bytes_early {

    use super::*;

    #[bench]
    fn find_from_set_jetscii_bytes_lipsum_emph(b: &mut Bencher) {
        let bytes = bytes!(b'#', b'_', b'*', b'=', b'-', b'~', b'|', b'[', b'\\', b'>', b'^', b'`', b'&', b'/', b':', b'@');
        b.iter(|| {
            let r = bytes.find(EXAMPLE_LIPSUM_EMPH.as_bytes());
            assert!(r.is_some());
            assert_eq!(EXAMPLE_LIPSUM_EMPH.as_bytes()[r.unwrap()] as char, '_');
            assert_eq!(r, Some(419));
            black_box(r);
        });
    }

    #[bench]
    fn find_from_set_jetscii_ascii_lipsum_emph(b: &mut Bencher) {
        let chars = ascii_chars!('#', '_', '*', '=', '-', '~', '|', '[', '\\', '>', '^', '`', '&', '/', ':', '@');
        b.iter(|| {
            let r = chars.find(EXAMPLE_LIPSUM_EMPH);
            assert!(r.is_some());
            assert_eq!(EXAMPLE_LIPSUM_EMPH.as_bytes()[r.unwrap()] as char, '_');
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
            assert!(r.is_some());
            assert_eq!(EXAMPLE_LIPSUM_EMPH.as_bytes()[r.unwrap()] as char, '_');
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
            assert!(r.is_some());
            assert_eq!(EXAMPLE_LIPSUM_EMPH.as_bytes()[r.unwrap()] as char, '_');
            assert_eq!(r, Some(419));
            black_box(r);
        });
    }

    #[bench]
    fn find_from_set_multi_memchr_lipsum_emph(b: &mut Bencher) {
        b.iter(|| {
            let mut r = None;
            for window in FORBIDDEN_CHARS.chunks(3) {
                let r_ = match window {
                    [a, b, c] => memchr3(*a, *b, *c, EXAMPLE_LIPSUM_EMPH.as_bytes()),
                    [a, b] => memchr2(*a, *b, EXAMPLE_LIPSUM_EMPH.as_bytes()),
                    [a] => memchr(*a, EXAMPLE_LIPSUM_EMPH.as_bytes()),
                    _ => unreachable!(),
                };
                if r_.is_some() {
                    r = r_;
                    break;
                }
            }
            assert!(r.is_some());
            assert_eq!(EXAMPLE_LIPSUM_EMPH.as_bytes()[r.unwrap()] as char, '_');
            assert_eq!(r, Some(419));
            black_box(r);
        });
    }

    #[bench]
    fn find_from_set_multi_jump_table_lipsum_emph(b: &mut Bencher) {
        b.iter(|| {
            let mut r = None;
            for (i, ch) in EXAMPLE_LIPSUM_EMPH.as_bytes().iter().enumerate() {
                match ch {
                    b'#' |  b'_' |  b'*' |  b'=' |  b'-' |  b'~' |  b'|' |  b'[' |
                    b'\\' | b'>' |  b'^' |  b'`' |  b'&' |  b'/' |  b':' |  b'@' => {
                        r = Some(i);
                        break
                    }
                    _ => ()
                }
            }
            assert!(r.is_some());
            assert_eq!(EXAMPLE_LIPSUM_EMPH.as_bytes()[r.unwrap()] as char, '_');
            assert_eq!(r, Some(419));
            black_box(r);
        });
    }
    
}

mod find_set_of_bytes_late {

    use super::*;

    #[bench]
    fn find_from_set_jetscii_bytes_lipsum_at(b: &mut Bencher) {
        let bytes = bytes!(b'#', b'_', b'*', b'=', b'-', b'~', b'|', b'[', b'\\', b'>', b'^', b'`', b'&', b'/', b':', b'@');
        b.iter(|| {
            let r = bytes.find(EXAMPLE_LIPSUM_AT.as_bytes());
            assert!(r.is_some());
            assert_eq!(EXAMPLE_LIPSUM_AT.as_bytes()[r.unwrap()] as char, '@');
            assert_eq!(r, Some(610));
            black_box(r);
        });
    }

    #[bench]
    fn find_from_set_jetscii_ascii_lipsum_at(b: &mut Bencher) {
        let chars = ascii_chars!('#', '_', '*', '=', '-', '~', '|', '[', '\\', '>', '^', '`', '&', '/', ':', '@');
        b.iter(|| {
            let r = chars.find(EXAMPLE_LIPSUM_AT);
            assert!(r.is_some());
            assert_eq!(EXAMPLE_LIPSUM_AT.as_bytes()[r.unwrap()] as char, '@');
            assert_eq!(r, Some(610));
            black_box(r);
        });
    }
    
    #[bench]
    fn find_from_set_position_table_lipsum_at(b: &mut Bencher) {
        let mut table: [bool; 256] = [false; 256];
        for ch in FORBIDDEN_CHARS {
            table[*ch as usize] = true;
        }
        let table = table;
        b.iter(|| {
            let r = EXAMPLE_LIPSUM_AT.as_bytes().iter().position(|byte| {
                if table[*byte as usize] {
                    true
                } else {
                    false
                }
            });
            assert!(r.is_some());
            assert_eq!(EXAMPLE_LIPSUM_AT.as_bytes()[r.unwrap()] as char, '@');
            assert_eq!(r, Some(610));
            black_box(r);
        });
    }

    #[bench]
    fn find_from_set_multi_memchr_lipsum_at(b: &mut Bencher) {
        b.iter(|| {
            let mut r = None;
            for chunk in FORBIDDEN_CHARS.chunks(3) {
                let r_ = match chunk {
                    [a, b, c] => memchr3(*a, *b, *c, EXAMPLE_LIPSUM_AT.as_bytes()),
                    [a, b] => memchr2(*a, *b, EXAMPLE_LIPSUM_AT.as_bytes()),
                    [a] => memchr(*a, EXAMPLE_LIPSUM_AT.as_bytes()),
                    _ => unreachable!(),
                };
                if r_.is_some() {
                    r = r_;
                    break;
                }
            }
            assert!(r.is_some());
            assert_eq!(EXAMPLE_LIPSUM_AT.as_bytes()[r.unwrap()] as char, '@');
            assert_eq!(r, Some(610));
            black_box(r);
        });
    }

    #[bench]
    fn find_from_set_multi_jump_table_lipsum_at(b: &mut Bencher) {
        b.iter(|| {
            let mut r = None;
            for (i, ch) in EXAMPLE_LIPSUM_AT.as_bytes().iter().enumerate() {
                match ch {
                    b'#' |  b'_' |  b'*' |  b'=' |  b'-' |  b'~' |  b'|' |  b'[' |
                    b'\\' | b'>' |  b'^' |  b'`' |  b'&' |  b'/' |  b':' |  b'@' => {
                        r = Some(i);
                        break
                    }
                    _ => ()
                }
            }
            assert!(r.is_some());
            assert_eq!(EXAMPLE_LIPSUM_AT.as_bytes()[r.unwrap()] as char, '@');
            assert_eq!(r, Some(610));
            black_box(r);
        });
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

mod find_substring {

    use super::*;

    #[bench]
    fn find_substring_std(b: &mut Bencher) {
        b.iter(|| {
            let r = EXAMPLE_WWW.find("www.");
            assert!(r.is_some());
            assert_eq!(EXAMPLE_WWW.as_bytes()[r.unwrap()] as char, 'w');
            assert_eq!(r, Some(599));
        });
    }

    #[bench]
    fn find_substring_jetscii(b: &mut Bencher) {
        let sub = ByteSubstring::new("www.".as_bytes());
        b.iter(|| {
            let r = sub.find(EXAMPLE_WWW.as_bytes());
            assert!(r.is_some());
            assert_eq!(EXAMPLE_WWW.as_bytes()[r.unwrap()] as char, 'w');
            assert_eq!(r, Some(599));
        });
    }
}

mod is_ascii {

    use super::*;

    #[bench]
    fn is_ascii_std(b: &mut Bencher) {
    }

    #[bench]
    fn is_ascii_faster(b: &mut Bencher) {
    }

    #[bench]
    fn line_split_std(b: &mut Bencher) {
    }

}

mod split_newlines {

    use super::*;

}
