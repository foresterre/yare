//! For additional docs see [here](https://docs.rs/crate/yare) :).

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
#[macro_export]
macro_rules! ide {
    () => {
        #[test]
        fn __mark_with_test_intent() {}
    };
}

#[cfg(test)]
mod fruits;

#[cfg(test)]
mod tests {
    use crate::parameterized as pm;

    fn add5<T: Into<u32>>(component: T) -> u32 {
        component.into() + 5
    }

    mod readme_test {
        use super::*;

        ide!();

        #[pm(zero = {
            0, 5
        }, one = {
            1, 6
        }, two = {
            2, 7
        })]
        fn test_add5(eh: u16, expected: u32) {
            assert_eq!(add5(eh), expected)
        }
    }

    mod marked_as_test_module {
        use super::*;

        ide!();

        #[pm(two = { 2, 4 }, six = { 6, 12 }, eight = { 8, 16 })]
        fn test_times2(input: i32, output: i32) {
            let times2 = |receiver: i32| receiver * 2;

            assert_eq!(times2(input), output);
        }
    }

    mod transitive_attrs {
        use super::*;

        ide!();

        #[pm(none = { None })]
        #[should_panic]
        fn numbers(input: Option<()>) {
            input.unwrap()
        }
    }
}
