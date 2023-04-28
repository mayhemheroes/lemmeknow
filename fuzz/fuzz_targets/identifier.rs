#![no_main]
use libfuzzer_sys::fuzz_target;
use lemmeknow::Identifier;

fuzz_target!(|data: &[u8]| {
    let input = String::from_utf8_lossy(data);
    let identifier = Identifier::default();
    let result = identifier.identify(&input);
    let _ = Identifier::to_json(&result);
});