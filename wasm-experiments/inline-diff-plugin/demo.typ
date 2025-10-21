#import "../../typst/inline-diff.typ": inline_diff

#let diffmod = plugin("target/wasm32-unknown-unknown/release/inline_diff_plugin.wasm")

= Inline Diff Plugin Demo

#inline_diff(diffmod, "let x = 1;\nprint(x);\n", "let x = 10;\nprint(x);\n", lang: "rust")

#inline_diff(diffmod, "def nonsense(n):\n  for x in range(len(n)):\n    print(x)\n", "def nonsense(n):\n  for x in range(len(n)):\n    print(\"Now this is different, isn't it?\", x)\n", lang: "python")
