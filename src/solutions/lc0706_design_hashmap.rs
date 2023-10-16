//! <https://leetcode.com/problems/design-hashmap/>

struct MyHashMap {
    entries: Vec<Option<Vec<(i32, i32)>>>,
}

impl MyHashMap {
    const MODULO: i32 = 997;

    fn new() -> Self {
        let entries = vec![None; Self::MODULO as usize];
        MyHashMap { entries }
    }

    fn put(&mut self, key: i32, value: i32) {
        let index = key % Self::MODULO;
        if let Some(values) = &mut self.entries[index as usize] {
            values.retain(|(k, _)| k != &key);
            values.push((key, value));
        } else {
            self.entries[index as usize] = Some(vec![(key, value)]);
        }
    }

    fn get(&self, key: i32) -> i32 {
        let index = key % Self::MODULO;
        if let Some(values) = &self.entries[index as usize] {
            if let Some(value) = values.iter().find(|(k, _)| k == &key) {
                return value.1;
            }
        }
        -1
    }

    fn remove(&mut self, key: i32) {
        let index = key % Self::MODULO;
        if let Some(values) = &mut self.entries[index as usize] {
            values.retain(|(k, _)| k != &key);
        }
    }
}
