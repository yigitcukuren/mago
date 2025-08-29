---
title: Lexer & Parser 🧠
---

# The Mago Lexer & Parser 🧠

At the heart of **Mago** lies its high-performance, resilient Lexer and Parser. These are the foundational components that turn your raw PHP source code into a structured representation that all of Mago's other tools can understand.

- **The Lexer (Tokenizer):** Scans your code and breaks it down into a stream of individual "tokens" (e.g., `OpenTag`, `LiteralString`, `Whitespace`).
- **The Parser:** Takes the stream of tokens from the lexer and assembles them into a hierarchical **Abstract Syntax Tree (AST)** that represents the code's structure.

While these components are mostly used internally, Mago exposes them through the `mago ast` command. This is an incredibly powerful tool for:

- Debugging complex parsing issues in your code.
- Understanding exactly how PHP interprets a piece of syntax.
- Integrating with other tools that need a robust PHP parser, delegating the hard work to Mago.

## For Rust Developers

If you're building your own tools in Rust and need a high-performance PHP parser, you can use Mago's core crates directly:

- **[`mago-syntax`](https://crates.io/crates/mago-syntax):** The crate containing the Lexer, Parser, and all AST node definitions, along with utilities for working with the AST.
- **[`mago-names`](https://crates.io/crates/mago-names):** The crate for resolving symbol names (e.g., turning a local class name into its fully qualified name).
