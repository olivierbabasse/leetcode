pub mod list;
pub mod mountainarray;
pub mod tree;

use std::collections::HashMap;
use std::hash::Hash;

fn item_counts<T>(items: &[T]) -> HashMap<&T, usize>
where
    T: Eq + Hash,
{
    let mut counts = HashMap::new();
    for i in items {
        *counts.entry(i).or_insert(0) += 1;
    }
    counts
}

pub fn array_eq<T>(a: &[T], b: &[T]) -> bool
where
    T: Eq + Hash,
{
    item_counts(a) == item_counts(b)
}

#[cfg(test)]
mod tests {
    use super::array_eq;

    #[test]
    fn test_array_eq() {
        assert!(array_eq::<i32>(&[], &[]));
        assert!(array_eq(&[1], &[1]));
        assert!(!array_eq(&[1], &[2]));
        assert!(array_eq(&[1, 2], &[2, 1]));
        assert!(!array_eq(&[1], &[1, 1]));
    }
}
