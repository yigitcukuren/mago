# Using the Linter

The Mago linter is a robust tool for analyzing and improving the quality of your PHP codebase. This document explains how to use the linter effectively, from basic usage to advanced workflows.

---

## Overview

The linter provides two primary functionalities:

- **Linting**: Analyze your codebase and report issues.
- **Fixing**: Automatically apply fixes to resolve reported issues.

Both functionalities work with customizable rules and plugins, allowing you to tailor the behavior of the linter to your project's needs.

---

## Basic Usage

### Linting Your Project

To analyze your project, run:

```bash
mago lint
```

This command scans your project, identifies issues, and reports them. By default, all enabled plugins and rules are used.

### Fixing Issues

To automatically fix issues identified during linting, run:

```bash
mago lint --fix
```

The `--fix` flag only applies safe fixes unless otherwise specified. Unsafe and potentially unsafe fixes require additional flags (explained below).

## Advanced Usage

### Filtering Results

If you only want to see issues that can be automatically fixed:

```bash
mago lint --fixable-only
```

> This is useful for previewing issues before running the `fix` command.

### Semantic Analysis

For a quick check of your project's syntax and semantics without running linting rules:

```bash
mago lint --semantics-only
```

This skips plugin-based rule checks and focuses solely on code correctness.

### Previewing Fixes

You can preview the changes that the `--fix` flag would make without applying them:

```bash
mago lint --fix --dry-run
```

This allows you to review planned fixes before making changes.

> Note: The `--fix` flag will cause `lint` command to exit with a non-zero status if any changes are planned.

## Configuration

The linter relies on your `mago.toml` file for configuration. This includes:

- **Plugins**: Enable or disable specific plugins.
- **Rules**: Customize individual rules with options and severity levels.

For more details, see the [Linter Configuration Guide](/getting-started/configuration?id=linter-configuration).
