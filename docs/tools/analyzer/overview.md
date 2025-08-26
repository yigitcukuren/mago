---
title: The Mago Analyzer
---

# The Mago Analyzer 🔬

The **Mago** Analyzer is a powerful static analysis engine that finds logical errors, type mismatches, and potential bugs in your code _before_ you run it. It's the core of Mago's ability to ensure your code is not just well-styled, but also correct and robust.

---

## Analyzer vs. Linter: What's the Difference?

While they both find issues, the Analyzer and the Linter operate at different levels of understanding.

- **The Linter is a Style Editor:** It looks at the _structure_ of your code. It checks for stylistic issues, inconsistencies, and code smells (e.g., "this `if` statement has no `else`"). It doesn't know what your code _does_.

- **The Analyzer is a Fact-Checker:** It builds a deep, semantic understanding of your entire codebase. It knows what types your functions return, what properties your classes have, and what exceptions can be thrown. It finds logical impossibilities (e.g., "you're calling a method that doesn't exist on this object").

> **Analogy:** If your code were an essay, the **Linter** would be the grammar and style checker, while the **Analyzer** would be the editor who checks your facts and ensures your arguments are logical.

---

## Key Features

- **🚀 Blazing Fast:** Written in Rust and highly parallelized to analyze large codebases in seconds.
- **Deep Type Inference:** Understands your code's types, even without full type hinting.
- **Comprehensive Checks:** Catches a wide range of issues, from simple null pointer risks to complex logical errors.
- **Heuristic Engine:** Provides advice on code quality issues that aren't strict errors but could indicate potential bugs.

---

## Dive In

- **[Usage](./usage.md)**: Learn how to run the analyzer.
- **[Configuration Reference](./configuration-reference.md)**: See all the available options to customize the analysis.
- **[Command Reference](./command-reference.md)**: A detailed guide to the `mago analyze` command and its flags.
