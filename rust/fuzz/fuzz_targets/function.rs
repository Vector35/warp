#![no_main]

use libfuzzer_sys::fuzz_target;
use warp::signature::function::Function;

fuzz_target!(|data: &[u8]| {
    if let Some(func) = Function::from_bytes(data) {
        let func_bytes = func.to_bytes();
        Function::from_bytes(&func_bytes).expect("Failed to round-trip function");
    }
});
