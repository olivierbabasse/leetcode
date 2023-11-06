//! <https://leetcode.com/problems/design-a-number-container-system/>

use std::collections::{BTreeSet, HashMap};

struct NumberContainers {
    containers: HashMap<i32, i32>,
    container_index_by_number: HashMap<i32, BTreeSet<i32>>,
}

/// time-complexity : O(log(n))
/// space-complexity : O(n)
impl NumberContainers {
    fn new() -> Self {
        Self {
            containers: HashMap::new(),
            container_index_by_number: HashMap::new(),
        }
    }

    fn change(&mut self, index: i32, number: i32) {
        if let Some(number) = self.containers.get_mut(&index) {
            self.container_index_by_number
                .get_mut(number)
                .unwrap()
                .remove(&index);
        }
        self.containers.insert(index, number);
        self.container_index_by_number
            .entry(number)
            .and_modify(|x| {
                x.insert(index);
            })
            .or_insert({
                let mut set = BTreeSet::new();
                set.insert(index);
                set
            });
    }

    fn find(&self, number: i32) -> i32 {
        if let Some(set) = self.container_index_by_number.get(&number) {
            if let Some(container) = set.first() {
                return *container;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        let mut obj = NumberContainers::new();
        assert_eq!(obj.find(10), -1);
        obj.change(2, 10);
        obj.change(1, 10);
        obj.change(3, 10);
        obj.change(5, 10);
        assert_eq!(obj.find(10), 1);
        obj.change(1, 20);
        assert_eq!(obj.find(10), 2);
    }
}
