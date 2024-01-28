use crate::iter::{iterate_lexical, iterate_lexical_only_alnum};
use core::{
    cmp::Ordering,
    iter::Peekable,
};

fn cmp_ascii_digits(lhs: &mut Peekable<impl Iterator<Item=char>>, rhs: &mut Peekable<impl Iterator<Item=char>>) -> Option<Ordering> {
    #[derive(PartialEq)]
    enum Origin {
        Lhs,
        Rhs,
    }

    // The loop below iterates through both iterators at once and handles ascii digits for comparison.
    // If one iterator runs out of ascii digits, it is stored in this struct together with the
    // information where it originated from.
    struct NonDigit {
        c: char,
        origin: Origin,
    }

    impl core::ops::Deref for NonDigit {
        type Target = char;

        fn deref(&self) -> &Self::Target {
            &self.c
        }
    }

    impl NonDigit {
        #[allow(dead_code)]

        fn is_lhs(&self) -> bool {
            self.origin == Origin::Lhs
        }

        fn is_rhs(&self) -> bool {
            self.origin == Origin::Rhs
        }
    }

    fn ok_if_ascii_digit(c: char) -> Result<char, char> {
        Some(c).filter(char::is_ascii_digit).ok_or(c)
    }

    let mut current_cmp = None;
    loop {
        match (lhs.peek(), rhs.peek()) {
            (Some(&a), Some(&b)) => {
                let non_digit = match (ok_if_ascii_digit(a), ok_if_ascii_digit(b)) {
                    (Ok(a), Ok(b)) => {
                        // Only update current_cmp if the current comparison is yet undecided.
                        // current_cmp is returned later when at least one iterator has hit a non-digit.
                        if current_cmp.is_none() || current_cmp == Some(Ordering::Equal) {
                            current_cmp = Some(a.cmp(&b));
                        }
                        None
                    },
                    (Err(c), Ok(_)) => Some(NonDigit{ c, origin: Origin::Lhs }),
                    (Ok(_), Err(c)) => Some(NonDigit{ c, origin: Origin::Rhs }),
                    (Err(_), Err(_)) => break current_cmp,
                };

                // Advance underlying iterators, since we only peek and break early if no iterator
                // has any digits left, keeping these characters in the iterators for the caller to
                // deal with in case current_cmp.is_none() or current_cmp == Some(Ordering::Equal).
                let _ = lhs.next();
                let _ = rhs.next();

                // Return the appropriate ordering of a number versus non-digit characters.
                if let Some(c) = non_digit {
                    let mut ord = if current_cmp.is_none() && c.is_alphanumeric() {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    };
                    if c.is_rhs() {
                        ord = ord.reverse();
                    }
                    break Some(ord);
                }
            }
            (Some(_), None) => {
                let _ = lhs.next();
                break Some(Ordering::Greater);
            }
            (None, Some(_)) => {
                let _ = rhs.next();
                break Some(Ordering::Less);
            }
            (None, None) => {
                break current_cmp;
            }
        }
    }
}

#[inline]
fn ret_ordering(lhs: char, rhs: char) -> Ordering {
    let is_lhs_alnum = lhs.is_alphanumeric();
    let is_rhs_alnum = rhs.is_alphanumeric();

    let result = if is_lhs_alnum == is_rhs_alnum {
        lhs.cmp(&rhs)
    } else if is_lhs_alnum {
        Ordering::Greater
    } else {
        Ordering::Less
    };
    result
}

/// Compares strings lexicographically
///
/// For example, `"a" < "ä" < "aa"`
pub fn lexical_cmp(lhs: &str, rhs: &str) -> Ordering {
    let mut iter1 = iterate_lexical(lhs);
    let mut iter2 = iterate_lexical(rhs);

    loop {
        match (iter1.next(), iter2.next()) {
            (Some(lhs), Some(rhs)) => {
                if lhs != rhs {
                    return ret_ordering(lhs, rhs);
                }
            }
            (Some(_), None) => return Ordering::Greater,
            (None, Some(_)) => return Ordering::Less,
            (None, None) => return lhs.cmp(&rhs),
        }
    }
}

/// Compares strings lexicographically, skipping non-alphanumeric characters
///
/// For example, `"a" < " ä" < "ä" < "aa"`
pub fn lexical_only_alnum_cmp(s1: &str, s2: &str) -> Ordering {
    let mut iter1 = iterate_lexical_only_alnum(s1);
    let mut iter2 = iterate_lexical_only_alnum(s2);

    loop {
        match (iter1.next(), iter2.next()) {
            (Some(lhs), Some(rhs)) => {
                if lhs != rhs {
                    return lhs.cmp(&rhs);
                }
            }
            (Some(_), None) => return Ordering::Greater,
            (None, Some(_)) => return Ordering::Less,
            (None, None) => return s1.cmp(&s2),
        }
    }
}

/// Compares strings naturally and lexicographically
///
/// For example, `"a" < "ä" < "aa"`, `"50" < "100"`
pub fn natural_lexical_cmp(s1: &str, s2: &str) -> Ordering {
    let mut iter1 = iterate_lexical(s1).peekable();
    let mut iter2 = iterate_lexical(s2).peekable();

    loop {
        match cmp_ascii_digits(&mut iter1, &mut iter2) {
            None | Some(Ordering::Equal) => (),
            Some(result) => return result,
        }
        match (iter1.next(), iter2.next()) {
            (Some(lhs), Some(rhs)) => {
                if lhs != rhs {
                    return ret_ordering(lhs, rhs);
                }
            }
            (Some(_), None) => return Ordering::Greater,
            (None, Some(_)) => return Ordering::Less,
            (None, None) => return s1.cmp(&s2),
        }
    }
}

/// Compares strings naturally and lexicographically, skipping non-alphanumeric characters
///
/// For example, `"a" < " ä" < "ä" < "aa"`, `"50" < "100"`
pub fn natural_lexical_only_alnum_cmp(s1: &str, s2: &str) -> Ordering {
    let mut iter1 = iterate_lexical_only_alnum(s1).peekable();
    let mut iter2 = iterate_lexical_only_alnum(s2).peekable();

    loop {
        match cmp_ascii_digits(&mut iter1, &mut iter2) {
            None | Some(Ordering::Equal) => (),
            Some(result) => return result,
        }
        match (iter1.next(), iter2.next()) {
            (Some(lhs), Some(rhs)) => {
                if lhs != rhs {
                    return lhs.cmp(&rhs);
                }
            }
            (Some(_), None) => return Ordering::Greater,
            (None, Some(_)) => return Ordering::Less,
            (None, None) => return s1.cmp(&s2),
        }
    }
}

/// Compares strings naturally
///
/// For example, `"50" < "100"`
pub fn natural_cmp(s1: &str, s2: &str) -> Ordering {
    let mut iter1 = s1.chars().peekable();
    let mut iter2 = s2.chars().peekable();

    loop {
        match cmp_ascii_digits(&mut iter1, &mut iter2) {
            None | Some(Ordering::Equal) => (),
            Some(result) => return result,
        }
        match (iter1.next(), iter2.next()) {
            (Some(lhs), Some(rhs)) => {
                if lhs != rhs {
                    return lhs.cmp(&rhs);
                }
            }
            (Some(_), None) => return Ordering::Greater,
            (None, Some(_)) => return Ordering::Less,
            (None, None) => return Ordering::Equal,
        }
    }
}

/// Compares strings naturally, skipping non-alphanumeric characters
///
/// For example, `"a" < " b" < "b"`, `"50" < "100"`
pub fn natural_only_alnum_cmp(s1: &str, s2: &str) -> Ordering {
    let mut iter1 = s1.chars().filter(|c| c.is_alphanumeric()).peekable();
    let mut iter2 = s2.chars().filter(|c| c.is_alphanumeric()).peekable();

    loop {
        match cmp_ascii_digits(&mut iter1, &mut iter2) {
            None | Some(Ordering::Equal) => (),
            Some(result) => return result,
        }
        match (iter1.next(), iter2.next()) {
            (Some(lhs), Some(rhs)) => {
                if lhs != rhs {
                    return lhs.cmp(&rhs);
                }
            }
            (Some(_), None) => return Ordering::Greater,
            (None, Some(_)) => return Ordering::Less,
            (None, None) => return s1.cmp(&s2),
        }
    }
}

/// Compares strings, skipping non-alphanumeric characters
///
/// For example, `"a" < " b" < "b"`
pub fn only_alnum_cmp(s1: &str, s2: &str) -> Ordering {
    let mut iter1 = s1.chars().filter(|c| c.is_alphanumeric());
    let mut iter2 = s2.chars().filter(|c| c.is_alphanumeric());

    loop {
        match (iter1.next(), iter2.next()) {
            (Some(lhs), Some(rhs)) => {
                if lhs != rhs {
                    return lhs.cmp(&rhs);
                }
            }
            (Some(_), None) => return Ordering::Greater,
            (None, Some(_)) => return Ordering::Less,
            (None, None) => return s1.cmp(&s2),
        }
    }
}

/// Compares strings (not lexicographically or naturally, doesn't skip non-alphanumeric characters)
///
/// For example, `"B" < "a" < "b" < "ä"`
pub fn cmp(s1: &str, s2: &str) -> Ordering {
    let mut iter1 = s1.chars();
    let mut iter2 = s2.chars();

    loop {
        match (iter1.next(), iter2.next()) {
            (Some(lhs), Some(rhs)) => {
                if lhs != rhs {
                    return lhs.cmp(&rhs);
                }
            }
            (Some(_), None) => return Ordering::Greater,
            (None, Some(_)) => return Ordering::Less,
            (None, None) => return Ordering::Equal,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test(desc: &'static str, algo: impl Fn(&str, &str) -> Ordering) -> impl Fn(&str, &str) {
        move |lhs, rhs| {
            let success = algo(lhs, rhs) == Ordering::Less;
            assert!(success, "{} comparison {:?} < {:?} failed", desc, lhs, rhs);

            let success = algo(rhs, lhs) == Ordering::Greater;
            assert!(success, "{} comparison {:?} > {:?} failed", desc, rhs, lhs);
        }
    }

    #[test]
    fn test_cmp() {
        let ordered = make_test("Cmp", cmp);

        ordered("aaa", "aaaa");
        ordered("aaa", "aab");
        ordered("AAb", "aaa");
        ordered("aab", "äáa");
        ordered("aaa", "äáb");

        ordered("T-20", "T-5");
        ordered("T-5", "Ŧ-5");
    }

    #[test]
    fn test_only_alnum() {
        let ordered = make_test("Only-alnum", only_alnum_cmp);

        ordered("aaa", "aaaa");
        ordered("aaa", "aab");
        ordered("AAb", "aaa");
        ordered("aab", "äáa");
        ordered("aaa", "äáb");

        ordered("_ad", "_æ");
        ordered("_ae", "_æ");
        ordered("_ae_", "_æ");
        ordered("_af", "_æ");

        ordered("T-20", "T-5");
        ordered("T-5", "Ŧ-5");
    }

    #[test]
    fn test_lexical() {
        let ordered = make_test("Lexical", lexical_cmp);

        ordered("aaa", "aaaa");
        ordered("aaa", "aab");
        ordered("aaa", "AAb");
        ordered("äáa", "aab");
        ordered("aaa", "äáb");

        ordered("_ad", "_æ");
        ordered("_ae", "_æ");
        ordered("_æ", "_ae_");
        ordered("_æ", "_af");

        ordered("T-20", "T-5");
        ordered("T-5", "Ŧ-5");
    }

    #[test]
    fn test_lexical_only_alnum() {
        let ordered = make_test("Lexical, only-alnum", lexical_only_alnum_cmp);

        ordered("aaa", "aaaa");
        ordered("aaa", "aab");
        ordered("aaa", "AAb");
        ordered("äáa", "aab");
        ordered("aaa", "äáb");

        ordered("_ad", "_æ");
        ordered("_ae", "_æ");
        ordered("_ae_", "_æ");
        ordered("_æ", "_af");

        ordered("T20", "T-21");
        ordered("T-21", "T22");
        ordered("T-21", "T3");
    }

    #[test]
    fn test_natural() {
        let ordered = make_test("Natural", natural_cmp);

        ordered("1", "10");
        ordered("10", "15");
        ordered("150", "220");
        ordered("334", "335");
        ordered("433", "533");

        ordered("T-1", "T-5");
        ordered("T-27", "T5");
        ordered("T-27a", "T27b");

        ordered("T-27", "Ŧ-5");
        ordered("T-5", "Ŧ-27");
        ordered("T-5", "Ŧ-5");

        ordered("00000000000000000000", "18446744073709551616");
    }

    #[test]
    fn test_natural_only_alnum() {
        let ordered = make_test("Natural, only-alnum", natural_only_alnum_cmp);

        ordered("aaa", "aaaa");
        ordered("aaa", "aab");
        ordered("AAb", "aaa");
        ordered("aab", "äáa");
        ordered("aaa", "äáb");

        ordered("_ad", "_æ");
        ordered("_ae", "_æ");
        ordered("_ae_", "_æ");
        ordered("_af", "_æ");

        ordered("T20", "T-21");
        ordered("T-21", "T22");
        ordered("T3", "T-21");
    }

    #[test]
    fn test_natural_lexical() {
        let ordered = make_test("Natural, lexical", natural_lexical_cmp);

        ordered("1", "10");
        ordered("10", "15");
        ordered("150", "220");
        ordered("334", "335");
        ordered("433", "533");

        ordered("T-1", "T-5");
        ordered("T-5", "T-27");
        ordered("T-27a", "T-27b");

        ordered("Ŧ-5", "T-27");
        ordered("T-5", "Ŧ-27");
        ordered("T-5", "Ŧ-5");
    }

    #[test]
    fn test_natural_lexical_only_alnum() {
        let ordered = make_test(
            "Natural, lexical, only-alnum",
            natural_lexical_only_alnum_cmp,
        );

        ordered("1", "10");
        ordered("10", "15");
        ordered("150", "220");
        ordered("334", "335");
        ordered("433", "533");

        ordered("T-1", "T-5");
        ordered("T5", "T-27");
        ordered("T-27a", "T27b");

        ordered("Ŧ-5", "T-27");
        ordered("T-5", "Ŧ-27");
        ordered("T-5", "Ŧ-5");
    }
}
