// [See license/rust-lang/libm] Copyright (c) 2018 Jorge Aparicio
#[cfg(not(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "simd")))]
pub fn sqrt(x: f32) -> f32 {
    const TINY: f32 = 1.0e-30;

    let mut z: f32;
    let sign: i32 = 0x80000000u32 as i32;
    let mut ix: i32;
    let mut s: i32;
    let mut q: i32;
    let mut m: i32;
    let mut t: i32;
    let mut i: i32;
    let mut r: u32;

    ix = x.to_bits() as i32;

    /* take care of Inf and NaN */
    if (ix as u32 & 0x7f800000) == 0x7f800000 {
        return x * x + x; /* sqrt(NaN)=NaN, sqrt(+inf)=+inf, sqrt(-inf)=sNaN */
    }

    /* take care of zero */
    if ix <= 0 {
        if (ix & !sign) == 0 {
            return x; /* sqrt(+-0) = +-0 */
        }
        if ix < 0 {
            #[allow(clippy::eq_op)] // This has special semantics and is not wrong.
            return (x - x) / (x - x); /* sqrt(-ve) = sNaN */
        }
    }

    /* normalize x */
    m = ix >> 23;
    if m == 0 {
        /* subnormal x */
        i = 0;
        while ix & 0x00800000 == 0 {
            ix <<= 1;
            i = i + 1;
        }
        m -= i - 1;
    }
    m -= 127; /* unbias exponent */
    ix = (ix & 0x007fffff) | 0x00800000;
    if m & 1 == 1 {
        /* odd m, double x to make it even */
        ix += ix;
    }
    m >>= 1; /* m = [m/2] */

    /* generate sqrt(x) bit by bit */
    ix += ix;
    q = 0;
    s = 0;
    r = 0x01000000; /* r = moving bit from right to left */

    while r != 0 {
        t = s + r as i32;
        if t <= ix {
            s = t + r as i32;
            ix -= t;
            q += r as i32;
        }
        ix += ix;
        r >>= 1;
    }

    /* use floating add to find out rounding direction */
    if ix != 0 {
        z = 1.0 - TINY; /* raise inexact flag */
        if z >= 1.0 {
            z = 1.0 + TINY;
            if z > 1.0 {
                q += 2;
            } else {
                q += q & 1;
            }
        }
    }

    ix = (q >> 1) + 0x3f000000;
    ix += m << 23;
    f32::from_bits(ix as u32)
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "simd"))]
#[inline(always)]
pub fn sqrt(value: f32) -> f32 {
    #[cfg(target_arch = "x86")]
    use core::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::*;

    unsafe { _mm_cvtss_f32(_mm_sqrt_ss(_mm_set_ss(value))) }
}
