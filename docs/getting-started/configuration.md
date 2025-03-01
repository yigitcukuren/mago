# Configuration

Mago can be configured using a `mago.toml` file, environment variables, or command-line arguments.

## Configuration File

The configuration file is a TOML file named `mago.toml`. Mago will search for this file in the workspace directory.
You can specify a different configuration file using the `--config` command-line argument.

Here's an example of a `mago.toml` file:

```toml
[source]
paths = ["src", "tests"]
excludes = ["**/src/**/*.generated.php", "tests/fixtures"]

[format]
print_width = 80

[linter]
plugins = ["php-unit"]

[[linter.rules]]
name = "best-practices/excessive-nesting"
level = "warning"
threshold = 5
```

## Environment Variables

Mago supports the following environment variables:

- `MAGO_PHP_VERSION`: The PHP version to use for linting and formatting.
- `MAGO_THREADS`: The number of threads to use for parallel processing.
- `MAGO_STACK_SIZE`: The stack size for each thread.
- `MAGO_LOG`: The log level for Mago (`"error"`, `"warning"`, `"info"`, `"debug"`, or `"trace"`).
- `MAGO_ALLOW_UNSUPPORTED_PHP_VERSION`: Whether to allow unsupported PHP versions (`"true"` or `"false"`).

## Command-Line Arguments

Mago supports the following global command-line arguments:

- `--workspace`: The path to the workspace directory. This is the root directory of your project. If not specified, defaults to the current working directory. This argument also controls where the configuration file is loaded from. This value overrides the `source.workspace` setting in the configuration file.
- `--config`: The path to the configuration file. If not specified, Mago will search for a` mago.toml` file in the workspace directory.
- `--php-version`: The PHP version to use for parsing and analysis. This should be a valid PHP version number (e.g., `"8.3"`, `"8.4"`). This value overrides the `php_version` setting in the configuration file and the `MAGO_PHP_VERSION` environment variable.
- `--threads`: The number of threads to use for parallel processing. This value overrides the `threads` setting in the configuration file and the `MAGO_THREADS` environment variable.
- `--allow-unsupported-php-version`: Whether to allow unsupported PHP versions. This value overrides the `allow_unsupported_php_version` setting in the configuration file and the `MAGO_ALLOW_UNSUPPORTED_PHP_VERSION` environment variable.

## Configuration Options

### General Settings

#### Threads

The `threads` option specifies the number of threads to use for parallel processing.

- Default: The number of logical CPUs on the system.
- Type: `integer`
- Example:

  ```toml
  threads = 4
  ```

#### Stack Size

The `stack_size` option specifies the stack size for each thread.

- Default: `36_864_000` (36MB)
- Type: `string`
- Example:

  ```toml
  stack_size = "262144"
  ```

#### PHP Version

The `php_version` option specifies the PHP version to use for linting and formatting.

- Default: `8.3`
- Type: `string`
- Example:

  ```toml
  php_version = "8.4"
  ```

#### Allow Unsupported PHP Version

The `allow_unsupported_php_version` option specifies whether to allow unsupported PHP versions.

- Default: `false`
- Type: `boolean`
- Example:

  ```toml
  allow_unsupported_php_version = true
  ```

### Source Configuration

The `[source]` section controls how Mago discovers and processes files.

#### Workspace Directory

The `workspace` setting specifies the root directory of your project. This is where Mago will search for PHP source files, unless specific paths are defined in the `paths` setting.

- Default: Current working directory
- Type: `string`
- Example:

  ```toml
  workspace = "."
  ```

Unlike the `--workspace` command-line argument, this setting does not affect where the configuration file is loaded from, as the configuration file has already been loaded at this point. If the `--workspace` command-line argument is provided, this setting will be overridden.

#### Paths

The `paths` option specifies which directories to scan for files. If no paths are defined, Mago will scan the entire workspace directory.

- Default: `[]`
- Type: `array of strings`
- Example:

  ```toml
  paths = ["src", "tests"]
  ```

Use this to limit Mago’s search scope to specific directories.

**Note**: If any of the `source.paths` are relative, they are considered to be relative to the `source.workspace` directory.

#### Includes

The includes option is for adding extra files or directories that are not part of the main source paths but still need to be scanned. This is useful for external dependencies, such as third-party libraries or vendor directories.

- Default: `[]`
- Type: `array of strings`
- Example:

  ```toml
  includes = ["vendor"]
  ```

**Note**: If any of the `source.includes` paths are relative, they are considered to be relative to the `source.workspace` directory.

#### Excludes

The excludes option allows you to define patterns or paths to skip during file discovery. Patterns can include wildcards (\*) for more flexibility.

- Default: `[]`
- Type: `array of strings`
- Example:

  ```toml
  excludes = ["tests/fixtures", "**/src/**/*.generated.php"]
  ```

**Note**: If any of the `source.excludes` paths are relative, they are considered to be relative to the `source.workspace` directory.

#### Extensions

The extensions option specifies the file extensions to include in the search.

- Default: `["php"]`
- Type: `array of strings`
- Example:

  ```toml
  extensions = ["php", "php8"]
  ```

### Formatter Configuration

The `[format]` section customizes how Mago formats your PHP code, including settings like line width, tab width, and indentation style.
In addition to common formatting options, you can specify an optional `excludes` key to define patterns that should be excluded from formatting.
This allows you to, for example, skip formatting for generated files or other patterns while still processing them for linting.

For more details on the available formatter settings, see the [Formatter Settings](/formatter/settings.md) page.

- Default: `{}`
- Type: `table`
- Example:

  ```toml
  [format]
  excludes = ["**/src/**/*.generated.php"]
  print_width = 80
  tab_width = 2
  ```

**Note**: If any of the `format.excludes` paths are relative, they are considered to be relative to the `source.workspace` directory.

### Linter Configuration

The `[linter]` section controls the behavior of Mago's linter. This includes enabling plugins,
setting rule configurations, and determining the level of severity for specific rules.

Below is a detailed explanation of the available options.

#### Default Plugins

This setting specifies whether to enable the default set of linter plugins.

- Default: `true`
- Type: `boolean`
- Example:

  ```toml
  [linter]
  default_plugins = false
  ```

#### Plugins

The `plugins` option allows you to specify which linter plugins to enable.

- Default: `[]`
- Type: `array of strings`
- Example:

  ```toml
  [linter]
  plugins = ["symfony", "laravel", "php-unit"]
  ```

#### Rules

The `rules` option lets you configure specific rules for the linter. Each rule can have its own severity level, as well as additional options.

Each rule is specified as an array entry under `[[linter.rules]]`, with the following properties:

- `name`: The name of the rule in `{plugin}/{rule}` format.
- `level`: The severity level of the rule (`"error"`, `"warning"`, `"info"`, `"help"`, or `"off"`).
- `...`: Additional options specific to the rule.

Here's an example of configuring a rule:

```toml
[[linter.rules]]
name = "best-practices/excessive-nesting"
level = "warning"
threshold = 5

[[linter.rules]]
name = "safety/no-ffi"
level = "off"
```

For more information on the available plugins and rules, see the [Linter Plugins](/linter/plugins/) page.
