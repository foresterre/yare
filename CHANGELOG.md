# Changelog

## [Unreleased]

[Unreleased]: https://github.com/foresterre/storyteller/compare/v3.0.0...HEAD

## [3.0.0] - 2024-03-08

### Added

* Parameterized tests now parse and regenerate the `const`, `async`, `unsafe` and `extern` function qualifiers for test functions.

**Example**

```rust
use yare::parameterized;

// NB: The underlying test macro also must support these qualifiers. For example, the default `#[test]` doesn't support async and unsafe.

#[parameterized(
    purple = { &[128, 0, 128] },
    orange = { &[255, 127, 0] },
)]
const extern "C" fn has_reds(streamed_color: &[u8]) {
    assert!(streamed_color.first().is_some());
}
```

* It is now possible to specify a custom test macro with `#[test_macro(...)]`, which replaces the default `#[test]` attribute for a parameterized test.

**Example**

```rust
use yare::parameterized;

#[parameterized(
    zero_wait = { 0, 0 },
    show_paused = { 500, 0 },
)]
#[test_macro(tokio::test(start_paused = true))]
async fn test(wait: u64, time_elapsed: u128) {
    let start = std::time::Instant::now();
    tokio::time::sleep(tokio::time::Duration::from_millis(wait)).await;

    assert_eq!(time_elapsed, start.elapsed().as_millis());
}

```

[3.0.0]: https://github.com/foresterre/storyteller/compare/v2.0.0...v3.0.0

## [2.0.0] - 2023-10-16

### Added

* Test signature may now have return type, given that is also accepted by the `#[test]` macro.

**Example**

```rust
use yare::parameterized;

#[parameterized(
    ok = { Ok(0) },
    err = { Err("This case will fail".to_string()) },
)]
fn example_test(value: Result<u32, String>) -> Result<(), String> {
    let _ = value?;

    Ok(())
}
```

### Changed

* **Breaking:** Parameter and argument count now must match exactly 
* MSRV is now 1.56

### Removed

* **Breaking:** Removed deprecated `ide!` macro

[2.0.0]: https://github.com/foresterre/yare/releases/tag/v2.0.0

## [1.0.2] - 2022-08-19

### Added

* Added crate root documentation for rustdoc and docs.rs

### Deprecated

* Deprecated `ide!` macro

[1.0.2]: https://github.com/foresterre/yare/releases/tag/v1.0.2

<!-- Example:

## [0.1.0] - 2022-01-01

### Added

### Changed

### Removed

### Fixed

### Deprecated

### Security


[0.1.0]: https://github.com/foresterre/bisector/compare/v0.0.0...v0.1.0

-->
