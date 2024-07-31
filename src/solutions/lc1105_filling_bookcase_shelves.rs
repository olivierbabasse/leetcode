//! <https://leetcode.com/problems/filling-bookcase-shelves/>

struct Solution {}

/// time-complexity : O(n*width)
/// space-complexity : O(n*width)
impl Solution {
    fn rec(
        books: &[Vec<i32>],
        shelf_width: i32,
        n: usize,
        left: i32,
        max_height: i32,
        cache: &mut [Vec<Option<i32>>],
    ) -> i32 {
        let (width, height) = (books[n][0], books[n][1]);

        if n == books.len() - 1 {
            if left >= width {
                return max_height.max(height);
            }
            return max_height + height;
        }

        if let Some(val) = cache[n][left as usize] {
            val
        } else {
            let height1 = max_height
                + Self::rec(
                    books,
                    shelf_width,
                    n + 1,
                    shelf_width - width,
                    height,
                    cache,
                );
            let mut height2 = i32::MAX;
            if left >= width {
                height2 = Self::rec(
                    books,
                    shelf_width,
                    n + 1,
                    left - width,
                    max_height.max(height),
                    cache,
                );
            }
            cache[n][left as usize] = Some(height1.min(height2));
            height1.min(height2)
        }
    }

    pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
        let mut cache = vec![vec![None; shelf_width as usize + 1]; books.len()];
        Self::rec(&books, shelf_width, 0, shelf_width, 0, &mut cache)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::min_height_shelves(
                vec![
                    vec![1, 1],
                    vec![2, 3],
                    vec![2, 3],
                    vec![1, 1],
                    vec![1, 1],
                    vec![1, 1],
                    vec![1, 2]
                ],
                4
            ),
            6
        );
        assert_eq!(
            Solution::min_height_shelves(vec![vec![1, 3], vec![2, 4], vec![3, 2]], 6),
            4
        );
    }
}
