//! <https://leetcode.com/problems/spiral-matrix-iv/>

type ListNode = crate::utils::list::ListNode<i32>;

struct Solution {}

/// time-complexity : O(rows*cols)
/// space-complexity : O(rows*cols)
impl Solution {
    pub fn spiral_matrix(m: i32, n: i32, mut head: Option<Box<ListNode>>) -> Vec<Vec<i32>> {
        let mut res = vec![vec![-1; n as usize]; m as usize];
        let (mut top, mut bottom, mut left, mut right) = (0i32, m - 1, 0i32, n - 1);

        let mut get_next_val = move || {
            if let Some(current) = head.take() {
                head = current.next;
                current.val
            } else {
                -1
            }
        };

        while top <= bottom && left <= right {
            for col in left..=right {
                res[top as usize][col as usize] = get_next_val();
            }
            top += 1;

            for row in top..=bottom {
                res[row as usize][right as usize] = get_next_val();
            }
            right -= 1;

            if top <= bottom {
                for col in (left..=right).rev() {
                    res[bottom as usize][col as usize] = get_next_val();
                }
                bottom -= 1;
            }

            if left <= right {
                for row in (top..=bottom).rev() {
                    res[row as usize][left as usize] = get_next_val();
                }
                left += 1;
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::{ListNode, Solution};

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::spiral_matrix(
                3,
                5,
                ListNode::from_vec(&[3, 0, 2, 6, 8, 1, 7, 9, 4, 2, 5, 5, 0])
            ),
            vec![[3, 0, 2, 6, 8], [5, 0, -1, -1, 1], [5, 2, 4, 9, 7]]
        );
    }
}
