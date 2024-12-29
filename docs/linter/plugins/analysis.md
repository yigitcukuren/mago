# Analysis Plugin

The `analysis` plugin detects runtime problems in your PHP code, helping you catch critical issues before they happen.

## Rules

- [Inheritance](#inheritance)
- [Instantiation](#instantiation)
- [Undefined Constants](#undefined-constants)
- [Undefined Functions](#undefined-functions)

---

### Inheritance

- Name: `analysis/inheritance`
- Default Level: `error`
- Description: Detects problems with inheritance in your code, such as using invalid parent classes or interfaces, or circular inheritance dependencies.

#### Configuration Options

This rule does not have any configurable options.

---

### Instantiation

- Name: `analysis/instantiation`
- Default Level: `error`
- Description: Flags issues with object instantiation using `new Foo`, such as the class `Foo` not existing, or the class `Foo` being abstract or not being instantiable.

#### Configuration Options

This rule does not have any configurable options.

---

### Undefined Constants

- Name: `analysis/undefined-constants`
- Default Level: `error`
- Description: Identifies usage of undefined constants in your code.

#### Configuration Options

This rule does not have any configurable options.

---

### Undefined Functions

- Name: `analysis/undefined-functions`
- Default Level: `error`
- Description: Identifies usage of undefined functions in your code.

#### Configuration Options

This rule does not have any configurable options.
