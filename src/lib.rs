#![feature(test)]
#![feature(stdsimd)]
#![feature(mmx_target_feature)]
#![feature(slice_internals)]

extern crate core;
extern crate stdsimd;
extern crate test;
#[macro_use]
extern crate jetscii;
extern crate faster;
extern crate memchr;
extern crate twoway;

#[cfg(test)]
mod bench;

// FIXME: Trying doing this with aligned instructions
pub fn is_ascii_simd(slice: &[u8]) -> bool {

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
        unsafe { is_ascii_simd_x86_64(slice) }
    } else {
        slice.is_ascii()
    };

    #[cfg(target_arch = "x86_64")]
    unsafe fn is_ascii_simd_x86_64(slice: &[u8]) -> bool {
        use std::arch::x86_64::*;
        use std::simd::{u8x32, u8x16, u8x8};
        use std::simd::FromBits;

        let mut slice = slice;

        if cfg!(target_feature = "avx2") ||
            is_x86_feature_detected!("avx2")
        {
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

        slice.is_ascii()
    }
}

pub fn is_ascii_simd2(slice: &[u8]) -> bool {

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
        unsafe { is_ascii_simd_x86_64(slice) }
    } else {
        slice.is_ascii()
    };

    #[cfg(target_arch = "x86_64")]
    unsafe fn is_ascii_simd_x86_64(slice: &[u8]) -> bool {
        use std::arch::x86_64::*;
        use std::simd::{u8x32, u8x16, u8x8};
        use std::simd::FromBits;

        let have_avx2 = cfg!(target_feature = "avx2") ||
            is_x86_feature_detected!("avx2");
        let have_sse2 = cfg!(target_feature = "sse2") ||
            is_x86_feature_detected!("sse2");
        let have_sse = cfg!(target_feature = "sse") ||
            is_x86_feature_detected!("sse");

        let mut slice = slice;

        let avx2_align = 32;
        let sse2_align = 16;
        let sse_align = 8;

        let is_aligned = |addr: usize, align: usize| addr & (align - 1) == 0;

        let max_align = if have_avx2 { avx2_align }
        else if have_sse2 { sse2_align }
        else if have_sse { sse_align }
        else { 1 };
        
        loop {
            if slice.is_empty() { return true }
            let addr = slice.get_unchecked(0) as *const _ as usize;
            if is_aligned(addr, max_align) {
                break;
            }
            if have_sse2 && is_aligned(addr, sse2_align) && slice.len() >= 16 {
                let vec = u8x16::load_aligned_unchecked(slice.get_unchecked(..16));
                let vec: __m128i = __m128i::from_bits(vec);
                if _mm_movemask_epi8(vec) != 0 {
                    return false;
                }
                slice = slice.get_unchecked(16..);
            } else if have_sse && is_aligned(addr, sse_align) && slice.len() >= 8 {
                let vec = u8x8::load_aligned_unchecked(slice.get_unchecked(..8));
                let vec: __m64 = __m64::from_bits(vec);
                if _mm_movemask_pi8(vec) != 0 {
                    return false;
                }
                slice = slice.get_unchecked(8..);
            } else {
                if !slice.get_unchecked(0).is_ascii() {
                    return false;
                }
                slice = slice.get_unchecked(1..);
            }
        }

        if have_avx2 {
            while slice.len() >= 32 {
                let vec = u8x32::load_aligned_unchecked(slice.get_unchecked(..32));
                let vec: __m256i = __m256i::from_bits(vec);
                if _mm256_movemask_epi8(vec) != 0 {
                    return false;
                }
                slice = slice.get_unchecked(32..);
            }
            debug_assert!(slice.len() < 32);
        }

        if have_sse2 {
            while slice.len() >= 16 {
                let vec = u8x16::load_aligned_unchecked(slice.get_unchecked(..16));
                let vec: __m128i = __m128i::from_bits(vec);
                if _mm_movemask_epi8(vec) != 0 {
                    return false;
                }
                slice = slice.get_unchecked(16..);
            }
            debug_assert!(slice.len() < 16);
        }

        if have_sse {
            while slice.len() >= 8 {
                let vec = u8x8::load_aligned_unchecked(slice.get_unchecked(..8));
                let vec: __m64 = __m64::from_bits(vec);
                if _mm_movemask_pi8(vec) != 0 {
                    return false;
                }
                slice = slice.get_unchecked(8..);
            }
            debug_assert!(slice.len() < 8);
        }

        slice.is_ascii()
    }
}

#[derive(PartialEq, Eq)]
pub enum Accel { AVX2, SSE2, SSE, Any }

pub fn is_ascii_auto_simd(slice: &[u8], accel: Accel) -> bool {

    return if cfg!(target_arch = "x86_64") {
        if (cfg!(target_feature = "avx2") || is_x86_feature_detected!("avx2"))
            && (accel == Accel::AVX2 || accel == Accel::Any) {
            #[target_feature(enable = "avx2")]
            #[inline]
            unsafe fn go(slice: &[u8]) -> bool {
                slice.is_ascii()
            }
            unsafe { go(slice) }
        } else if (cfg!(target_feature = "sse2") || is_x86_feature_detected!("sse2"))
            && (accel == Accel::SSE2 || accel == Accel::Any) {
            #[target_feature(enable = "sse2")]
            #[inline]
            unsafe fn go(slice: &[u8]) -> bool {
                slice.is_ascii()
            }
            unsafe { go(slice) }
        } else if (cfg!(target_feature = "sse") || is_x86_feature_detected!("sse"))
            && (accel == Accel::SSE || accel == Accel::Any) {
            #[target_feature(enable = "sse")]
            #[inline]
            unsafe fn go(slice: &[u8]) -> bool {
                slice.is_ascii()
            }
            unsafe { go(slice) }
        } else {
            slice.is_ascii()
        }
    } else {
        slice.is_ascii()
    }
}

pub fn fast_lines(buf: &str) -> FastLines {
    FastLines(buf.as_bytes())
}

pub struct FastLines<'a>(&'a [u8]);

impl<'a> Iterator for FastLines<'a> {
    type Item = &'a str;

    // TODO: inline vs inline(never)
    fn next(&mut self) -> Option<&'a str> {
        use memchr::memchr;

        let slice = &mut self.0;

        if self.0.is_empty() {
            return None;
        }

        let line;

        unsafe {
            if let Some(i) = memchr(b'\n', slice) {
                if i > 0 && slice.get_unchecked(i - 1) == &b'\r' {
                    line = slice.get_unchecked(0..i - 1);
                    *slice = slice.get_unchecked(i + 1..);
                } else {
                    line = slice.get_unchecked(0..i);
                    *slice = slice.get_unchecked(i + 1..);
                }
            } else {
                line = slice;
                *slice = slice.get_unchecked(0..0);
            }

            Some(::std::str::from_utf8_unchecked(line))
        }
    }
}
