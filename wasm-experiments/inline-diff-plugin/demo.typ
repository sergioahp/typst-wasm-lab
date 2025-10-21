#let diffmod = plugin("target/wasm32-unknown-unknown/release/inline_diff_plugin.wasm")

#let diff(before, after) = str(
  diffmod.inline_diff(
    bytes(before),
    bytes(after),
  )
)

= Inline Diff Plugin Demo

+ #diff("cat", "cart")
  // expected: ca{+r+}t

+ #diff("let x = 1;", "let x = 10;")
  // expected: let x = 1{+0+};

#let err = diffmod.inline_diff(bytes("\u{0}"), bytes("\u{1}"))
#let err-text = str(err)

strong("Error message:") \
#err-text
