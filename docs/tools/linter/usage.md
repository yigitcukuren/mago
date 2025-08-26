---
title: Usage
---

# Usage

The `mago lint` command is the entry point for running the linter.

## Linting Your Project

To lint all the source files defined in your `mago.toml` configuration, simply run:

```sh
mago lint
```

Mago will scan your project in parallel and report any issues it finds.

## Auto-Fixing Issues

Many lint rules provide automatic fixes. To apply them, use the `--fix` flag:

```sh
mago lint --fix
```

This will modify your files in place. To see what changes would be made without applying them, you can combine it with `--dry-run`:

```sh
mago lint --fix --dry-run
```

## Running Specific Rules

If you want to run only a specific set of rules, use the `--only` flag. This is great for incrementally introducing new rules to a project.

```sh
# Run only these two rules
mago lint --only no-empty,use-compound-assignment
```
