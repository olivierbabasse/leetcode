//! <https://leetcode.com/problems/minimized-maximum-of-products-distributed-to-any-store/>

use std::collections::BinaryHeap;

struct Solution {}

#[derive(Eq, PartialEq, Debug)]
struct QuantitiesStores {
    quantity: i32,
    stores: i32,
}

impl Ord for QuantitiesStores {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.quantity * other.stores).cmp(&(self.stores * other.quantity))
    }
}

impl PartialOrd for QuantitiesStores {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// time-complexity : O(n*log(n))
/// space-complexity : O(n)
impl Solution {
    pub fn minimized_maximum(n: i32, quantities: Vec<i32>) -> i32 {
        let mut quantities_stores = quantities
            .iter()
            .map(|q| QuantitiesStores {
                quantity: *q,
                stores: 1,
            })
            .collect::<BinaryHeap<_>>();

        for _ in 0..n - (quantities.len() as i32) {
            if let Some(qs) = quantities_stores.pop() {
                quantities_stores.push(QuantitiesStores {
                    quantity: qs.quantity,
                    stores: qs.stores + 1,
                });
            }
        }

        let top = quantities_stores.pop().unwrap();
        (top.quantity as f64 / top.stores as f64).ceil() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::minimized_maximum(6, vec![11, 6]), 3);
        assert_eq!(Solution::minimized_maximum(7, vec![15, 10, 10]), 5);
        assert_eq!(Solution::minimized_maximum(1, vec![10000]), 10000);
    }
}
