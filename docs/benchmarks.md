---
title: Benchmarks  ⚡️
---

Performance is a core feature of **Mago**. Every component, from the parser to the analyzer, is designed to be as fast as possible.

We regularly benchmark Mago against other popular tools in the PHP ecosystem to ensure it remains the fastest toolchain available.

---

## Our Performance Promise

At its core, Mago is built on a simple philosophy: **it must be the fastest.**

This is not just a goal; it's a guarantee. If any tool listed in our benchmarks ever outperforms Mago in a like-for-like comparison, we consider it a high-priority bug that needs to be fixed. Speed is a feature, and we promise to always deliver it.

---

## `mago analyze`

This benchmark measures the time it takes to perform a full static analysis on a large codebase.

| Tool     | Time (seconds) | Memory (MB) |
| :------- | :------------- | :---------- |
| **Mago** |                |             |
| PHPStan  |                |             |
| Psalm    |                |             |

---

## `mago lint`

This benchmark measures the time it takes to lint an entire codebase for stylistic issues.

| Tool         | Time (seconds) | Memory (MB) |
| :----------- | :------------- | :---------- |
| **Mago**     |                |             |
| PHP-CS-Fixer |                |             |
| Pint         |                |             |

---

## `mago fmt`

This benchmark measures the time it takes to format an entire codebase.

| Tool         | Time (seconds) | Memory (MB) |
| :----------- | :------------- | :---------- |
| **Mago**     |                |             |
| PHP-CS-Fixer |                |             |
| Pint         |                |             |
