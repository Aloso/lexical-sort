//! This is a library to sort strings (or file paths) **lexicographically**. This means that
//! non-ASCII characters such as `á` or `ß` are treated like their closest ASCII character: `á` is
//! treated as `a`, `ß` is treated as `ss`, etc.
//!
//! The sort is case-insensitive. Alphanumeric characters are sorted after all other characters
//! (punctuation, whitespace, special characters, emojis, ...).
//!
//! It is possible to enable **natural sorting**, which also handles ASCII numbers. For example,
//! `50` is less than `100` with natural sorting turned on. It's also possible to skip
//! characters that aren't alphanumeric, so e.g. `f-5` is next to `f5`.
//!
//! If different strings have the same ASCII representation (e.g. `"Foo"` and `"fóò"`), it
//! falls back to the default method from the standard library, so sorting is deterministic.
//!
//! <table><tr><td>
//! <b>NOTE</b>: This crate doesn't attempt to be correct for every locale, but it should work
//! reasonably well for a wide range of locales at a minimal performance cost.
//! </td></tr></table>
//!
//! ## Usage
//!
//! To sort strings or paths, you can use the `CmpStrings` or `CmpPaths` trait:
//!
//! ```rust
//! use lexical_sort::CmpStrings;
//!
//! let mut strings = vec!["ß", "é", "100", "hello", "world", "50", ".", "B!"];
//! strings.sort_unstable_by(|l, r| l.natural_lexical_cmp(r));
//!
//! assert_eq!(&strings, &[".", "50", "100", "B!", "é", "hello", "ß", "world"]);
//! ```
//!
//! Alternatively, you can use the `natural_cmp`, `lexical_cmp`, `lexical_natural_cmp`,
//! `lexical_cmp_only_alnum` and `lexical_natural_cmp_only_alnum` free functions.

mod cmp;
pub mod iter;

pub use cmp::{
    lexical_cmp, lexical_only_alnum_cmp, natural_cmp, natural_lexical_cmp,
    natural_lexical_only_alnum_cmp,
};

use std::{cmp::Ordering, path::Path};

/// A trait that implements various string comparison functions.
/// This trait is implemented for all types that implement `AsRef<str>`.
///
/// See the [module-level documentation](./index.html) for more information.
pub trait CmpStrings {
    fn natural_cmp(self, other: Self) -> Ordering;
    fn lexical_cmp(self, other: Self) -> Ordering;
    fn natural_lexical_cmp(self, other: Self) -> Ordering;
    fn lexical_only_alnum_cmp(self, other: Self) -> Ordering;
    fn natural_lexical_only_alnum_cmp(self, other: Self) -> Ordering;
}

impl<A: AsRef<str>> CmpStrings for A {
    #[inline]
    fn natural_cmp(self, other: Self) -> Ordering {
        natural_cmp(self.as_ref(), other.as_ref())
    }

    #[inline]
    fn lexical_cmp(self, other: Self) -> Ordering {
        lexical_cmp(self.as_ref(), other.as_ref())
    }

    #[inline]
    fn natural_lexical_cmp(self, other: Self) -> Ordering {
        natural_lexical_cmp(self.as_ref(), other.as_ref())
    }

    #[inline]
    fn lexical_only_alnum_cmp(self, other: Self) -> Ordering {
        lexical_only_alnum_cmp(self.as_ref(), other.as_ref())
    }

    #[inline]
    fn natural_lexical_only_alnum_cmp(self, other: Self) -> Ordering {
        natural_lexical_only_alnum_cmp(self.as_ref(), other.as_ref())
    }
}

/// A trait that implements various path comparison functions.
/// This trait is implemented for all types that implement `AsRef<Path>`.
///
/// See the [module-level documentation](./index.html) for more information.
pub trait CmpPaths {
    fn natural_cmp(self, other: Self) -> Ordering;
    fn lexical_cmp(self, other: Self) -> Ordering;
    fn natural_lexical_cmp(self, other: Self) -> Ordering;
    fn lexical_only_alnum_cmp(self, other: Self) -> Ordering;
    fn natural_lexical_only_alnum_cmp(self, other: Self) -> Ordering;
}

impl<A: AsRef<Path>> CmpPaths for A {
    #[inline]
    fn natural_cmp(self, other: Self) -> Ordering {
        natural_cmp(
            &self.as_ref().to_string_lossy(),
            &other.as_ref().to_string_lossy(),
        )
    }

    #[inline]
    fn lexical_cmp(self, other: Self) -> Ordering {
        lexical_cmp(
            &self.as_ref().to_string_lossy(),
            &other.as_ref().to_string_lossy(),
        )
    }

    #[inline]
    fn natural_lexical_cmp(self, other: Self) -> Ordering {
        natural_lexical_cmp(
            &self.as_ref().to_string_lossy(),
            &other.as_ref().to_string_lossy(),
        )
    }

    #[inline]
    fn lexical_only_alnum_cmp(self, other: Self) -> Ordering {
        lexical_only_alnum_cmp(
            &self.as_ref().to_string_lossy(),
            &other.as_ref().to_string_lossy(),
        )
    }

    #[inline]
    fn natural_lexical_only_alnum_cmp(self, other: Self) -> Ordering {
        natural_lexical_only_alnum_cmp(
            &self.as_ref().to_string_lossy(),
            &other.as_ref().to_string_lossy(),
        )
    }
}

#[test]
fn test_sort() {
    macro_rules! assert_lexically_sorted {
        ($T:ident, $array:expr, natural = $natural:expr) => {{
            let mut sorted = $array.clone();
            if $natural {
                sorted.sort_unstable_by(|l, r| $T::natural_lexical_cmp(l, r));
            } else {
                sorted.sort_unstable_by(|l, r| $T::lexical_cmp(l, r));
            }

            assert_eq!($array, sorted);
        }};
    }

    let strings = vec![
        "-", "-$", "-a", "100", "50", "a", "ä", "aa", "áa", "AB", "Ab", "ab", "AE", "ae", "æ", "af",
    ];
    let strings_nat = vec![
        "-", "-$", "-a", "50", "100", "a", "ä", "aa", "áa", "AB", "Ab", "ab", "AE", "ae", "æ", "af",
    ];

    assert_lexically_sorted!(CmpStrings, strings, natural = false);
    assert_lexically_sorted!(CmpStrings, strings_nat, natural = true);

    let paths: Vec<&Path> = strings.iter().map(|s| Path::new(s)).collect();
    let paths_nat: Vec<&Path> = strings_nat.iter().map(|s| Path::new(s)).collect();

    assert_lexically_sorted!(CmpPaths, paths, natural = false);
    assert_lexically_sorted!(CmpPaths, paths_nat, natural = true);
}
