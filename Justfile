default:
  @just --list

# build the library
build:
    cargo build --release

# detect linting problems.
lint:
    cargo fmt --all -- --check --unstable-features
    cargo clippy --workspace --all-targets --all-features -- -D warnings
    cargo check --workspace --locked

# fix linting problems.
fix:
    cargo fmt --all -- --unstable-features
    cargo clippy --workspace --all-targets --all-features --fix --allow-dirty --allow-staged
    cargo fix --allow-dirty --allow-staged

publish:
    cd crates/casing && cargo publish
    cd crates/trinary && cargo publish
    cd crates/interner && cargo publish
    cd crates/source && cargo publish
    cd crates/span && cargo publish
    cd crates/reflection && cargo publish
    cd crates/token && cargo publish
    cd crates/ast && cargo publish
    cd crates/walker && cargo publish
    cd crates/traverser && cargo publish
    cd crates/ast-utils && cargo publish
    cd crates/composer && cargo publish
    cd crates/docblock && cargo publish
    cd crates/feedback && cargo publish
    cd crates/fixer && cargo publish
    cd crates/reporting && cargo publish
    cd crates/formatter && cargo publish
    cd crates/lexer && cargo publish
    cd crates/parser && cargo publish
    cd crates/names && cargo publish
    cd crates/symbol-table && cargo publish
    cd crates/semantics && cargo publish
    cd crates/reflector && cargo publish
    cd crates/linter && cargo publish
    cd crates/typing && cargo publish
    cd crates/service && cargo publish
    cd crates/wasm && cargo publish
    cd crates/cli && cargo publish
    cargo publish
