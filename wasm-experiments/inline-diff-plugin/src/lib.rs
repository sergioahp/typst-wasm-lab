//! Inline diff Typst plugin experiment.
//!
//! Provides both a legacy string diff and a structured JSON diff that carries
//! line metadata for richer rendering in Typst.

use similar::{Algorithm, ChangeTag, TextDiff};
use serde::Serialize;

#[cfg(target_arch = "wasm32")]
wasm_minimal_protocol::initiate_protocol!();

#[derive(Serialize)]
struct SpanEntry {
    start: usize,
    end: usize,
}

#[derive(Serialize)]
struct LineEntry<'a> {
    line: usize,
    text: &'a str,
    spans: Vec<SpanEntry>,
}

#[derive(Serialize)]
struct DiffPayload<'a> {
    language: &'a str,
    before: Vec<LineEntry<'a>>,
    after: Vec<LineEntry<'a>>,
}

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

/// Produce a structured JSON diff suitable for Typst rendering.
#[cfg_attr(target_arch = "wasm32", wasm_minimal_protocol::wasm_func)]
pub fn inline_diff_segments(
    language: &[u8],
    before: &[u8],
    after: &[u8],
) -> Result<Vec<u8>, String> {
    let language = std::str::from_utf8(language)
        .map_err(|_| "language must be valid UTF-8".to_string())?;
    let before_text = std::str::from_utf8(before)
        .map_err(|_| "before must be valid UTF-8".to_string())?;
    let after_text = std::str::from_utf8(after)
        .map_err(|_| "after must be valid UTF-8".to_string())?;

    let diff = TextDiff::configure()
        .algorithm(Algorithm::Myers)
        .diff_lines(before_text, after_text);

    let mut before_lines = Vec::new();
    let mut after_lines = Vec::new();

    let mut before_line_no = 1usize;
    let mut after_line_no = 1usize;

    for change in diff.iter_all_changes() {
        let text_raw = change
            .value()
            .trim_end_matches('\n')
            .trim_end_matches('\r');
        match change.tag() {
            ChangeTag::Delete => {
                let len = text_raw.len();
                before_lines.push(LineEntry {
                    line: before_line_no,
                    text: text_raw,
                    spans: if len == 0 {
                        Vec::new()
                    } else {
                        vec![SpanEntry { start: 0, end: len }]
                    },
                });
                before_line_no += 1;
            }
            ChangeTag::Insert => {
                let len = text_raw.len();
                after_lines.push(LineEntry {
                    line: after_line_no,
                    text: text_raw,
                    spans: if len == 0 {
                        Vec::new()
                    } else {
                        vec![SpanEntry { start: 0, end: len }]
                    },
                });
                after_line_no += 1;
            }
            ChangeTag::Equal => {
                before_lines.push(LineEntry {
                    line: before_line_no,
                    text: text_raw,
                    spans: Vec::new(),
                });
                after_lines.push(LineEntry {
                    line: after_line_no,
                    text: text_raw,
                    spans: Vec::new(),
                });
                before_line_no += 1;
                after_line_no += 1;
            }
        }
    }

    let payload = DiffPayload {
        language,
        before: before_lines,
        after: after_lines,
    };

    serde_json::to_vec(&payload).map_err(|err| err.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

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

    #[test]
    fn structured_diff_highlights_changes() {
        let payload = inline_diff_segments(
            b"rust",
            b"let x = 1;\n",
            b"let x = 10;\n",
        )
        .unwrap();

        let json: Value = serde_json::from_slice(&payload).unwrap();
        assert_eq!(json["language"], "rust");

        let before_lines = json["before"].as_array().unwrap();
        let after_lines = json["after"].as_array().unwrap();

        assert_eq!(before_lines.len(), 1);
        assert_eq!(after_lines.len(), 1);

        assert_eq!(before_lines[0]["line"], 1);
        assert_eq!(after_lines[0]["line"], 1);

        assert_eq!(
            before_lines[0]["spans"].as_array().unwrap().len(),
            1
        );
        assert_eq!(
            after_lines[0]["spans"].as_array().unwrap().len(),
            1
        );
    }
}
