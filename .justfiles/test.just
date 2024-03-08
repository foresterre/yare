test: yare tests-integration tests-ui

yare:
    cargo test --all

tests-integration:
    cargo test --manifest-path  {{justfile_directory()}}/yare-tests-integration/Cargo.toml

tests-ui:
    cargo test --manifest-path  {{justfile_directory()}}/yare-tests-ui/Cargo.toml
