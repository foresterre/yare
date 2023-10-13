//! # Yare
//!
//! Procedural macro based parameterized testing library. Run a test case with many different inputs.
//! Test cases can be defined using the 'parameterized' attribute instead of the 'test' attribute.
//!
//! ### Examples:
//!
//! <br>
//!
//! **Example: Add5**
//!
//! ```rust, no_run
//! fn add5<T: Into<u32>>(component: T) -> u32 {
//!     component.into() + 5
//! }
//!
//! #[cfg(test)]
//! mod tests {
//!     # use std::assert_eq;
//! use super::*;
//!     use yare::parameterized;
//!
//!     #[parameterized(
//!         zero_plus_five = { 0, 5 },
//!         one_plus_five = { 1, 6 },
//!         two_plus_five = { 2, 7 },
//!     )]
//!     fn test_add5(input: u16, expected: u32) {
//!         assert_eq!(add5(input), expected);
//!     }
//! }
//! ```
//!
//! **Example: Fruits**
//!
//! ```rust, no_run
//! enum Fruit {
//!     Apple,
//!     Bramble(BrambleFruit),
//!     Pear,
//! }
//!
//! trait NameOf {
//!     fn name_of(&self) -> &str;
//! }
//!
//! impl NameOf for Fruit {
//!     fn name_of(&self) -> &str {
//!         match self {
//!             Fruit::Apple => "apple",
//!             Fruit::Bramble(fruit) => fruit.name_of(),
//!             Fruit::Pear => "pear",
//!         }
//!     }
//! }
//!
//! enum BrambleFruit {
//!     Blackberry,
//! }
//!
//! impl NameOf for BrambleFruit {
//!     fn name_of(&self) -> &str {
//!         match self {
//!             BrambleFruit::Blackberry => "blackberry",
//!         }
//!     }
//! }
//!
//! #[cfg(test)]
//! mod tests {
//!     # use std::assert_eq;
//!     use super::*;
//!     use yare::parameterized;
//!
//!     #[parameterized(
//!         apple = { Fruit::Apple, "apple" },
//!         pear = { Fruit::Pear, "pear" },
//!         blackberry = { Fruit::Bramble(BrambleFruit::Blackberry), "blackberry" },
//!     )]
//!     fn a_fruity_test(fruit: Fruit, name: &str) {
//!         assert_eq!(fruit.name_of(), name)
//!     }
//! }
//! ```
//!
//! <br>
//!
//! ### Imports
//!
//! If you prefer not to import this library (with `use yare::parameterized;`) in every test module, you can put
//! the following snippet at the top of your crate root:
//! ```rust, no_run
//! #[cfg(test)]
//! #[macro_use]
//! extern crate yare;
//! ```
//!
//! ### License
//!
//! Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
//! 2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
//!
//! <br>
//!
//! Unless you explicitly state otherwise, any contribution intentionally submitted
//! for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
//! be dual licensed as above, without any additional terms or conditions.
//!

extern crate yare_macro as yare;

pub use yare::parameterized;

/// Attribute macro's such as 'parameterized' do not enable the run tests intent for a module
/// marked as cfg(test) (or a #[test] function for that matter) in Intellij.
///
/// To enable the intent within a module, we need at least a single test marked with `#[test]`.
/// The `ide!()` macro is a work around for this issue and creates this empty test. It can be called
/// within every module where we wish to run test cases using the run configuration / run test context
/// menu.
///
/// Using the intellij-rust new macro expansion engine, if this macro is called within a module,
/// the module will be marked as test, and the 'run as test' context menu will be provided in the
/// gutter.
#[doc(hidden)]
#[deprecated]
#[macro_export]
macro_rules! ide {
    () => {
        #[test]
        fn __mark_with_test_intent() {}
    };
}

#[cfg(test)]
mod tests;
