# Linter Plugins

Mago's linter supports a variety of plugins to improve code quality, enforce best practices, and detect issues. Below is a list of available plugins. Click on a plugin to learn more about its rules and configuration options.

---

## Available Plugins

- [Analysis](linter/plugins/analysis.md): Detect runtime problems such as inheritance issues, undefined constants, and functions.
- [Best Practices](linter/plugins/best-practices.md): Enforce recommended coding practices.
- [Comment](linter/plugins/comment.md): Manage and validate comments in your code.
- [Consistency](linter/plugins/consistency.md): Ensure consistent code style and structure.
- [Deprecation](linter/plugins/deprecation.md): Identify usage of deprecated features.
- [Laravel](linter/plugins/laravel.md) _(Optional)_: Enforce rules specific to Laravel projects.
- [Migration](linter/plugins/migration.md): Help migrate code to newer PHP versions.
- [Naming](linter/plugins/naming.md): Enforce consistent naming conventions.
- [PHPUnit](linter/plugins/php-unit.md) _(Optional)_: Improve PHPUnit test code quality.
- [Redundancy](linter/plugins/redundancy.md): Detect and eliminate redundant code.
- [Safety](linter/plugins/safety.md): Identify unsafe coding patterns.
- [Strictness](linter/plugins/strictness.md): Apply strict coding rules for rigorous projects.
- [Symfony](linter/plugins/symfony.md) _(Optional)_: Enforce rules for Symfony projects.

---

To enable a plugin, add it to the `plugins` list in your `mago.toml` file. For example:

```toml
[linter]
default_plugins = false
plugins = ["analysis", "best-practices", "symfony"]
```
