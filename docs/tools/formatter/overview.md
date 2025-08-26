---
title: The Mago Formatter
---

# The Mago Formatter ✨

The **Mago** Formatter is a powerful, opinionated code formatter for PHP that ensures your entire codebase adheres to a single, consistent style.

Its primary goal is to end debates over code style. By automating the formatting process, it allows you and your team to stop worrying about whitespace and focus on what truly matters: building great software.

---

## How It Works

Mago takes a "parse-and-reprint" approach, inspired by modern formatters like Prettier and `rustfmt`.

1.  It first parses your PHP code into a detailed Abstract Syntax Tree (AST).
2.  It then **throws away your original formatting**, including all newlines, spacing, and indentation.
3.  Finally, it **reprints the AST from scratch** according to its own set of consistent, PSR-12-compliant rules.

This process guarantees that the output is always 100% consistent, regardless of the input style. Most importantly, it does this without ever changing the behavior of your code.

---

## Key Features

- **🚀 Blazing Fast:** Written in Rust for maximum performance, making it one of the fastest PHP formatters available.
- **Opinionated & Consistent:** Ends style debates by enforcing a single, unified coding style across your entire project.
- **PSR-12 Compliant:** Follows the widely accepted PSR-12 coding standard.
- **Safe:** The formatter is designed to never alter the runtime behavior of your code.

---

## Dive In

- **[Usage](./usage.md)**: Learn how to run the formatter from the command line.
- **[Configuration Reference](./configuration-reference.md)**: See all the available options to customize the formatter's behavior.
