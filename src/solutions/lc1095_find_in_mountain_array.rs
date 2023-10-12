//! <https://leetcode.com/problems/find-in-mountain-array/>

use crate::utils::mountainarray::MountainArray;

struct Solution {}

impl Solution {
    /// time-complexity : O(log(n))
    /// space-complexity : O(1)
    pub fn find_in_mountain_array(target: i32, mountain_array: &MountainArray) -> i32 {
        fn is_asc_part(num: i32, index: i32, mountain_array: &MountainArray) -> bool {
            let len = mountain_array.length();
            if len == 1 || index == 0 {
                true
            } else {
                num > mountain_array.get(index - 1)
            }
        }

        let mut begin = 0;
        let mut end = mountain_array.length() - 1;
        loop {
            let middle = (begin + end) / 2;
            let num = mountain_array.get(middle);

            if !is_asc_part(num, middle, mountain_array) {
                end = middle - 1;
            } else if num == target {
                return middle;
            } else if num > target {
                end = middle - 1;
            } else {
                begin = middle + 1;
            }

            if begin > end {
                break;
            }
        }

        let mut begin = 0;
        let mut end = mountain_array.length() - 1;
        loop {
            let middle = (begin + end) / 2;
            let num = mountain_array.get(middle);

            if is_asc_part(num, middle, mountain_array) {
                begin = middle + 1;
            } else if num == target {
                return middle;
            } else if num > target {
                begin = middle + 1;
            } else {
                end = middle - 1;
            }

            if begin > end {
                break;
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::{MountainArray, Solution};

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::find_in_mountain_array(2, &MountainArray::new(&[1, 5, 2])),
            2
        );
        assert_eq!(
            Solution::find_in_mountain_array(3, &MountainArray::new(&[1, 2, 3, 4, 5, 3, 1])),
            2
        );
        assert_eq!(
            Solution::find_in_mountain_array(3, &MountainArray::new(&[0, 1, 2, 4, 2, 1])),
            -1
        );
        assert_eq!(
            Solution::find_in_mountain_array(
                116,
                &MountainArray::new(&[
                    22, 23, 32, 42, 46, 52, 58, 61, 68, 76, 77, 79, 89, 95, 97, 104, 106, 112, 116,
                    120, 125, 127, 137, 141, 142, 148, 150, 157, 148, 147, 146, 136, 128, 126, 116,
                    106, 100, 93, 87, 85, 84, 80, 70, 65, 62, 52, 49, 47, 44, 34, 28, 25, 15, 6, 0
                ])
            ),
            18
        );
        assert_eq!(
            Solution::find_in_mountain_array(
                101,
                &MountainArray::new(&[
                    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                    23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42,
                    43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62,
                    63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82,
                    83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101,
                    100, 99, 98, 97, 96, 95, 94, 93, 92, 91, 90, 89, 88, 87, 86, 85, 84, 83, 82
                ])
            ),
            100
        );
    }
}
