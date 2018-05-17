#![allow(warnings)]
#![feature(test)]
#![feature(stdsimd)]
#![feature(mmx_target_feature)]

extern crate stdsimd;
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
use faster::*;


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

    #[bench]
    fn find_substring_memchr(b: &mut Bencher) {
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
            assert_eq!(r, Some(599));
        });
    }

}


fn is_ascii_simd(slice: &[u8]) -> bool {

    return if cfg!(target_arch = "x86_64") &&
        ((cfg!(target_feature = "avx2") ||
          cfg!(target_feature = "sse2") ||
          cfg!(target_feature = "sse") ||
          cfg!(target_feature = "mmx")) ||
         (is_x86_feature_detected!("avx2") ||
          is_x86_feature_detected!("sse2") ||
          is_x86_feature_detected!("sse") ||
          is_x86_feature_detected!("mmx")))
    {
        unsafe { is_ascii_simd_x86_64_simd(slice) }
    } else {
        slice.iter().all(|b| b.is_ascii())
    };

    #[cfg(target_arch = "x86_64")]
    unsafe fn is_ascii_simd_x86_64_simd(slice: &[u8]) -> bool {
        use std::arch::x86_64::*;
        use std::simd::{u8x32, u8x16, u8x8};
        use std::simd::FromBits;

        let mut slice = slice;

        if cfg!(target_feature = "avx2") ||
            is_x86_feature_detected!("avx2")
        {
            #[target_feature(enable = "avx2")]
            while slice.len() >= 32 {
                let vec = u8x32::load_unaligned(&slice[..32]);
                let vec: __m256i = __m256i::from_bits(vec);
                if _mm256_movemask_epi8(vec) != 0 {
                    return false;
                }
                slice = &slice[32..];
            }
            debug_assert!(slice.len() < 32);
        }

        if cfg!(target_feature = "sse2") ||
            is_x86_feature_detected!("sse2")
        {
            while slice.len() >= 16 {
                let vec = u8x16::load_unaligned(&slice[..16]);
                let vec: __m128i = __m128i::from_bits(vec);
                if _mm_movemask_epi8(vec) != 0 {
                    return false;
                }
                slice = &slice[16..];
            }
            debug_assert!(slice.len() < 16);
        }

        if cfg!(target_feature = "sse") ||
            is_x86_feature_detected!("sse")
        {
            while slice.len() >= 8 {
                let vec = u8x8::load_unaligned(&slice[..8]);
                let vec: __m64 = __m64::from_bits(vec);
                if _mm_movemask_pi8(vec) != 0 {
                    return false;
                }
                slice = &slice[8..];
            }
            debug_assert!(slice.len() < 8);
        }

        slice.iter().all(|b| b.is_ascii())
    }
}

mod is_ascii {

    use super::*;

    #[bench]
    fn is_ascii_std(b: &mut Bencher) {
        b.iter(|| {
            let is_ascii = EXAMPLE_LIPSUM.as_bytes().iter().all(|b| b.is_ascii());
            assert!(is_ascii);
        });
    }

    #[bench]
    fn is_ascii_simd(b: &mut Bencher) {
        b.iter(|| {
            let is_ascii = super::is_ascii_simd("test".as_bytes());
            assert!(is_ascii);
        });
    }

    /*#[bench]
    fn is_ascii_faster(b: &mut Bencher) {
        let r = EXAMPLE_LIPSUM.as_bytes().simd_iter(u8s(0))
           .simd_reduce(u8s::splat(0), |a, v| (v & u8s::splat(128)) + a).sum();
        let is_ascii = r == 0;
        assert!(is_ascii);
    }*/

}

mod is_not_ascii {

    use super::*;

    #[bench]
    fn is_not_ascii_std(b: &mut Bencher) {
        b.iter(|| {
            let is_ascii = EXAMPLE_UNICODE.as_bytes().iter().all(|b| b.is_ascii());
            assert!(!is_ascii);
        });
    }

    /*#[bench]
    fn is_not_ascii_faster(b: &mut Bencher) {
        let r = EXAMPLE_UNICODE.as_bytes().simd_iter(u8s(0))
           .simd_reduce(u8s::splat(0), |a, v| (v & u8s::splat(128)) + a).sum();
        let is_ascii = r == 0;
        assert!(!is_ascii);
    }*/

}

mod is_not_ascii_late {

    use super::*;

    #[bench]
    fn is_not_ascii_std(b: &mut Bencher) {
        b.iter(|| {
            let is_ascii = EXAMPLE_LATE_UNICODE.as_bytes().iter().all(|b| b.is_ascii());
            assert!(!is_ascii);
        });
    }

    /*#[bench]
    fn is_not_ascii_faster(b: &mut Bencher) {
        let r = EXAMPLE_LATE_UNICODE.as_bytes().simd_iter(u8s(0))
           .simd_reduce(u8s::splat(0), |a, v| (v & u8s::splat(128)) + a).sum();
        let is_ascii = r == 0;
        assert!(!is_ascii);
    }*/

}

mod split_newlines {

    use super::*;

    #[bench]
    fn line_split_std(b: &mut Bencher) {
    }

}
