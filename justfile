test:
    cargo test --all
    cargo test --manifest-path yare-tests-integration/Cargo.toml
    cargo test --manifest-path yare-tests-ui/Cargo.toml

fmt:
    cargo fmt --all
    cargo fmt --manifest-path yare-tests-integration/Cargo.toml
    cargo fmt --manifest-path yare-tests-ui/Cargo.toml

clippy:
    cargo clippy --all-targets --all-features -- -D warnings

msrv:
    cargo install cargo-msrv --version 0.16.0-beta.20
    cargo msrv verify
    cargo msrv verify --manifest-path yare-macro/Cargo.toml

before-push: fmt test clippy msrv
