#![no_main]
use libfuzzer_sys::fuzz_target;
use warp::r#type::Type;

fuzz_target!(|data: &[u8]| {
    if let Some(ty) = Type::from_bytes(data) {
        let ty_bytes = ty.to_bytes();
        Type::from_bytes(&ty_bytes).expect("Failed to round-trip type");
    }
});
