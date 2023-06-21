#![no_main]

use libfuzzer_sys::fuzz_target;
use packit::{PackItArchiveDecoder, PackItResult};
use std::hint::black_box;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    fn do_case(data: &[u8]) -> PackItResult<()> {
        let dec = PackItArchiveDecoder::load(data)?;
        for file in dec {
            let file = file?;
            assert!(!file.name().is_empty());
            assert!(file.data().len() < data.len());
            assert!(file.data().len() < file.total_size());
            assert!(file.total_size() < data.len());
        }
        Ok(())
    }

    let _ = black_box(do_case(data));
});
