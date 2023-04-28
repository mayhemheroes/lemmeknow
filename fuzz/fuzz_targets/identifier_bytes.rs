#![no_main]
use libfuzzer_sys::fuzz_target;
use lemmeknow::bytes::Identifier;

fuzz_target!(|data: &[u8]| {
    let identifier = Identifier::default();
    let result = identifier.identify(data);
    let _ = Identifier::to_json(&result);
});