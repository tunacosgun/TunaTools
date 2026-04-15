#![cfg_attr(not(feature = "tuna_all"), no_main)]

#[path = "tuna_common.rs"]
mod tuna_common;

use libfuzzer_sys::Corpus;

pub fn do_fuzz(case: &[u8]) -> Corpus {
    tuna_common::fuzz_json_parser(case)
}

#[cfg(not(feature = "tuna_all"))]
libfuzzer_sys::fuzz_target!(|case: &[u8]| -> Corpus { do_fuzz(case) });
