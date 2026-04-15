#![cfg_attr(not(feature = "tuna_all"), no_main)]

#[path = "tuna_common.rs"]
mod tuna_common;

use libfuzzer_sys::Corpus;
use tuna_js_syntax::JsFileSource;

pub fn do_fuzz(case: &[u8]) -> Corpus {
    let parse_type = JsFileSource::tsx();
    tuna_common::fuzz_js_formatter_with_source_type(case, parse_type)
}

#[cfg(not(feature = "tuna_all"))]
libfuzzer_sys::fuzz_target!(|case: &[u8]| -> Corpus { do_fuzz(case) });
