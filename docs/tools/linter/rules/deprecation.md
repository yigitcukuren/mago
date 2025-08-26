---
title: Deprecation Rules
---

# Deprecation Rules

This document details the rules available in the `Deprecation` category.

## Available Rules

| Rule | Code |
| :--- | :---------- |
| Optional Parameter Before Required | [`optional-param-order`](#optional-param-order) |

---

## <a id="optional-param-order"></a>`optional-param-order`

                Detects optional parameters defined before required parameters in function-like declarations.
Such parameter order is considered deprecated; required parameters should precede optional parameters.


### Requirements

- **PHP Version:** PHP `>= 8.0.0`

### Configuration

| Option | Type | Default |
| :--- | :--- | :--- |
| `enabled` | `boolean` | `true` |
| `level` | `string` | `"warning"` |

### Examples

#### Correct Code

```php
<?php

function foo(string $required, ?string $optional = null): void {}
```

#### Incorrect Code

```php
<?php

function foo(?string $optional = null, string $required): void {}
```

---
