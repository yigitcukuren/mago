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
    # Sadly, we get rate-limited by crates.io, so we have to sleep between each publish.
    # This is a workaround until we hopefully get a limit increase from crates.io.
    # Note: the order of publishing is important, as some crates depend on others.
    sleep 120 && cargo publish -p mago-casing --allow-dirty
    sleep 120 && cargo publish -p mago-trinary --allow-dirty
    sleep 120 && cargo publish -p mago-interner --allow-dirty
    sleep 120 && cargo publish -p mago-source --allow-dirty
    sleep 120 && cargo publish -p mago-span --allow-dirty
    sleep 120 && cargo publish -p mago-reflection --allow-dirty
    sleep 120 && cargo publish -p mago-token --allow-dirty
    sleep 120 && cargo publish -p mago-ast --allow-dirty
    sleep 120 && cargo publish -p mago-walker --allow-dirty
    sleep 120 && cargo publish -p mago-traverser --allow-dirty
    sleep 120 && cargo publish -p mago-ast-utils --allow-dirty
    sleep 120 && cargo publish -p mago-composer --allow-dirty
    sleep 120 && cargo publish -p mago-docblock --allow-dirty
    sleep 120 && cargo publish -p mago-feedback --allow-dirty
    sleep 120 && cargo publish -p mago-fixer --allow-dirty
    sleep 120 && cargo publish -p mago-reporting --allow-dirty
    sleep 120 && cargo publish -p mago-formatter --allow-dirty
    sleep 120 && cargo publish -p mago-lexer --allow-dirty
    sleep 120 && cargo publish -p mago-parser --allow-dirty
    sleep 120 && cargo publish -p mago-names --allow-dirty
    sleep 120 && cargo publish -p mago-symbol-table --allow-dirty
    sleep 120 && cargo publish -p mago-semantics --allow-dirty
    sleep 120 && cargo publish -p mago-typing --allow-dirty
    sleep 120 && cargo publish -p mago-reflector --allow-dirty
    sleep 120 && cargo publish -p mago-linter --allow-dirty
    sleep 120 && cargo publish -p mago-service --allow-dirty
    sleep 120 && cargo publish -p mago-wasm --allow-dirty
    sleep 120 && cargo publish -p mago-cli --allow-dirty
    sleep 120 && cargo publish
