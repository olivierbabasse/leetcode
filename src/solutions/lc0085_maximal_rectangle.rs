//! <https://leetcode.com/problems/maximal-rectangle/>

struct Solution {}

/// time-complexity : O(n^2)
/// space-complexity : O(n)
impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let mut heights = vec![0; matrix[0].len() + 1];
        let mut res = 0;
        for l in matrix {
            for (i, &c) in l.iter().enumerate() {
                heights[i] = if c == '0' { 0 } else { heights[i] + 1 };
            }

            let mut stack = Vec::<i32>::new();
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
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::maximal_rectangle(vec![
                vec!['1', '0', '1', '0', '0'],
                vec!['1', '0', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '0', '0', '1', '0']
            ]),
            6
        );
    }
}
