#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    match (a.len(), b.len()) {
        (0, 0) => Comparison::Equal,
        (0, _) => Comparison::Sublist,
        (_, 0) => Comparison::Superlist,
        (m, n) if (m > n) => {
            if a.windows(n).any(|v| v == b) {
                Comparison::Superlist
            } else {
                Comparison::Unequal
            }
        }
        (m, n) if (m < n) => {
            if b.windows(m).any(|v| v == a) {
                Comparison::Sublist
            } else {
                Comparison::Unequal
            }
        }
        (_, _) => {
            if a == b {
                Comparison::Equal
            } else {
                Comparison::Unequal
            }
        }
    }
}
