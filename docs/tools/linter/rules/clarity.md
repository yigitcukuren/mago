---
title: Clarity Rules
---

# Clarity Rules

This document details the rules available in the `Clarity` category.

## Available Rules

| Rule | Code |
| :--- | :---------- |
| No Empty | [`no-empty`](#no-empty) |
| No Hash Emoji | [`no-hash-emoji`](#no-hash-emoji) |
| No Multi Assignments | [`no-multi-assignments`](#no-multi-assignments) |
| No Shorthand Ternary | [`no-shorthand-ternary`](#no-shorthand-ternary) |
| Tagged FIXME | [`tagged-fixme`](#tagged-fixme) |
| Tagged TODO | [`tagged-todo`](#tagged-todo) |
| Valid Docblock | [`valid-docblock`](#valid-docblock) |

---

## <a id="no-empty"></a>`no-empty`

Detects the use of the `empty()` construct.

The `empty()` language construct can lead to ambiguous and potentially buggy code due to
loose and counterintuitive definition of emptiness. It fails to clearly convey
developer's intent or expectation, making it preferable to use explicit checks.



### Configuration

| Option | Type | Default |
| :--- | :--- | :--- |
| `enabled` | `boolean` | `true` |
| `level` | `string` | `"error"` |

### Examples

#### Correct Code

```php
<?php

if ($myArray === []) {
    // ...
}
```

#### Incorrect Code

```php
<?php

if (!empty($myArray)) {
    // ...
}
```

---

## <a id="no-hash-emoji"></a>`no-hash-emoji`

Discourages usage of the `#️⃣` emoji in place of the ASCII `#`.

While PHP allows the use of emojis in comments, it is generally discouraged to use them in place
of the normal ASCII `#` symbol. This is because it can confuse readers and may break external
tools that expect the normal ASCII `#` symbol.



### Configuration

| Option | Type | Default |
| :--- | :--- | :--- |
| `enabled` | `boolean` | `true` |
| `level` | `string` | `"warning"` |

### Examples

#### Correct Code

```php
<?php

# This is a comment

#[MyAttribute]
class Foo {}
```

#### Incorrect Code

```php
<?php

#️⃣ This is a comment

#️⃣[MyAttribute] <- not a valid attribute
class Foo {}
```

---

## <a id="no-multi-assignments"></a>`no-multi-assignments`

Flags any instances of multiple assignments in a single statement. This can lead to
confusion and unexpected behavior, and is generally considered poor practice.



### Configuration

| Option | Type | Default |
| :--- | :--- | :--- |
| `enabled` | `boolean` | `true` |
| `level` | `string` | `"warning"` |

### Examples

#### Correct Code

```php
<?php

$b = 0;
$a = $b;
```

#### Incorrect Code

```php
<?php

$a = $b = 0;
```

---

## <a id="no-shorthand-ternary"></a>`no-shorthand-ternary`

Detects the use of the shorthand ternary and elvis operators.

Both shorthand ternary operator (`$a ? : $b`) and elvis operator (`$a ?: $b`) relies on loose comparison.



### Configuration

| Option | Type | Default |
| :--- | :--- | :--- |
| `enabled` | `boolean` | `true` |
| `level` | `string` | `"warning"` |

### Examples

#### Correct Code

```php
<?php

$value = $foo ?? $default;
$value = $foo ? $foo : $default;
```

#### Incorrect Code

```php
<?php
$value = $foo ?: $default;
$value = $foo ? : $default;
```

---

## <a id="tagged-fixme"></a>`tagged-fixme`

Detects FIXME comments that are not tagged with a user or issue reference. Untagged FIXME comments
are not actionable and can be easily missed by the team. Tagging the FIXME comment with a user or
issue reference ensures that the issue is tracked and resolved.



### Configuration

| Option | Type | Default |
| :--- | :--- | :--- |
| `enabled` | `boolean` | `true` |
| `level` | `string` | `"warning"` |

### Examples

#### Correct Code

```php
<?php

// FIXME(@azjezz) This is a valid FIXME comment.
// FIXME(azjezz) This is a valid FIXME comment.
// FIXME(#123) This is a valid FIXME comment.
```

#### Incorrect Code

```php
<?php

// FIXME: This is an invalid FIXME comment.
```

---

## <a id="tagged-todo"></a>`tagged-todo`

Detects TODO comments that are not tagged with a user or issue reference. Untagged TODOs
can be difficult to track and may be forgotten. Tagging TODOs with a user or issue reference
makes it easier to track progress and ensures that tasks are not forgotten.



### Configuration

| Option | Type | Default |
| :--- | :--- | :--- |
| `enabled` | `boolean` | `true` |
| `level` | `string` | `"warning"` |

### Examples

#### Correct Code

```php
<?php

// TODO(@azjezz) This is a valid TODO comment.
// TODO(azjezz) This is a valid TODO comment.
// TODO(#123) This is a valid TODO comment.
```

#### Incorrect Code

```php
<?php

// TODO: This is an invalid TODO comment.
```

---

## <a id="valid-docblock"></a>`valid-docblock`

Checks for syntax errors in docblock comments. This rule is disabled by default because
it can be noisy and may not be relevant to all codebases.



### Configuration

| Option | Type | Default |
| :--- | :--- | :--- |
| `enabled` | `boolean` | `true` |
| `level` | `string` | `"note"` |

### Examples

#### Correct Code

```php
<?php

/**
 * @param int $a
 * @return int
 */
function foo($a) {
    return $a;
}
```

#### Incorrect Code

```php
<?php

/**
 @param int $a
    */
function foo($a) {
    return $a;
}
```

---
