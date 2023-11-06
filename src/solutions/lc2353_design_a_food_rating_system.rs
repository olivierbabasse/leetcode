//! <https://leetcode.com/problems/design-a-food-rating-system/>

use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap};

#[derive(PartialEq, Eq, Clone)]
struct FoodRating {
    name: String,
    rating: i32,
}

impl PartialOrd for FoodRating {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FoodRating {
    fn cmp(&self, other: &Self) -> Ordering {
        match other.rating.cmp(&self.rating) {
            Ordering::Equal => self.name.cmp(&other.name),
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
        }
    }
}

struct FoodRatings {
    food_to_foodrating: HashMap<String, FoodRating>,
    food_to_cuisine: HashMap<String, String>,
    cuisine_to_foodrating: HashMap<String, BTreeSet<FoodRating>>,
}

/// time-complexity : O(log(n))
/// space-complexity : O(n)
impl FoodRatings {
    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let len = foods.len();

        let mut food_to_foodrating = HashMap::new();
        let mut food_to_cuisine = HashMap::new();
        let mut cuisine_to_foodrating = HashMap::new();

        for i in 0..len {
            let fr = FoodRating {
                name: foods[i].clone(),
                rating: ratings[i],
            };
            food_to_foodrating.insert(foods[i].clone(), fr.clone());
            food_to_cuisine.insert(foods[i].clone(), cuisines[i].clone());
            cuisine_to_foodrating
                .entry(cuisines[i].clone())
                .or_insert(BTreeSet::new())
                .insert(fr);
        }

        Self {
            food_to_foodrating,
            food_to_cuisine,
            cuisine_to_foodrating,
        }
    }

    fn change_rating(&mut self, food: String, new_rating: i32) {
        let fr = self.food_to_foodrating.get_mut(&food).unwrap();
        let cuisine = self.food_to_cuisine.get(&food).unwrap();
        let ratings = self.cuisine_to_foodrating.get_mut(cuisine).unwrap();
        ratings.remove(fr);
        fr.rating = new_rating;
        ratings.insert(fr.clone());
    }

    fn highest_rated(&self, cuisine: String) -> String {
        let ratings = self.cuisine_to_foodrating.get(&cuisine).unwrap();
        ratings.first().unwrap().name.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        let mut obj = FoodRatings::new(
            vec![
                "kimchi".to_string(),
                "miso".to_string(),
                "sushi".to_string(),
                "moussaka".to_string(),
                "ramen".to_string(),
                "bulgogi".to_string(),
            ],
            vec![
                "korean".to_string(),
                "japanese".to_string(),
                "japanese".to_string(),
                "greek".to_string(),
                "japanese".to_string(),
                "korean".to_string(),
            ],
            vec![9, 12, 8, 15, 14, 7],
        );
        assert_eq!(obj.highest_rated("korean".into()), "kimchi");
        assert_eq!(obj.highest_rated("japanese".into()), "ramen");
        obj.change_rating("sushi".into(), 16);
        assert_eq!(obj.highest_rated("japanese".into()), "sushi");
        obj.change_rating("ramen".into(), 16);
        assert_eq!(obj.highest_rated("japanese".into()), "ramen");
    }
}
