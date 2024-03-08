fmt: yare tests-integration tests-ui

yare:
    cargo fmt --all --manifest-path {{justfile_directory()}}/Cargo.toml

tests-integration:
    cargo fmt --manifest-path {{justfile_directory()}}/yare-tests-integration/Cargo.toml

tests-ui:
    cargo fmt --manifest-path {{justfile_directory()}}/yare-tests-ui/Cargo.toml

check:
    cargo fmt --all --manifest-path {{justfile_directory()}}/Cargo.toml -- --check
    cargo fmt --manifest-path {{justfile_directory()}}/yare-tests-integration/Cargo.toml -- --check
    cargo fmt --manifest-path {{justfile_directory()}}/yare-tests-ui/Cargo.toml -- --check

