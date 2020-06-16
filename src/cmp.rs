use crate::iter::iterate_lexical;
use core::cmp::Ordering;

pub fn lexical_cmp(lhs: &str, rhs: &str) -> Ordering {
    let mut iter1 = iterate_lexical(lhs);
    let mut iter2 = iterate_lexical(rhs);

    loop {
        match (iter1.next(), iter2.next()) {
            (Some(lhs), Some(rhs)) => {
                if lhs != rhs {
                    let is_left_alnum = lhs.is_alphanumeric();
                    let is_right_alnum = rhs.is_alphanumeric();

                    return if is_left_alnum == is_right_alnum {
                        lhs.cmp(&rhs)
                    } else if is_left_alnum {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    };
                }
            }
            (Some(_), None) => return Ordering::Greater,
            (None, Some(_)) => return Ordering::Less,
            (None, None) => return lhs.cmp(&rhs),
        }
    }
}

pub fn lexical_natural_cmp(lhs: &str, rhs: &str) -> Ordering {
    let mut iter1 = iterate_lexical(lhs).peekable();
    let mut iter2 = iterate_lexical(rhs).peekable();

    loop {
        match (iter1.next(), iter2.next()) {
            (Some(lhs), Some(rhs)) => {
                if lhs.is_ascii_digit() && rhs.is_ascii_digit() {
                    loop {
                        match (
                            iter1.peek().copied().filter(|c| c.is_ascii_digit()),
                            iter2.peek().copied().filter(|c| c.is_ascii_digit()),
                        ) {
                            (Some(lhs), Some(rhs)) => {
                                if lhs != rhs {
                                    return lhs.cmp(&rhs);
                                } else {
                                    let _ = iter1.next();
                                    let _ = iter2.next();
                                }
                            }
                            (Some(_), None) => return Ordering::Greater,
                            (None, Some(_)) => return Ordering::Less,
                            (None, None) => break,
                        }
                    }
                } else if lhs != rhs {
                    let is_left_alnum = lhs.is_alphanumeric();
                    let is_right_alnum = rhs.is_alphanumeric();

                    return if is_left_alnum == is_right_alnum {
                        lhs.cmp(&rhs)
                    } else if is_left_alnum {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    };
                }
            }
            (Some(_), None) => return Ordering::Greater,
            (None, Some(_)) => return Ordering::Less,
            (None, None) => return lhs.cmp(&rhs),
        }
    }
}

pub fn natural_cmp(lhs: &str, rhs: &str) -> Ordering {
    let mut iter1 = lhs.chars().peekable();
    let mut iter2 = rhs.chars().peekable();

    loop {
        match (iter1.next(), iter2.next()) {
            (Some(lhs), Some(rhs)) => {
                if lhs.is_ascii_digit() && rhs.is_ascii_digit() {
                    loop {
                        match (
                            iter1.peek().copied().filter(char::is_ascii_digit),
                            iter2.peek().copied().filter(char::is_ascii_digit),
                        ) {
                            (Some(lhs), Some(rhs)) => {
                                if lhs != rhs {
                                    return lhs.cmp(&rhs);
                                } else {
                                    let _ = iter1.next();
                                    let _ = iter2.next();
                                }
                            }
                            (Some(_), None) => return Ordering::Greater,
                            (None, Some(_)) => return Ordering::Less,
                            (None, None) => break,
                        }
                    }
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
        let success = lexical_natural_cmp(lhs, rhs) == Ordering::Less;
        assert!(success, "Natural comparison {:?} < {:?} failed", lhs, rhs);

        let success = lexical_natural_cmp(rhs, lhs) == Ordering::Greater;
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

    nat_ordered("T-1", "T-5");
    nat_ordered("T-5", "T-20");
    nat_ordered("T-20a", "T-20b");

    nat_ordered("Ŧ-5", "T-20");
    nat_ordered("T-5", "Ŧ-20");
    nat_ordered("T-5", "Ŧ-5");
}
