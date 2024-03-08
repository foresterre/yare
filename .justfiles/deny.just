deny: workspace test-crates

workspace: install-cargo-deny
    cargo deny --log-level error --workspace --manifest-path  {{justfile_directory()}}/Cargo.toml check

test-crates: install-cargo-deny
    cargo deny --log-level error --manifest-path  {{justfile_directory()}}/yare-tests-integration/Cargo.toml check
    cargo deny --log-level error --manifest-path  {{justfile_directory()}}/yare-tests-ui/Cargo.toml check

install-cargo-deny:
    cargo install cargo-deny
