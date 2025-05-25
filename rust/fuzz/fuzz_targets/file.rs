#![no_main]

use libfuzzer_sys::fuzz_target;
use warp::WarpFile;

fuzz_target!(|data: &[u8]| {
    if let Some(file) = WarpFile::from_bytes(data) {
        let file_bytes = file.to_bytes();
        if WarpFile::from_bytes(&file_bytes).is_none() {
            panic!("Failed to round-trip file");
        }
    }
});
