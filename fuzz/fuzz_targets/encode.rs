#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use packit::{PackItArchiveEncoder, PackItFile, PackItResult};
use std::hint::black_box;
use std::io::{self, Write};

#[derive(Arbitrary, Clone, Debug)]
struct FuzzFile<'a> {
    name: &'a str,
    data: &'a [u8],
}

fuzz_target!(|files: Vec<FuzzFile<'_>>| {
    fn do_case(files: Vec<FuzzFile<'_>>) -> PackItResult<()> {
        let mut dst = io::sink();
        let mut enc = PackItArchiveEncoder::new(&mut dst)?;
        for file in files.iter() {
            let f = PackItFile::new(file.name, file.data)?;
            enc.write_file(&f)?;
        }
        dst.flush()?;
        Ok(())
    }

    let _ = black_box(do_case(files));
});
