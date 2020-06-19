use crate::iter::{iterate_lexical, iterate_lexical_only_alnum};
use core::cmp::Ordering;

macro_rules! cmp_ascii_digits {
    (first_digits($lhs:ident, $rhs:ident), iterators($iter1:ident, $iter2:ident)) => {
        let mut n1 = ascii_to_u64($lhs);
        let mut n2 = ascii_to_u64($rhs);
        loop {
            match (
                $iter1.peek().copied().filter(|c| c.is_ascii_digit()),
                $iter2.peek().copied().filter(|c| c.is_ascii_digit()),
            ) {
                (Some(lhs), Some(rhs)) => {
                    n1 = n1 * 10 + ascii_to_u64(lhs);
                    n2 = n2 * 10 + ascii_to_u64(rhs);
                    let _ = $iter1.next();
                    let _ = $iter2.next();
                }
                (Some(_), None) => return Ordering::Greater,
                (None, Some(_)) => return Ordering::Less,
                (None, None) => {
                    if n1 != n2 {
                        return n1.cmp(&n2);
                    } else {
                        break;
                    }
                }
            }
        }
    };
}

#[inline]
fn ascii_to_u64(c: char) -> u64 {
    (c as u64) - (b'0' as u64)
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

/// Compares two strings lexicographically
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

/// Compares two strings lexicographically, skipping characters that aren't alphanumeric
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

/// Compares two strings naturally and lexicographically
pub fn natural_lexical_cmp(s1: &str, s2: &str) -> Ordering {
    let mut iter1 = iterate_lexical(s1).peekable();
    let mut iter2 = iterate_lexical(s2).peekable();

    loop {
        match (iter1.next(), iter2.next()) {
            (Some(lhs), Some(rhs)) => {
                if lhs.is_ascii_digit() && rhs.is_ascii_digit() {
                    cmp_ascii_digits!(first_digits(lhs, rhs), iterators(iter1, iter2));
                } else if lhs != rhs {
                    return ret_ordering(lhs, rhs);
                }
            }
            (Some(_), None) => return Ordering::Greater,
            (None, Some(_)) => return Ordering::Less,
            (None, None) => return s1.cmp(&s2),
        }
    }
}

/// Compares two strings naturally and lexicographically, skipping characters that aren't
/// alphanumeric
pub fn natural_lexical_only_alnum_cmp(s1: &str, s2: &str) -> Ordering {
    let mut iter1 = iterate_lexical_only_alnum(s1).peekable();
    let mut iter2 = iterate_lexical_only_alnum(s2).peekable();

    loop {
        match (iter1.next(), iter2.next()) {
            (Some(lhs), Some(rhs)) => {
                if lhs.is_ascii_digit() && rhs.is_ascii_digit() {
                    cmp_ascii_digits!(first_digits(lhs, rhs), iterators(iter1, iter2));
                } else if lhs != rhs {
                    return lhs.cmp(&rhs);
                }
            }
            (Some(_), None) => return Ordering::Greater,
            (None, Some(_)) => return Ordering::Less,
            (None, None) => return s1.cmp(&s2),
        }
    }
}

/// Compares two strings naturally
pub fn natural_cmp(s1: &str, s2: &str) -> Ordering {
    let mut iter1 = s1.chars().peekable();
    let mut iter2 = s2.chars().peekable();

    loop {
        match (iter1.next(), iter2.next()) {
            (Some(lhs), Some(rhs)) => {
                if lhs.is_ascii_digit() && rhs.is_ascii_digit() {
                    cmp_ascii_digits!(first_digits(lhs, rhs), iterators(iter1, iter2));
                } else if lhs != rhs {
                    return lhs.cmp(&rhs);
                }
            }
            (Some(_), None) => return Ordering::Greater,
            (None, Some(_)) => return Ordering::Less,
            (None, None) => return Ordering::Equal,
        }
    }
}

#[test]
fn test_comparison() {
    fn ordered(lhs: &str, rhs: &str) {
        let success = lexical_cmp(lhs, rhs) == Ordering::Less;
        assert!(success, "Lexical comparison {:?} < {:?} failed", lhs, rhs);

        let success = lexical_cmp(rhs, lhs) == Ordering::Greater;
        assert!(success, "Lexical comparison {:?} > {:?} failed", rhs, lhs);
    }

    fn nat_ordered(lhs: &str, rhs: &str) {
        let success = natural_lexical_cmp(lhs, rhs) == Ordering::Less;
        assert!(success, "Natural comparison {:?} < {:?} failed", lhs, rhs);

        let success = natural_lexical_cmp(rhs, lhs) == Ordering::Greater;
        assert!(success, "Lexical comparison {:?} > {:?} failed", rhs, lhs);
    }

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

    nat_ordered("1", "10");
    nat_ordered("10", "15");
    nat_ordered("150", "220");
    nat_ordered("334", "335");
    nat_ordered("433", "533");

    nat_ordered("T-1", "T-5");
    nat_ordered("T-5", "T-27");
    nat_ordered("T-27a", "T-27b");

    nat_ordered("Ŧ-5", "T-27");
    nat_ordered("T-5", "Ŧ-27");
    nat_ordered("T-5", "Ŧ-5");
}

#[test]
fn test_comparison_only_alnum() {
    fn ordered(lhs: &str, rhs: &str) {
        let success = lexical_only_alnum_cmp(lhs, rhs) == Ordering::Less;
        assert!(success, "Lexical comparison {:?} < {:?} failed", lhs, rhs);

        let success = lexical_only_alnum_cmp(rhs, lhs) == Ordering::Greater;
        assert!(success, "Lexical comparison {:?} > {:?} failed", rhs, lhs);
    }

    fn nat_ordered(lhs: &str, rhs: &str) {
        let success = natural_lexical_only_alnum_cmp(lhs, rhs) == Ordering::Less;
        assert!(success, "Natural comparison {:?} < {:?} failed", lhs, rhs);

        let success = natural_lexical_only_alnum_cmp(rhs, lhs) == Ordering::Greater;
        assert!(success, "Lexical comparison {:?} > {:?} failed", rhs, lhs);
    }

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

    nat_ordered("1", "10");
    nat_ordered("10", "15");
    nat_ordered("150", "220");
    nat_ordered("334", "335");
    nat_ordered("433", "533");

    nat_ordered("T-1", "T-5");
    nat_ordered("T5", "T-27");
    nat_ordered("T-27a", "T27b");

    nat_ordered("Ŧ-5", "T-27");
    nat_ordered("T-5", "Ŧ-27");
    nat_ordered("T-5", "Ŧ-5");
}
