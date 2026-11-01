#![no_main]

libfuzzer_sys::fuzz_target!(|data: &[u8]| block_fuzz::run("llsd", data));
