// Store highlighted code in a variable, then display its value and representation.
#let snippet = [
  ```rust
  fn greet() {
    println!("Hello, Typst!");
  }
  ```
]

#snippet

#repr(snippet)
