msrv: yare yare-macro

yare: install-cargo-msrv
    cargo msrv verify --manifest-path  {{justfile_directory()}}/Cargo.toml

yare-macro: install-cargo-msrv
    cargo msrv verify --manifest-path  {{justfile_directory()}}/yare-macro/Cargo.toml

install-cargo-msrv:
    cargo install cargo-msrv --version 0.16.0-beta.20
