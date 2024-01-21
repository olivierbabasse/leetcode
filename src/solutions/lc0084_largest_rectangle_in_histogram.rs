//! <https://leetcode.com/problems/largest-rectangle-in-histogram/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn largest_rectangle_area(mut heights: Vec<i32>) -> i32 {
        heights.push(0);

        let mut stack = Vec::<i32>::new();
        let mut res = 0;

        for i in 0..heights.len() as i32 {
            while !stack.is_empty()
                && heights[i as usize] <= heights[*stack.last().unwrap() as usize]
            {
                let height = heights[stack.pop().unwrap() as usize];
                let left = *stack.last().unwrap_or(&-1);
                res = res.max((i - left - 1) * height);
            }
            stack.push(i);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
        assert_eq!(Solution::largest_rectangle_area(vec![2, 4]), 4);
    }
}
