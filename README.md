# Mago: The Oxidized PHP Toolchain

**Mago** (derived from [Mago (Punic: 𐤌𐤂‬𐤍‬, MGN)](<https://en.wikipedia.org/wiki/Mago_(agricultural_writer)>), a renowned Carthaginian figure) is a toolchain for PHP that aims to provide a set of tools to help developers write better code. Mago draws inspiration from the Rust programming language and its ecosystem, striving to bring similar convenience, reliability, and a great developer experience to the PHP world.

---

_Note: This project was previously named “Fennec” before being rebranded due to a naming conflict._

## Disclaimer

> [!WARNING]
> Mago is in an early stage of development. Many features are not yet implemented, and existing functionality may change, break, or stop working without notice.
> While we are not actively promoting or advertising the project, we are working in public to share our progress with the community.

## Roadmap

### Core Functionality

- [x] String Interning: [`crates/interner`](crates/interner)
- [x] Lexer: [`crates/lexer`](crates/lexer) [`crates/token`](crates/token)
- [x] AST: [`crates/ast`](crates/ast)
- [x] Parser: [`crates/parser`](crates/parser)
- [x] Source Management: [`crates/source`](crates/source)
- [x] AST Traversal / Walk: [`crates/traverser`](crates/traverser) [`crates/walker`](crates/walker)
- [x] Name Resolution: [`crates/names`](crates/names)
- [x] Code Fixer: [`crates/fixer`](crates/fixer)
- [x] Error Reporting: [`crates/reporting`](crates/reporting)
- [x] Semantic Analysis: [`crates/semantics`](crates/semantics)
- [x] Symbol Table: [`crates/symbol-table`](crates/symbol-table)
- [x] Linter: [`crates/linter`](crates/linter)
- [x] Services: [`crates/service`](crates/service)
- [x] String Case Conversion: [`crates/casing`](crates/casing)
- [x] Reflections: [`crates/reflection`](crates/reflection)
- [x] Reflector: [`crates/reflector`](crates/reflector), [`crates/scanner`](crates/scanner)
- [x] Type Inference: [`crates/typing`](crates/typing)
- [x] Formatter: [`crates/formatter`](crates/formatter)
- [ ] Static Analyzer
- [ ] Refactoring
- [ ] Code Generation
- [ ] Documentation Generation
- [x] Docblock Parser [`crates/docblock`](crates/docblock)

### Tooling

- [x] CLI Tool: [`crates/cli`](crates/cli)
- [x] Web Assembly (WASM) Interface: [`crates/wasm`](crates/wasm)
- [ ] Web Interface
- [ ] Language Server Protocol
- [ ] Editor Integration

## PHP Version Compatibility

Currently, **Mago** is built around PHP 8.3 and also supports PHP 8.4. While the linter and formatter may work with earlier versions of PHP, **we cannot guarantee compatibility**. They might suggest fixes or write code that is only compatible with PHP 8.3 and later.

At this stage, there is no option to select a PHP version target, which means Mago operates under the assumption of modern PHP versions. In the future, we plan to introduce support for selecting a PHP version target. Once implemented, this feature should enable better compatibility with earlier PHP versions, potentially down to PHP 8.0 or even earlier.

If you're working with PHP versions prior to 8.3, please proceed with caution and review suggested fixes or formatted code to ensure compatibility with your version.

## Installation

### One-Line Installation (Recommended)

To quickly install the latest release of Mago for macOS or Linux, use the following command:

#### Using `curl`:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://carthage.software/mago.sh | bash
```

#### Using `wget`:

```bash
wget -qO- https://carthage.software/mago.sh | bash
```

#### Custom Installation Directory

To specify a custom directory for the binary, use the --install-dir option:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://carthage.software/mago.sh | bash -s -- --install-dir="/.bin"
```

If the directory is not in your `PATH`, the script will provide instructions to add it.

#### Installing with `sudo`

If you need to install Mago system-wide, you can use `sudo` with the installation command:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://carthage.software/mago.sh | sudo bash
```

### Pre-compiled Binaries

You can find precompiled binaries for various platforms on our [Releases page](https://github.com/carthage-software/mago/releases).
Simply download the archive for your platform, extract it, and place the mago binary somewhere in your `PATH`.

### Installation via Cargo

If you have Rust installed, you can install Mago using Cargo:

```bash
cargo install mago
```

### Installation from source

To install Mago from source, you can clone the repository and build the project using Cargo:

```bash
git clone https://github.com/carthage-software/mago
cd mago
cargo install --path .
```

## Usage

For a quick start, you can refer to the example configuration files provided:

- Simple configuration: [`examples/mago.toml`](examples/mago.toml)
- Full configuration with all possible options: [`examples/mago-full.toml`](examples/mago-full.toml)

You can try Mago by navigating to the [`examples`](examples) directory and running the linter on the sample PHP files:

```bash
cd examples
mago lint
mago fmt
```

This will analyze the PHP files located in the [`examples/src/`](examples/src) directory and display any linting errors.

## How You Can Help

Mago is a community-driven project, and we’d love for you to join us! Here are some ways you can contribute:

- _Suggest Ideas_: Have an idea for Mago? We’re open to suggestions that can make the toolchain even better!
- _Help Write Documentation_: Clear, user-friendly documentation is key to making Mago accessible to everyone. If you enjoy writing or organizing docs, we'd love your help.
- _Contribute Code_: Join us in building Mago! Please discuss any feature or bug fixes in the issues first to ensure we coordinate effectively.
- _Sponsor the Project_: If you’d like to support Mago financially, consider sponsoring [@azjezz](https://github.com/azjezz). Every contribution helps!
- _Help with Art_: Mago could use a logo! We’d appreciate the help of a skilled artist to create an original logo for Mago. (Please note that AI-generated art will not be accepted.)

## Join the Mago Community

Got questions, feedback, or ideas? Join the Mago community on Discord to connect with other developers and stay up-to-date.

[Join Here](https://discord.gg/mwyyjr27eu)

## Inspiration

Mago is inspired by several tools and projects that have significantly contributed to the development community:

- [Clippy](https://github.com/rust-lang/rust-clippy): A collection of lints to catch common mistakes and improve your Rust code.
- [OXC](https://github.com/oxc-project/oxc/): A JavaScript toolchain written in Rust.
- [php-rust-tools/parser](https://github.com/php-rust-tools/parser/): A PHP parser written in Rust, which influenced our parsing approach.
- [slackhq/hakana](https://github.com/slackhq/hakana/): A static analysis tool for HackLang written in Rust, by the creator of [Psalm](https://github.com/vimeo/psalm).

These tools have inspired us and helped shape Mago's design and functionality.

## Acknowledgements

We would like to acknowledge the following PHP tools that have greatly helped hundreds of thousands of PHP developers in their journey,
ourselves included:

- [PHP CS Fixer](https://github.com/PHP-CS-Fixer/PHP-CS-Fixer): A tool to automatically fix PHP Coding Standards issues.
- [Psalm](https://github.com/vimeo/psalm): A static analysis tool for finding errors in PHP applications.
- [PHPStan](https://github.com/phpstan/phpstan): PHP Static Analysis Tool.
- [PHP_CodeSniffer](https://github.com/squizlabs/PHP_CodeSniffer): Detects violations of a defined set of coding standards.

While Mago is intended to be a comprehensive toolchain that may eventually replace some of these tools,
we deeply appreciate their contributions and the foundation they have built for the PHP community.

## License

Mago is licensed under either of

- MIT License (MIT) - see [LICENSE-MIT](./LICENSE-MIT) file for details
- Apache License, Version 2.0 (Apache-2.0) - see [LICENSE-APACHE](./LICENSE-APACHE) file for details

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in Mago by you shall be dual licensed as above, without any additional terms or conditions.

---

Thank you for your interest in Mago. We look forward to sharing our progress and collaborating with the community as the project evolves.
