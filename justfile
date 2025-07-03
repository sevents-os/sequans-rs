set quiet

_default: _help

_help:
    just --list

# Run tests
test:
    cargo test --lib --features "log,gm02sp" -- --nocapture

# Lint the code
lint:
    cargo clippy --lib --features "log,gm02sp" --tests -- -D warnings

# Format code
fmt:
    cargo fmt --all
