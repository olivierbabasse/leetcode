//! <https://leetcode.com/problems/insert-delete-getrandom-o1-duplicates-allowed/>

use rand::seq::SliceRandom;
use std::collections::{HashMap, HashSet};

struct RandomizedCollection {
    values: Vec<i32>,
    indexes: HashMap<i32, HashSet<usize>>,
}

impl RandomizedCollection {
    fn new() -> Self {
        Self {
            values: Vec::new(),
            indexes: HashMap::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        let index = self.values.len();
        self.values.push(val);
        self.indexes.entry(val).or_default().insert(index);
        self.indexes.get(&val).unwrap().len() == 1
    }

    fn remove(&mut self, val: i32) -> bool {
        if let Some(indexes) = self.indexes.get_mut(&val) {
            if !indexes.is_empty() {
                let last_index = self.values.len() - 1;
                let last_value = self.values[last_index];

                let index = *indexes.iter().next().unwrap();
                indexes.remove(&index);

                self.values.swap_remove(index);
                if index < self.values.len() {
                    self.indexes.entry(last_value).and_modify(|i| {
                        i.remove(&last_index);
                        i.insert(index);
                    });
                }

                return true;
            }
        }
        false
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
        let mut obj = RandomizedCollection::new();
        assert!(obj.insert(1));
        assert!(!obj.insert(1));
        assert!(!obj.remove(2));
        assert!(obj.insert(2));
        assert!([1, 2].contains(&obj.get_random()));
        assert!(obj.remove(1));
        assert!([1, 2].contains(&obj.get_random()));
    }
}
