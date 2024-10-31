//! <https://leetcode.com/problems/minimum-total-distance-traveled/>

struct Solution {}

/// time-complexity : O(len(factories)^2*len(robots))
/// space-complexity : O(len(factories)*len(robots))
impl Solution {
    fn rec(
        robot: usize,
        factory: usize,
        robots: &[i64],
        factories: &[i64],
        cache: &mut [Vec<Option<i64>>],
    ) -> i64 {
        if robot == robots.len() {
            return 0;
        }
        if factory == factories.len() {
            return i64::MAX / 2;
        }
        if let Some(res) = cache[robot][factory] {
            return res;
        }
        let with = (robots[robot] - factories[factory]).abs()
            + Self::rec(robot + 1, factory + 1, robots, factories, cache);
        let without = Self::rec(robot, factory + 1, robots, factories, cache);
        cache[robot][factory] = Some(with.min(without));
        with.min(without)
    }

    pub fn minimum_total_distance(mut robot: Vec<i32>, mut factory: Vec<Vec<i32>>) -> i64 {
        robot.sort_unstable();
        factory.sort_unstable();

        let robots = robot.into_iter().map(|v| v as i64).collect::<Vec<_>>();
        let factories = factory
            .into_iter()
            .flat_map(|v| std::iter::repeat(v[0] as i64).take(v[1] as usize))
            .collect::<Vec<_>>();
        let mut cache = vec![vec![None; factories.len()]; robots.len()];

        Self::rec(0, 0, &robots, &factories, &mut cache)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::minimum_total_distance(vec![0, 4, 6], vec![vec![2, 2], vec![6, 2]]),
            4
        );
        assert_eq!(
            Solution::minimum_total_distance(vec![1, -1], vec![vec![-2, 1], vec![2, 1]]),
            2
        );
    }
}
