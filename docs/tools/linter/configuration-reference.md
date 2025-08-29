---
title: Configuration Reference
---

# Configuration Reference

The **Mago** Linter is configured in your `mago.toml` file under the `[linter]` and `[linter.rules]` tables.

```toml
# Example linter configuration
[linter]
integrations = ["symfony", "phpunit"]
excludes = ["src/Generated/"]

[linter.rules]
# Disable a rule completely
ambiguous-function-call = { enabled = false }

# Change a rule's severity level
no-else-clause = { level = "warning" }

# Configure a rule's specific options
cyclomatic-complexity = { threshold = 20 }
```

## `[linter]` Table

| Option         | Type       | Default | Description                                                                  |
| :------------- | :--------- | :------ | :--------------------------------------------------------------------------- |
| `excludes`     | `string[]` | `[]`    | A list of glob patterns to exclude from linting.                             |
| `integrations` | `string[]` | `[]`    | A list of framework integrations to enable (e.g., `"symfony"`, `"laravel"`). |

## `[linter.rules]` Table

This table allows you to configure individual lint rules. Each key is the rule's code (in `kebab-case`).

### Common Rule Options

All rules accept two common options:

- `enabled`: A boolean (`true` or `false`) to enable or disable the rule.
- `level`: A string to set the issue severity. Options are `"error"`, `"warning"`, `"help"`, and `"note"`.

### Rule-Specific Options

Some rules have additional configuration options. For example, `cyclomatic-complexity` has a `threshold`:

```toml
[linter.rules]
cyclomatic-complexity = { level = "error", threshold = 15 }
```

To find the specific options available for any rule, the best and most up-to-date method is to use the `--explain` command:

```sh
mago lint --explain cyclomatic-complexity
```
