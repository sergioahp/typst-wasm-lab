//! Basic Typst WASM plugin experiment.
//!
//! Exports a single function that uppercases incoming UTF-8 text. Invalid UTF-8
//! sequences are passed through unchanged.

#[cfg(target_arch = "wasm32")]
wasm_minimal_protocol::initiate_protocol!();

/// Uppercase a text payload. Non UTF-8 inputs are returned as-is.
#[cfg_attr(target_arch = "wasm32", wasm_minimal_protocol::wasm_func)]
pub fn uppercase(input: &[u8]) -> Vec<u8> {
    match std::str::from_utf8(input) {
        Ok(text) => text.to_uppercase().into_bytes(),
        Err(_) => input.to_vec(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uppercase_ascii() {
        assert_eq!(
            uppercase(b"Hello Typst"),
            b"HELLO TYPST".to_vec()
        );
    }

    #[test]
    fn uppercase_handles_utf8() {
        assert_eq!(
            String::from_utf8(uppercase("ärger".as_bytes())).unwrap(),
            "ÄRGER"
        );
    }

    #[test]
    fn invalid_utf8_roundtrips() {
        let data = [0xff, 0xfe, 0xfd];
        assert_eq!(uppercase(&data), data);
    }
}
