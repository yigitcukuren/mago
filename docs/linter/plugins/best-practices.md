# Best Practices Plugin

The `best-practices` plugin enforces recommended coding practices to help you write better code.

## Rules

- [Combine Consecutive Issets](#combine-consecutive-issets)
- [Disallowed Functions](#disallowed-functions)
- [Excessive Nesting](#excessive-nesting)
- [Loop Does Not Iterate](#loop-does-not-iterate)
- [No Debug Symbols](#no-debug-symbols)
- [No Empty Loop](#no-empty-loop)
- [No Goto](#no-goto)
- [No Multi Assignment](#no-multi-assignment)
- [No Unused Parameters](#no-unused-parameters)
- [Use While Instead Of For](#use-while-instead-of-for)

---

### Combine Consecutive Issets

- Name: `best-practices/combine-consecutive-issets`
- Default Level: `warning`
- Description: This rule checks for consecutive `isset` calls that can be combined into a single call.

#### Configuration Options

This rule does not have any configurable options.

---

### Disallowed Functions

- Name: `best-practices/disallowed-functions`
- Default Level: `error`
- Description: This rule checks for the use of disallowed functions.

#### Configuration Options

##### Functions

An array of function names that are disallowed.

- Default: `[]`
- Type: `array of strings`
- Example:

  ```toml
  [[linter.rules]]
  name = "best-practices/disallowed-functions"
  function = ["shell_exec", "passthru"]
  ```

##### Extensions

An array of PHP extensions that are disallowed.

- Default: `[]`
- Type: `array of strings`
- Example:

  ```toml
  [[linter.rules]]
  name = "best-practices/disallowed-functions"
  extensions = ["curl", "libxml", "hash"]
  ```

---

### Excessive Nesting

- Name: `best-practices/excessive-nesting`
- Default Level: `warning`
- Description: This rule checks for excessive nesting in your code.

#### Configuration Options

##### Threshold

The maximum allowed nesting level.

- Default: `7`
- Type: `integer`
- Example:

  ```toml
  [[linter.rules]]
  name = "best-practices/excessive-nesting"
  threshold = 5
  ```

---

### Loop Does Not Iterate

- Name: `best-practices/loop-does-not-iterate`
- Default Level: `warning`
- Description: This rule checks for loops that do not iterate, typically caused by terminating condition that is always met on the first iteration.

#### Configuration Options

This rule does not have any configurable options.

---

### No Debug Symbols

- Name: `best-practices/no-debug-symbols`
- Default Level: `note`
- Description: This rule checks for debug symbols in your code that should not be present in production, such as `var_dump`, `print_r`, and `debug_backtrace`.

#### Configuration Options

This rule does not have any configurable options.

---

### No Empty Loop

- Name: `best-practices/no-empty-loop`
- Default Level: `note`
- Description: This rule checks for empty loops that do not perform any work.

#### Configuration Options

This rule does not have any configurable options.

---

### No Goto

- Name: `best-practices/no-goto`
- Default Level: `note`
- Description: This rule checks for the use of `goto` statements.

#### Configuration Options

This rule does not have any configurable options.

---

### No Multi Assignment

- Name: `best-practices/no-multi-assignment`
- Default Level: `warning`
- Description: This rule checks for multiple assignments in a single statement.

#### Configuration Options

This rule does not have any configurable options.

---

### No Unused Parameters

- Name: `best-practices/no-unused-parameters`
- Default Level: `note`
- Description: This rule checks for unused parameters in functions, methods, and closures.

#### Configuration Options

This rule does not have any configurable options.

---

### Use While Instead Of For

- Name: `best-practices/use-while-instead-of-for`
- Default Level: `warning`
- Description: This rule checks for `for` loops that can be replaced with `while` loops.

#### Configuration Options

This rule does not have any configurable options.
