//! <https://leetcode.com/problems/find-the-city-with-the-smallest-number-of-neighbors-at-a-threshold-distance/>

struct Solution {}

/// time-complexity : O(n^3)
/// space-complexity : O(n^2)
impl Solution {
    pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
        let n = n as usize;
        let mut distances = vec![vec![i32::MAX / 2; n]; n];
        for (i, d) in distances.iter_mut().enumerate() {
            d[i] = 0;
        }

        for edge in edges {
            let (start, stop, dist) = (edge[0] as usize, edge[1] as usize, edge[2]);
            distances[start][stop] = dist;
            distances[stop][start] = dist;
        }

        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    distances[i][j] = distances[i][j].min(distances[i][k] + distances[k][j]);
                }
            }
        }

        let mut best_city = n;
        let mut best_count = n;
        for (i, dist) in distances.iter_mut().enumerate() {
            let mut count = 0;
            for (j, d) in dist.iter().enumerate() {
                if i == j {
                    continue;
                }
                if *d <= distance_threshold {
                    count += 1;
                }
            }
            if count <= best_count {
                best_count = count;
                best_city = i;
            }
        }

        best_city as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::find_the_city(
                4,
                vec![vec![0, 1, 3], vec![1, 2, 1], vec![1, 3, 4], vec![2, 3, 1]],
                4
            ),
            3
        );
        assert_eq!(
            Solution::find_the_city(
                5,
                vec![
                    vec![0, 1, 2],
                    vec![0, 4, 8],
                    vec![1, 2, 3],
                    vec![1, 4, 2],
                    vec![2, 3, 1],
                    vec![3, 4, 1]
                ],
                2
            ),
            0
        );
    }
}
