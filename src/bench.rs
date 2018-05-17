#![allow(warnings)]

use test::Bencher;
use test::black_box;
use memchr::*;
use jetscii::ByteSubstring;
use super::*;

static FORBIDDEN_CHARS: &[u8] = &[b'#', b'_', b'*', b'=', b'-', b'~', b'|', b'[', b'\\', b'>', b'^', b'`', b'&', b'/', b':', b'@'];

static EXAMPLE_BIG: &str = include_str!("comrak-readme.md");
static EXAMPLE_SIMPLE: &str = include_str!("simple.md");
static EXAMPLE_WWW: &str = include_str!("www.md");
static EXAMPLE_LIPSUM: &str = include_str!("lipsum.md");
static EXAMPLE_LIPSUM_BR: &str = include_str!("lipsum-linebreaks.md");
static EXAMPLE_LIPSUM_EMPH: &str = include_str!("lipsum-emph.md");
static EXAMPLE_LIPSUM_AT: &str = include_str!("lipsum-at.md");
static EXAMPLE_UNICODE: &str = include_str!("unicode.md");
static EXAMPLE_LATE_UNICODE: &str = include_str!("late-unicode.md");

mod find_set_of_bytes_early {

    use super::*;

    #[bench]
    fn jetscii_bytes(b: &mut Bencher) {
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
    fn jetscii_ascii(b: &mut Bencher) {
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
    fn open_table(b: &mut Bencher) {
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
    fn position_table(b: &mut Bencher) {
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
    fn multi_memchr(b: &mut Bencher) {
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
    fn jump_table(b: &mut Bencher) {
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
    fn jetscii_bytes(b: &mut Bencher) {
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
    fn jetscii_ascii(b: &mut Bencher) {
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
    fn position_table(b: &mut Bencher) {
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
    fn multi_memchr(b: &mut Bencher) {
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
    fn jump_table(b: &mut Bencher) {
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

#[bench]
fn jetscii_setup(b: &mut Bencher) {
    b.iter(|| {
        let bytes = bytes!(b'#', b'_', b'*', b'=', b'-', b'~', b'|', b'[', b'\\', b'>', b'^', b'`', b'&', b'/', b':', b'@');
        black_box(bytes);
    });
}

// FIXME: Try twoway for substring search
// FIXME: Try intrinsic-based pcmp ala twoway crate
// FIXME: Try intrinsic-based 'short pattern' ala https://github.com/bluss/twoway/blob/master/src/pcmp.rs#L245
// Try pathological test case

mod find_substring {

    use super::*;

    #[bench]
    fn find_std(b: &mut Bencher) {
        b.iter(|| {
            let r = EXAMPLE_WWW.find("www.");
            assert!(r.is_some());
            assert_eq!(EXAMPLE_WWW.as_bytes()[r.unwrap()] as char, 'w');
            assert_eq!(r, Some(600));
            black_box(r);
        });
    }

    #[bench]
    fn contains_std(b: &mut Bencher) {
        b.iter(|| {
            let r = EXAMPLE_WWW.contains("www.");
            assert!(r);
            black_box(r);
        });
    }
    
    #[bench]
    fn jetscii(b: &mut Bencher) {
        let sub = ByteSubstring::new("www.".as_bytes());
        b.iter(|| {
            let r = sub.find(EXAMPLE_WWW.as_bytes());
            assert!(r.is_some());
            assert_eq!(EXAMPLE_WWW.as_bytes()[r.unwrap()] as char, 'w');
            assert_eq!(r, Some(600));
            black_box(r);
        });
    }

    #[bench]
    fn memchr_(b: &mut Bencher) {
        let needle = "www.".as_bytes();
        b.iter(|| {
            let mut total_offset = 0;
            let mut r = None;
            let mut slice = EXAMPLE_WWW.as_bytes();
            while let Some(i) = memchr(b'w', slice) {
                assert_eq!(slice[i], b'w');
                if slice.len() - i >= needle.len() {
                    let subslice = &slice[i..i + needle.len()];
                    total_offset += i;
                    if subslice == needle {
                        r = Some(total_offset);
                        break;
                    } else {
                        total_offset += 1;
                        slice = &slice[i + 1..];
                    }
                } else {
                    break;
                }
            }
            assert!(r.is_some());
            assert_eq!(EXAMPLE_WWW.as_bytes()[r.unwrap()] as char, 'w');
            assert_eq!(r, Some(600));
            black_box(r);
        });
    }

    #[bench]
    fn memchr_std(b: &mut Bencher) {
        let needle = "www.".as_bytes();
        b.iter(|| {
            let mut total_offset = 0;
            let mut r = None;
            let mut slice = EXAMPLE_WWW.as_bytes();
            while let Some(i) = ::core::slice::memchr::memchr(b'w', slice) {
                assert_eq!(slice[i], b'w');
                if slice.len() - i >= needle.len() {
                    let subslice = &slice[i..i + needle.len()];
                    total_offset += i;
                    if subslice == needle {
                        r = Some(total_offset);
                        break;
                    } else {
                        total_offset += 1;
                        slice = &slice[i + 1..];
                    }
                } else {
                    break;
                }
            }
            assert!(r.is_some());
            assert_eq!(EXAMPLE_WWW.as_bytes()[r.unwrap()] as char, 'w');
            assert_eq!(r, Some(600));
            black_box(r);
        });
    }
    
    #[bench]
    fn twoway(b: &mut Bencher) {
        b.iter(|| {
            let r = twoway::find_bytes(EXAMPLE_WWW.as_bytes(), b"www.");
            assert!(r.is_some());
            assert_eq!(EXAMPLE_WWW.as_bytes()[r.unwrap()] as char, 'w');
            assert_eq!(r, Some(600));
            black_box(r);
        });
    }

    #[bench]
    fn bmh(b: &mut Bencher) {
        b.iter(|| {
            let r = twoway::bmh::find(EXAMPLE_WWW.as_bytes(), b"www.");
            assert!(r.is_some());
            assert_eq!(EXAMPLE_WWW.as_bytes()[r.unwrap()] as char, 'w');
            assert_eq!(r, Some(600));
            black_box(r);
        });
    }
    
}

mod is_ascii {

    use super::*;

    #[bench]
    fn std_bytes_closure(b: &mut Bencher) {
        b.iter(|| {
            let is_ascii = EXAMPLE_LIPSUM.as_bytes().iter().all(|b| b.is_ascii());
            assert!(is_ascii);
            black_box(is_ascii);
        });
    }

    #[bench]
    fn std_bytes(b: &mut Bencher) {
        b.iter(|| {
            let is_ascii = EXAMPLE_LIPSUM.as_bytes().is_ascii();
            assert!(is_ascii);
            black_box(is_ascii);
        });
    }

    #[bench]
    fn simd(b: &mut Bencher) {
        b.iter(|| {
            let is_ascii = super::is_ascii_simd(EXAMPLE_LIPSUM.as_bytes());
            assert!(is_ascii);
            black_box(is_ascii);
        });
    }

    #[bench]
    fn auto_simd_avx2(b: &mut Bencher) {
        b.iter(|| {
            let is_ascii = super::is_ascii_auto_simd(EXAMPLE_LIPSUM.as_bytes(), Accel::AVX2);
            assert!(is_ascii);
            black_box(is_ascii);
        });
    }

    #[bench]
    fn auto_simd_sse2(b: &mut Bencher) {
        b.iter(|| {
            let is_ascii = super::is_ascii_auto_simd(EXAMPLE_LIPSUM.as_bytes(), Accel::SSE2);
            assert!(is_ascii);
            black_box(is_ascii);
        });
    }
    
    #[bench]
    fn auto_simd_sse(b: &mut Bencher) {
        b.iter(|| {
            let is_ascii = super::is_ascii_auto_simd(EXAMPLE_LIPSUM.as_bytes(), Accel::SSE);
            assert!(is_ascii);
            black_box(is_ascii);
        });
    }

    #[bench]
    fn auto_simd_any(b: &mut Bencher) {
        b.iter(|| {
            let is_ascii = super::is_ascii_auto_simd(EXAMPLE_LIPSUM.as_bytes(), Accel::Any);
            assert!(is_ascii);
            black_box(is_ascii);
        });
    }
}

mod is_not_ascii {

    use super::*;

    #[bench]
    fn std_bytes_closure(b: &mut Bencher) {
        b.iter(|| {
            let is_ascii = EXAMPLE_LATE_UNICODE.as_bytes().iter().all(|b| b.is_ascii());
            assert!(!is_ascii);
            black_box(is_ascii);
        });
    }

    #[bench]
    fn std_bytes(b: &mut Bencher) {
        b.iter(|| {
            let is_ascii = EXAMPLE_LATE_UNICODE.as_bytes().is_ascii();
            assert!(!is_ascii);
            black_box(is_ascii);
        });
    }

    #[bench]
    fn simd(b: &mut Bencher) {
        b.iter(|| {
            let is_ascii = super::is_ascii_simd(EXAMPLE_LATE_UNICODE.as_bytes());
            assert!(!is_ascii);
            black_box(is_ascii);
        });
    }

    #[bench]
    fn auto_simd_avx2(b: &mut Bencher) {
        b.iter(|| {
            let is_ascii = super::is_ascii_auto_simd(EXAMPLE_LATE_UNICODE.as_bytes(), Accel::AVX2);
            assert!(!is_ascii);
            black_box(is_ascii);
        });
    }

    #[bench]
    fn auto_simd_sse2(b: &mut Bencher) {
        b.iter(|| {
            let is_ascii = super::is_ascii_auto_simd(EXAMPLE_LATE_UNICODE.as_bytes(), Accel::SSE2);
            assert!(!is_ascii);
            black_box(is_ascii);
        });
    }
    
    #[bench]
    fn auto_simd_sse(b: &mut Bencher) {
        b.iter(|| {
            let is_ascii = super::is_ascii_auto_simd(EXAMPLE_LATE_UNICODE.as_bytes(), Accel::SSE);
            assert!(!is_ascii);
            black_box(is_ascii);
        });
    }

    #[bench]
    fn auto_simd_any(b: &mut Bencher) {
        b.iter(|| {
            let is_ascii = super::is_ascii_auto_simd(EXAMPLE_LATE_UNICODE.as_bytes(), Accel::Any);
            assert!(!is_ascii);
            black_box(is_ascii);
        });
    }
}

// Try twoway for split lines

mod split_lines {

    use super::*;

    #[bench]
    fn std(b: &mut Bencher) {
        b.iter(|| {
            let c = EXAMPLE_BIG.lines().count();
            assert_eq!(c, 172);
            black_box(c);
        });
    }

    #[bench]
    fn memchr2_(b: &mut Bencher) {
        b.iter(|| {
            let mut slice = EXAMPLE_BIG.as_bytes();
            let mut line = &[][..];
            let mut lines = 0;
            while !slice.is_empty() {
                if let Some(i) = memchr2(b'\r', b'\n', slice) {
                    line = &slice[..i];
                    if slice[i] == b'\r' && slice.len() > i && slice[i] == b'\n' {
                        slice = &slice[i + 2..];
                    } else if slice[i] == b'\n' {
                        slice = &slice[i + 1..];
                    }
                } else {
                    line = slice;
                    slice = &[];
                }
                lines += 1;
            }
            assert_eq!(lines, 172);
            black_box(line);
            black_box(lines);
        });
    }

    #[bench]
    fn memchr2_unchecked(b: &mut Bencher) {
        b.iter(|| {
            unsafe {
                let mut slice = EXAMPLE_BIG.as_bytes();
                let mut line = &[][..];
                let mut lines = 0;
                while !slice.is_empty() {
                    if let Some(i) = memchr2(b'\r', b'\n', slice) {
                        line = slice.get_unchecked(0..i);
                        if slice.get_unchecked(i) == &b'\r' && slice.len() > i
                            && slice.get_unchecked(i) == &b'\n'
                        {
                            slice = slice.get_unchecked(i + 2..slice.len());
                        } else if slice.get_unchecked(i) == &b'\n' {
                            slice = slice.get_unchecked(i + 1..slice.len());
                        }
                    } else {
                        line = slice;
                        slice = &[];
                    }
                    lines += 1;
                }
                assert_eq!(lines, 172);
                black_box(line);
                black_box(lines);
            }
        });
    }

    #[bench]
    fn memchr_(b: &mut Bencher) {
        b.iter(|| {
            let mut slice = EXAMPLE_BIG.as_bytes();
            let mut line = &[][..];
            let mut lines = 0;
            while !slice.is_empty() {
                if let Some(i) = memchr(b'\n', slice) {
                    if i > 0 && &slice[i - 1] == &b'\r'
                        && &slice[i] == &b'\n'
                    {
                        line = &slice[0..i - 1];
                        slice = &slice[i + 1..slice.len()];
                    } else if &slice[i] == &b'\n' {
                        line = &slice[0..i];
                        slice = &slice[i + 1..slice.len()];
                    }
                } else {
                    line = slice;
                    slice = &[];
                }
                lines += 1;
            }
            assert_eq!(lines, 172);
            black_box(line);
            black_box(lines);
        });
    }

    #[bench]
    fn memchr_unchecked(b: &mut Bencher) {
        b.iter(|| {
            unsafe {
                let mut slice = EXAMPLE_BIG.as_bytes();
                let mut line = &[][..];
                let mut lines = 0;
                while !slice.is_empty() {
                    if let Some(i) = memchr(b'\n', slice) {
                        if i > 0 && slice.get_unchecked(i - 1) == &b'\r'
                            && slice.get_unchecked(i) == &b'\n'
                        {
                            line = slice.get_unchecked(0..i - 1);
                            slice = slice.get_unchecked(i + 1..slice.len());
                        } else if slice.get_unchecked(i) == &b'\n' {
                            line = slice.get_unchecked(0..i);
                            slice = slice.get_unchecked(i + 1..slice.len());
                        }
                    } else {
                        line = slice;
                        slice = &[];
                    }
                    lines += 1;
                }
                assert_eq!(lines, 172);
                black_box(line);
                black_box(lines);
            }
        });
    }

    #[bench]
    fn line_split_memchr_std_unchecked(b: &mut Bencher) {
        b.iter(|| {
            unsafe {
                let mut slice = EXAMPLE_BIG.as_bytes();
                let mut line = &[][..];
                let mut lines = 0;
                while !slice.is_empty() {
                    if let Some(i) = ::core::slice::memchr::memchr(b'\n', slice) {
                        if i > 0 && slice.get_unchecked(i - 1) == &b'\r'
                            && slice.get_unchecked(i) == &b'\n'
                        {
                            line = slice.get_unchecked(0..i - 1);
                            slice = slice.get_unchecked(i + 1..slice.len());
                        } else if slice.get_unchecked(i) == &b'\n' {
                            line = slice.get_unchecked(0..i);
                            slice = slice.get_unchecked(i + 1..slice.len());
                        }
                    } else {
                        line = slice;
                        slice = &[];
                    }
                    lines += 1;
                }
                assert_eq!(lines, 172);
                black_box(line);
                black_box(lines);
            }
        });
    }
    
}
