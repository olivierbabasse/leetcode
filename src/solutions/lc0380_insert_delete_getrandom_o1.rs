//! <https://leetcode.com/problems/insert-delete-getrandom-o1/>

use rand::seq::SliceRandom;
use std::collections::HashMap;

struct RandomizedSet {
    values: Vec<i32>,
    indexes: HashMap<i32, usize>,
}

impl RandomizedSet {
    fn new() -> Self {
        Self {
            values: Vec::new(),
            indexes: HashMap::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if let std::collections::hash_map::Entry::Vacant(e) = self.indexes.entry(val) {
            e.insert(self.values.len());
            self.values.push(val);
            true
        } else {
            false
        }
    }

    fn remove(&mut self, val: i32) -> bool {
        if let Some(index) = self.indexes.remove(&val) {
            self.values.swap_remove(index);
            if index < self.values.len() {
                self.indexes.entry(self.values[index]).and_modify(|i| {
                    *i = index;
                });
            }
            true
        } else {
            false
        }
    }

    fn get_random(&self) -> i32 {
        *self.values.choose(&mut rand::thread_rng()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        let mut obj = RandomizedSet::new();
        assert!(obj.insert(1));
        assert!(!obj.remove(2));
        assert!(obj.insert(2));
        assert!([1, 2].contains(&obj.get_random()));
        assert!(obj.remove(1));
        assert!(!obj.insert(2));
        assert_eq!(obj.get_random(), 2);
    }
}
