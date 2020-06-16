//! This is a library to sort strings (or file paths) **lexically**. This means that non-ASCII
//! characters such as `á` or `ß` are treated like their closest ASCII character: `á` is treated
//! as `a`, `ß` is treated as `ss`.
//!
//! The sort is case-insensitive. Alphanumeric characters are sorted after all other characters
//! (punctuation, whitespace, special characters, emojis, ...).
//!
//! It is possible to enable **natural sorting**, which also handles ASCII numbers. For example,
//! `50` is sorted before `100` with natural sorting turned on.
//!
//! If different strings have the same ASCII representation (e.g. `"Foo"` and `"fóò"`), we fall
//! back to the default implementation, which just compares Unicode code points.
//!
//! ## Usage
//!
//! To sort strings or paths, use the `LexicalSort` trait:
//!
//! ```rust
//! use lexical_sort::LexicalSort;
//!
//! let mut strings = vec!["ß", "é", "100", "hello", "world", "50", ".", "B!"];
//! strings.lexical_sort(/* enable natural sorting: */ true);
//!
//! assert_eq!(&strings, &[".", "50", "100", "B!", "é", "hello", "ß", "world"]);
//! ```
//!
//! To just compare two strings, use the `lexical_cmp` or `lexical_natural_cmp` function.

mod cmp;
pub mod iter;

pub use cmp::{lexical_cmp, lexical_natural_cmp, natural_cmp};

use std::{
    borrow::Cow,
    ffi::{CStr, CString, OsStr, OsString},
    fs::DirEntry,
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
};

/// This trait adds functionality to slices containing strings or file paths
/// for sorting them lexically.
///
/// See the [module-level documentation](./index.html) for more information.
pub trait LexicalSort {
    /// Sorts the values lexically. If `natural` is set to `true`, numbers are sorted naturally.
    fn lexical_sort(&mut self, natural: bool);

    /// Sorts the values lexically. If `natural` is set to `true`, numbers are sorted naturally.
    /// This sort is unstable.
    fn unstable_lexical_sort(&mut self, natural: bool);
}

macro_rules! impl_for_str {
    ($t:ty) => {
        impl LexicalSort for $t {
            #[inline]
            fn lexical_sort(&mut self, natural: bool) {
                if natural {
                    self.sort_by(|lhs, rhs| lexical_natural_cmp(lhs, rhs));
                } else {
                    self.sort_by(|lhs, rhs| lexical_cmp(lhs, rhs));
                }
            }

            #[inline]
            fn unstable_lexical_sort(&mut self, natural: bool) {
                if natural {
                    self.sort_unstable_by(|lhs, rhs| lexical_natural_cmp(lhs, rhs));
                } else {
                    self.sort_unstable_by(|lhs, rhs| lexical_cmp(lhs, rhs));
                }
            }
        }
    };
}

impl_for_str!([&'_ str]);
impl_for_str!([String]);
impl_for_str!([Cow<'_, str>]);
impl_for_str!([Box<str>]);
impl_for_str!([Rc<str>]);
impl_for_str!([Arc<str>]);

macro_rules! impl_for_path_or_ffi {
    ($t:ty) => {
        impl LexicalSort for $t {
            #[inline]
            fn lexical_sort(&mut self, natural: bool) {
                if natural {
                    self.sort_by(|lhs, rhs| {
                        lexical_natural_cmp(&lhs.to_string_lossy(), &rhs.to_string_lossy())
                    });
                } else {
                    self.sort_by(|lhs, rhs| {
                        lexical_cmp(&lhs.to_string_lossy(), &rhs.to_string_lossy())
                    });
                }
            }

            #[inline]
            fn unstable_lexical_sort(&mut self, natural: bool) {
                if natural {
                    self.sort_unstable_by(|lhs, rhs| {
                        lexical_natural_cmp(&lhs.to_string_lossy(), &rhs.to_string_lossy())
                    });
                } else {
                    self.sort_unstable_by(|lhs, rhs| {
                        lexical_cmp(&lhs.to_string_lossy(), &rhs.to_string_lossy())
                    });
                }
            }
        }
    };
}

impl_for_path_or_ffi!([&'_ Path]);
impl_for_path_or_ffi!([PathBuf]);
impl_for_path_or_ffi!([Cow<'_, Path>]);
impl_for_path_or_ffi!([Box<Path>]);
impl_for_path_or_ffi!([Rc<Path>]);
impl_for_path_or_ffi!([Arc<Path>]);

impl LexicalSort for [DirEntry] {
    #[inline]
    fn lexical_sort(&mut self, natural: bool) {
        if natural {
            self.sort_by(|lhs, rhs| {
                lexical_natural_cmp(
                    &lhs.file_name().to_string_lossy(),
                    &rhs.file_name().to_string_lossy(),
                )
            });
        } else {
            self.sort_by(|lhs, rhs| {
                lexical_cmp(
                    &lhs.file_name().to_string_lossy(),
                    &rhs.file_name().to_string_lossy(),
                )
            });
        }
    }

    #[inline]
    fn unstable_lexical_sort(&mut self, natural: bool) {
        if natural {
            self.sort_unstable_by(|lhs, rhs| {
                lexical_natural_cmp(
                    &lhs.file_name().to_string_lossy(),
                    &rhs.file_name().to_string_lossy(),
                )
            });
        } else {
            self.sort_unstable_by(|lhs, rhs| {
                lexical_cmp(
                    &lhs.file_name().to_string_lossy(),
                    &rhs.file_name().to_string_lossy(),
                )
            });
        }
    }
}

impl_for_path_or_ffi!([&'_ OsStr]);
impl_for_path_or_ffi!([OsString]);
impl_for_path_or_ffi!([Cow<'_, OsStr>]);
impl_for_path_or_ffi!([Box<OsStr>]);
impl_for_path_or_ffi!([Rc<OsStr>]);
impl_for_path_or_ffi!([Arc<OsStr>]);

impl_for_path_or_ffi!([&'_ CStr]);
impl_for_path_or_ffi!([CString]);
impl_for_path_or_ffi!([Cow<'_, CStr>]);
impl_for_path_or_ffi!([Box<CStr>]);
impl_for_path_or_ffi!([Rc<CStr>]);
impl_for_path_or_ffi!([Arc<CStr>]);

#[test]
fn test_sort() {
    macro_rules! assert_lexically_sorted {
        ($array:expr, natural = $natural:expr) => {{
            let mut sorted = $array.clone();
            sorted.lexical_sort($natural);

            assert_eq!($array, sorted);
        }};
    }

    let strings = vec![
        "-", "-$", "-a", "100", "50", "a", "ä", "aa", "áa", "AB", "Ab", "ab", "AE", "ae", "æ", "af",
    ];
    let strings_nat = vec![
        "-", "-$", "-a", "50", "100", "a", "ä", "aa", "áa", "AB", "Ab", "ab", "AE", "ae", "æ", "af",
    ];

    assert_lexically_sorted!(strings, natural = false);
    assert_lexically_sorted!(strings_nat, natural = true);

    let paths: Vec<&Path> = strings.iter().map(|s| Path::new(s)).collect();
    let paths_nat: Vec<&Path> = strings_nat.iter().map(|s| Path::new(s)).collect();

    assert_lexically_sorted!(paths, natural = false);
    assert_lexically_sorted!(paths_nat, natural = true);
}
