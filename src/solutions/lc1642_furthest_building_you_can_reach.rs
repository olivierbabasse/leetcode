//! <https://leetcode.com/problems/furthest-building-you-can-reach/>

use std::collections::BinaryHeap;

struct Solution {}

/// time-complexity : O(n*log(n))
/// space-complexity : O(n)
impl Solution {
    pub fn furthest_building(heights: Vec<i32>, mut bricks: i32, mut ladders: i32) -> i32 {
        let mut queue = BinaryHeap::new();

        let mut i = 0;
        while i < heights.len() - 1 {
            let h = heights[i + 1] - heights[i];
            if h <= 0 {
                i += 1;
                continue;
            }

            bricks -= h;
            queue.push(h);

            if bricks < 0 {
                bricks += queue.pop().unwrap();
                ladders -= 1;
            }

            if ladders < 0 {
                break;
            }

            i += 1;
        }

        i as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::furthest_building(vec![4, 2, 7, 6, 9, 14, 12], 5, 1),
            4
        );
        assert_eq!(
            Solution::furthest_building(vec![4, 12, 2, 7, 3, 18, 20, 3, 19], 10, 2),
            7
        );
        assert_eq!(Solution::furthest_building(vec![14, 3, 19, 3], 17, 0), 3);
    }
}
