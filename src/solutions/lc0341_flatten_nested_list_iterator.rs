//! <https://leetcode.com/problems/flatten-nested-list-iterator/>

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

struct NestedIterator {
    values: Vec<i32>,
    cur_pos: usize,
}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl NestedIterator {
    fn new(nested_list: Vec<NestedInteger>) -> Self {
        let mut values = Vec::new();
        Self::extract(&nested_list, &mut values);
        Self { values, cur_pos: 0 }
    }

    fn extract(nested_list: &Vec<NestedInteger>, values: &mut Vec<i32>) {
        for value in nested_list {
            match value {
                NestedInteger::Int(v) => values.push(*v),
                NestedInteger::List(l) => Self::extract(l, values),
            }
        }
    }

    fn next(&mut self) -> i32 {
        let v = self.values[self.cur_pos];
        self.cur_pos += 1;
        v
    }

    fn has_next(&self) -> bool {
        self.cur_pos < self.values.len()
    }
}
