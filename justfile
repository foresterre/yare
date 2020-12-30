test:
    cargo test --all

before-push:
    cargo fmt --all
    cargo clippy --all-targets --all-features -- -D warnings
    cargo test --all