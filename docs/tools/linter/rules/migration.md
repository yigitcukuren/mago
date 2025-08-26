---
title: Migration Rules
---

# Migration Rules

This document details the rules available in the `Migration` category.

## Available Rules

| Rule | Code |
| :--- | :---------- |
| Explicit Octal | [`explicit-octal`](#explicit-octal) |
| Str Contains | [`str-contains`](#str-contains) |
| Str Starts With | [`str-starts-with`](#str-starts-with) |

---

## <a id="explicit-octal"></a>`explicit-octal`

Detects implicit octal numeral notation and suggests replacing it with explicit octal numeral notation.


### Requirements

- **PHP Version:** PHP `>= 8.1.0`

### Configuration

| Option | Type | Default |
| :--- | :--- | :--- |
| `enabled` | `boolean` | `true` |
| `level` | `string` | `"warning"` |

### Examples

#### Correct Code

```php
<?php

$a = 0o123;
```

#### Incorrect Code

```php
<?php

$a = 0123;
```

---

## <a id="str-contains"></a>`str-contains`

Detects `strpos($a, $b) !== false` comparisons and suggests replacing them with `str_contains($a, $b)`
for improved readability and intent clarity.


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

$a = 'hello world';
$b = 'world';

if (str_contains($a, $b)) {
    echo 'Found';
}
```

#### Incorrect Code

```php
<?php

$a = 'hello world';
$b = 'world';

if (strpos($a, $b) !== false) {
    echo 'Found';
}
```

---

## <a id="str-starts-with"></a>`str-starts-with`

Detects `strpos($a, $b) === 0` comparisons and suggests replacing them with `str_starts_with($a, $b)`
for improved readability and intent clarity.


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

$a = 'hello world';
$b = 'hello';
if (str_starts_with($a, $b)) {
    echo 'Found';
}
```

#### Incorrect Code

```php
<?php

$a = 'hello world';
$b = 'hello';
if (strpos($a, $b) === 0) {
echo 'Found';
}
```

---
