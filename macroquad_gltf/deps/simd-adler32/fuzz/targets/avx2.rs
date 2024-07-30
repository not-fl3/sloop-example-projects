#![no_main]
#[macro_use]
extern crate libfuzzer_sys;

fuzz_target!(|data: &[u8]| {
  if let Some(imp) = simd_adler32::imp::avx2::get_imp() {
    imp(1, 0, data);
  }
});
