---
title: The Mago Linter
---

# The Mago Linter 🔎

The **Mago** Linter is a blazing-fast tool for finding and fixing stylistic issues, inconsistencies, and code smells in your PHP code. It helps you maintain a clean, readable, and consistent codebase with minimal effort.

---

## Linter vs. Analyzer

While they both find issues, the Analyzer and the Linter operate at different levels of understanding.

- **The Linter is a Style Editor:** It looks at the _structure_ of your code. It enforces your team's coding standards, flags redundant code, and suggests more modern syntax. It doesn't know what your code _does_, only what it _looks like_.

- **The Analyzer is a Fact-Checker:** It builds a deep, semantic understanding of your entire codebase. It knows what types your functions return, what properties your classes have, and what exceptions can be thrown. It finds logical impossibilities (e.g., "you're calling a method that doesn't exist on this object").

> **Analogy:** If your code were an essay, the **Linter** would be the grammar and style checker, while the **Analyzer** would be the editor who checks your facts and ensures your arguments are logical.

---

## The Semantic Checker

Mago processes files in three stages: **Parse** -> **Semantic Check** -> **Lint**.

The Mago parser is intentionally tolerant—it can parse syntax that the standard PHP compiler would reject, like features from a future PHP version.

The **Semantic Checker** is the crucial second step. Its job is to find syntax errors that Mago's parser allows but PHP would consider fatal. This includes things like:

- Invalid enum backing types (`enum Foo: array {}`)
- Using features not available in your configured PHP version (e.g., property hooks in PHP 8.1).

You can run just the first two stages using the `--semantics-only` (or `-s`) flag:

```sh
mago lint -s
```

This makes it a faster and more powerful replacement for `php -l`, allowing you to quickly validate the basic correctness of your files. It's a great way to start introducing Mago to your codebase incrementally.

---

## Key Features

- **🚀 Blazing Fast:** Written in Rust and built on a high-performance arena allocator, making it **_the fastest_** PHP linter available.
- **Highly Configurable:** Every rule can be enabled, disabled, or have its severity level adjusted.
- **Auto-Fixing:** Many rules provide automatic fixes that can be applied with the `--fix` flag.
- **Framework Integrations:** Includes specialized rules for frameworks like Symfony, Laravel, and PHPUnit.

---

## Dive In

- **[Usage](./usage.md)**: Learn how to run the linter from the command line.
- **[Integrations](./integrations.md)**: Enable framework-specific checks.
- **[Rules & Categories](./rules-and-categories.md)**: Discover all available rules.
- **[Configuration Reference](./configuration-reference.md)**: See all the available settings.
- **[Command Reference](./command-reference.md)**: A detailed guide to the `mago lint` command.
