# Yare ⛵

Procedural macro based parameterized testing library. Run a test case with many different inputs.
Test cases can be defined using the 'parameterized' attribute instead of the 'test' attribute.

**Features**

- **Parameterization:** Specify different inputs to test multiple scenarios with a single test definition.
- **Flexible:** Parameterized test case arguments are expressions.
- **Out of the box experience:** Works with any Rust version out of the box. No custom test harness necessary.
- **Reusable:** Promotes code reuse by defining test cases once and using them with different parameters across multiple
  tests.
- **Readable:** Maintains readability by using a familiar Rustic attribute syntax.
- **Identifiable:** Each test case has a user defined name which can be referred back to, and can be used to run
  individual test cases.
- **Battle tested:** Used for years in tests of
  the [cargo-msrv](https://crates.io/crates/cargo-msrv), [bisector](https://crates.io/crates/bisector)
  and [rust-releases](https://crates.io/crates/rust-releases) crates (amongst others).

## Table of contents

* [Introduction](#yare-)
* [Examples](#examples-back-to-top)
* [Arguments are expressions](#arguments-are-expressions-back-to-top)
* [Custom test macro (e.g. tokio::test)](#custom-test-macro-eg-tokiotest-back-to-top)
* [Return types](#return-types-back-to-top)
* [Function qualifiers](#function-qualifiers-back-to-top)
* [Global #[parameterized(,,,)] import](#globally-importing-parameterized-back-to-top)
* [License](#license-back-to-top)

## Examples <sup>(<a href="#yare-">back to top</a>)</sup>

**A first example**

```rust
fn add5<T: Into<u32>>(component: T) -> u32 {
    component.into() + 5
}

#[cfg(test)]
mod tests {
    use super::*;
    use yare::parameterized;

    #[parameterized(
      zero_plus_five = { 0, 5 },
      one_plus_five = { 1, 6 },
      two_plus_five = { 2, 7 },
    )]
    fn test_add5(input: u16, expected: u32) {
        assert_eq!(add5(input), expected);
    }
}
```

**An example with values**

```rust
enum Fruit {
    Apple,
    Bramble(BrambleFruit),
    Pear,
}

trait NameOf {
    fn name_of(&self) -> &str;
}

impl NameOf for Fruit {
    fn name_of(&self) -> &str {
        match self {
            Fruit::Apple => "apple",
            Fruit::Bramble(fruit) => fruit.name_of(),
            Fruit::Pear => "pear",
        }
    }
}

enum BrambleFruit {
    Blackberry,
}

impl NameOf for BrambleFruit {
    fn name_of(&self) -> &str {
        match self {
            BrambleFruit::Blackberry => "blackberry",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use yare::parameterized;

    #[parameterized(
      apple = { Fruit::Apple, "apple" },
      pear = { Fruit::Pear, "pear" },
      blackberry = { Fruit::Bramble(BrambleFruit::Blackberry), "blackberry" },
    )]
    fn a_fruity_test(fruit: Fruit, name: &str) {
        assert_eq!(fruit.name_of(), name)
    }
}
```

<br>

## Arguments are expressions <sup>(<a href="#yare-">back to top</a>)</sup>

While the arguments above were simple values, any expression can be used as argument in a test case.

**Example**

In the example below, we roll the dice 3 times to generate a seed for later roll_dice function calls.
The first argument `seed1` is a _function call_ to roll_dice. This randomness function is seeded with value `0`.
The second argument `seed2` is a _block expression_. In the expression the roll_dice function is called twice.
The test itself takes the maximum of `seed1` and `seed2`, rolls the die 1000 times, and checks that all values
are valid for a d6 die.

```rust
use std::sync::atomic::{AtomicU32, Ordering};
use yare::parameterized;

#[parameterized(
  seeding_randomness_with_two_dice_rolls = 
  {
    roll_dice(&AtomicU32::new(0)),                              // <- This is an expression (a function call)
    {                                                           // <- This is also an expression (a block expression)
      let from_half = roll_dice( &AtomicU32::new(u32::MAX / 2));
      let from_max = roll_dice( &AtomicU32::new(u32::MAX));
      
      u8::min(from_half, from_max)
    }
  }
)]
fn dicey(seed1: u8, seed2: u8) {
    // Creating a base seed in a complicated way for the sake of it.
    let max = u8::max(seed1, seed2);
    let seed = AtomicU32::new(u32::from(max));

    let out_of_bounds_values = (0..1000)         // roll the dice 1000 times
        .map(|_| roll_dice(&seed))
        .find(|value| !(1..=6).contains(value)); // check that the outputs of the dice are just 1, 2, 3, 4, 5 or 6. 

    assert!(out_of_bounds_values.is_none());
}
```

## Custom test macro (e.g. tokio::test) <sup>(<a href="#yare-">back to top</a>)</sup>

By default, the code generation step of the `parameterized` attribute will generate test cases marked with a `#[test]`
attribute. For example, the `add5` test from the [examples](#examples-back-to-top) would generate something like:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use yare::parameterized;

    // Approximate generated code from add5 example:
    #[cfg(test)]
    mod add5 {
        use super::*;

        #[test]
        fn zero_plus_five() {
            let input: u16 = 0;
            let expected: u32 = 5;
            assert_eq!(add5(input), expected);
        }

        #[test]
        fn one_plus_five() {
            let input: u16 = 1;
            let expected: u32 = 6;
            assert_eq!(add5(input), expected);
        }

        #[test]
        fn two_plus_five() {
            let input: u16 = 2;
            let expected: u32 = 7;
            assert_eq!(add5(input), expected);
        }
    }
}
```

However, sometimes a different test macro is desired. An example is when writing tests for projects which depend on tokio.
For this, you may want to use `#[tokio::test]` (it also requires the test function to also have the `async` qualifier).

In Yare, it is possible to specify a custom test macro. To do so, you may add the `#[test_macro(...)]` attribute _after_
a `#[parameterized]` attribute.

**Custom test macro example: tokio::test**

```rust,ignore
use yare::parameterized;

#[parameterized(
    zero_wait = { 0, 0 },
    show_paused = { 500, 0 },
)]
#[test_macro(tokio::test(start_paused = true))]
async fn test(wait: u64, time_elapsed: u128) {
    let start = std::time::Instant::now();
    tokio::time::sleep(tokio::time::Duration::from_millis(wait)).await;

    assert_eq!(start.elapsed().as_millis(), time_elapsed);
}
```

Gotchas:

* The `#[test_macro(...)]` must always be specified after a `#[parameterized(...)]` attribute.
* Only one `#[test_macro(...)]` attribute per parameterized test function is allowed.
* While you can rename the parameterized attribute using import aliassing (
  e.g. `use yare::parameterized as pm`), the `test_macro` attribute cannot be renamed,
  since it's not actually defined as a separate macro.
  Instead, the `parameterized` macro parses this attribute as well.

## Return types <sup>(<a href="#yare-">back to top</a>)</sup>

Yare supports specifying a return type for a parameterized test function.

Note that the underlying test attribute must also have support for return types.
By default, Yare generates individual test cases decorated with the
familiar [test](https://doc.rust-lang.org/reference/attributes/testing.html#the-test-attribute)
attribute, which is included with any Rust distribution by default.

**Example**

```rust
use yare::parameterized;

#[parameterized(
  ok = { Ok(0) },
  // err = {  Err("noes!".to_string()) }, <-- enabling this would result in a failed test, since the error code will not be an `ErrorCode::Success`. See the `Termination` trait for more.
)]
fn test(value: Result<u32, String>) -> Result<(), String> {
    let v = value?;

    assert_eq!(v.unwrap(), 0);
}

```

## Function qualifiers <sup>(<a href="#yare-">back to top</a>)</sup>

Yare supports the following function qualifiers: `const`, `async`, `unsafe` and `extern`.
This is particularly useful if you use `#[parameterized(...)]` with a custom test macro such as `tokio::test`, instead
of the built-in test macro.

**Example**

```rust
use yare::parameterized;

#[parameterized(
  purple = { & [128, 0, 128] },
  orange = { & [255, 127, 0] },
)]
const extern "C" fn has_reds(streamed_color: &[u8]) {
    assert!(streamed_color.first().is_some());
}
```

## Globally importing parameterized <sup>(<a href="#yare-">back to top</a>)</sup>

If you prefer not to import this library (with `use yare::parameterized;`) in every test module, you can put
the following snippet at the top of your crate root:

```rust
#[cfg(test)]
#[macro_use]
extern crate yare;
```

## License <sup>(<a href="#yare-">back to top</a>)</sup>

Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.

<br>

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
