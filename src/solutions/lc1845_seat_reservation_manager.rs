//! <https://leetcode.com/problems/seat-reservation-manager/>

use std::collections::BTreeSet;

struct SeatManager {
    available_seats: BTreeSet<i32>,
}

/// time-complexity : O(log(n))
/// space-complexity : O(n)
impl SeatManager {
    fn new(n: i32) -> Self {
        Self {
            available_seats: (1..=n).collect(),
        }
    }

    fn reserve(&mut self) -> i32 {
        self.available_seats.pop_first().unwrap()
    }

    fn unreserve(&mut self, seat_number: i32) {
        self.available_seats.insert(seat_number);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        let mut obj = SeatManager::new(5);
        assert_eq!(obj.reserve(), 1);
        assert_eq!(obj.reserve(), 2);
        obj.unreserve(2);
        assert_eq!(obj.reserve(), 2);
        assert_eq!(obj.reserve(), 3);
        assert_eq!(obj.reserve(), 4);
        assert_eq!(obj.reserve(), 5);
        obj.unreserve(5);
    }
}
