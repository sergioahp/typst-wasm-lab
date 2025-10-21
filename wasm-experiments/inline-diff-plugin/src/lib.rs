//! Inline diff Typst plugin experiment.
//!
//! Generates a lightweight markup stream resembling `[-old-]{+new+}` around
//! character-level additions and deletions.

use similar::{Algorithm, TextDiff};

#[cfg(target_arch = "wasm32")]
wasm_minimal_protocol::initiate_protocol!();

/// Compute an inline diff between `before` and `after`.
#[cfg_attr(target_arch = "wasm32", wasm_minimal_protocol::wasm_func)]
pub fn inline_diff(before: &[u8], after: &[u8]) -> Vec<u8> {
    let before_text = std::str::from_utf8(before);
    let after_text = std::str::from_utf8(after);

    let (before_ok, after_ok) = match (before_text, after_text) {
        (Ok(b), Ok(a)) => (b, a),
        _ => return b"inputs must be valid UTF-8".to_vec(),
    };

    let diff = TextDiff::configure()
        .algorithm(Algorithm::Myers)
        .diff_chars(before_ok, after_ok);

    let mut output = String::with_capacity(before.len() + after.len());

    for change in diff.iter_all_changes() {
        match change.tag() {
            similar::ChangeTag::Equal => output.push_str(change.value()),
            similar::ChangeTag::Delete => {
                output.push_str("[-");
                output.push_str(change.value());
                output.push_str("-]");
            }
            similar::ChangeTag::Insert => {
                output.push_str("{+");
                output.push_str(change.value());
                output.push_str("+}");
            }
        }
    }

    output.into_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unchanged_round_trip() {
        assert_eq!(
            inline_diff("same".as_bytes(), "same".as_bytes()),
            b"same"
        );
    }

    #[test]
    fn detects_insertions_and_deletions() {
        let diff = String::from_utf8(inline_diff(b"cat", b"cart")).unwrap();
        assert_eq!(diff, "ca{+r+}t");
    }

    #[test]
    fn handles_replace() {
        let diff = String::from_utf8(inline_diff(b"bat", b"cat")).unwrap();
        assert_eq!(diff, "[-b-]{+c+}at");
    }

    #[test]
    fn rejects_non_utf8() {
        assert_eq!(
            inline_diff(&[0xff], &[0xfe]),
            b"inputs must be valid UTF-8"
        );
    }
}
