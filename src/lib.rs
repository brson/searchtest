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

#[derive(PartialEq, Eq)]
pub enum Accel { AVX2, SSE2, SSE, Any }

pub fn is_ascii_auto_simd(slice: &[u8], accel: Accel) -> bool {

    return if cfg!(target_arch = "x86_64") {
        if (cfg!(target_feature = "avx2") || is_x86_feature_detected!("avx2"))
            && (accel == Accel::AVX2 || accel == Accel::Any) {
            #[target_feature(enable = "avx2")]
            {
                slice.iter().all(|b| b.is_ascii())
            }
        } else if (cfg!(target_feature = "sse2") || is_x86_feature_detected!("sse2"))
            && (accel == Accel::SSE2 || accel == Accel::Any) {
            #[target_feature(enable = "sse2")]
            {
                slice.iter().all(|b| b.is_ascii())
            }
        } else if (cfg!(target_feature = "sse") || is_x86_feature_detected!("sse"))
            && (accel == Accel::SSE || accel == Accel::Any) {
            #[target_feature(enable = "sse")]
            {
                slice.iter().all(|b| b.is_ascii())
            }
        } else {
            slice.iter().all(|b| b.is_ascii())
        }
    } else {
        slice.iter().all(|b| b.is_ascii())
    }
}
