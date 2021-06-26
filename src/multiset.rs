use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::Hash;
use std::iter::FromIterator;
use std::ops::{Add, Sub};

pub struct MultiSet<T> {
    map: HashMap<T, i32>,
}

impl<T> MultiSet<T> {
    pub fn new() -> MultiSet<T> {
        MultiSet {
            map: HashMap::new(),
        }
    }
}

impl<T: Eq + Hash> MultiSet<T> {
    pub fn insert(&mut self, item: T) -> Option<i32> {
        self.repeat_insert(item, 1)
    }

    pub fn repeat_insert(&mut self, item: T, count: i32) -> Option<i32> {
        let new_count = self.get(&item).unwrap_or(0) + count;
        self.map.insert(item, new_count)
    }

    pub fn remove(&mut self, item: T) -> Option<i32> {
        self.repeat_remove(item, 1)
    }

    pub fn repeat_remove(&mut self, item: T, count: i32) -> Option<i32> {
        let current_count = self.get(&item);
        match current_count {
            None => None,
            Some(value) if count >= value => self.map.remove(&item),
            Some(value) => self.map.insert(item, value - count),
        }
    }

    pub fn get(&self, item: &T) -> Option<i32> {
        match self.map.get(item) {
            Some(value) => Some(*value),
            None => None,
        }
    }

    fn contained_in(&self, other: &MultiSet<T>) -> bool {
        self.map.len() <= other.map.len()
            && self.map.iter().all(|(key, value)| match other.get(key) {
                None => false,
                Some(other_value) => *value <= other_value,
            })
    }
}

impl<T: Eq + Hash> PartialEq for MultiSet<T> {
    fn eq(&self, other: &MultiSet<T>) -> bool {
        self.map.len() == other.map.len()
            && self
                .map
                .keys()
                .all(|key| self.map.get(key) == other.map.get(key))
    }
}

impl<T: Eq + Hash> PartialOrd for MultiSet<T> {
    fn partial_cmp(&self, other: &MultiSet<T>) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        };
        if self.contained_in(other) {
            return Some(Ordering::Less);
        };
        if other.contained_in(self) {
            return Some(Ordering::Greater);
        };
        None
    }
}

impl<T: Eq + Hash> FromIterator<T> for MultiSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut new_map: MultiSet<T> = MultiSet::new();
        iter.into_iter().for_each(|key| {
            new_map.insert(key);
        });
        new_map
    }
}

impl<T: Eq + Hash> Add for MultiSet<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut new_map: MultiSet<T> = MultiSet::new();
        other.map.into_iter().for_each(|(key, value)| {
            new_map.repeat_insert(key, value);
        });
        self.map.into_iter().for_each(|(key, value)| {
            new_map.repeat_insert(key, value);
        });
        new_map
    }
}

impl<T: Eq + Hash> Sub for MultiSet<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut new_map: MultiSet<T> = MultiSet::new();
        self.map.into_iter().for_each(|(key, value)| {
            let new_value = value - other.get(&key).unwrap_or(0);
            if new_value > 0 {
                new_map.repeat_insert(key, new_value);
            }
        });
        new_map
    }
}

impl<T: Eq + Hash + Clone> Clone for MultiSet<T> {
    fn clone(&self) -> Self {
        let mut new_map: MultiSet<T> = MultiSet::new();
        self.map.clone().into_iter().for_each(|(key, value)| {
            new_map.repeat_insert(key, value);
        });
        new_map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeat_insert() {
        let mut set: MultiSet<i32> = MultiSet::new();
        assert!(set.repeat_insert(1, 3).is_none());
        assert!(set.repeat_insert(2, 2).is_none());
        assert_eq!(set.get(&1).unwrap(), 3);
        assert_eq!(set.get(&2).unwrap(), 2);
    }

    #[test]
    fn insert() {
        let mut set: MultiSet<i32> = MultiSet::new();
        assert!(set.insert(5).is_none());
        assert_eq!(set.get(&5).unwrap(), 1);
    }

    #[test]
    fn multiple_insert() {
        let mut set: MultiSet<i32> = MultiSet::new();
        assert!(set.insert(5).is_none());
        assert_eq!(set.insert(5).unwrap(), 1);
        assert!(set.insert(2).is_none());
        assert_eq!(set.get(&5).unwrap(), 2);
        assert_eq!(set.get(&2).unwrap(), 1);
    }

    #[test]
    fn multiset_from_iter() {
        let set: MultiSet<i32> = vec![1, 2, 2, 4].into_iter().collect();
        assert_eq!(set.get(&1).unwrap(), 1);
        assert_eq!(set.get(&2).unwrap(), 2);
        assert_eq!(set.get(&4).unwrap(), 1);
    }

    #[test]
    fn repeat_remove() {
        let mut set: MultiSet<i32> = vec![1, 2, 2, 4].into_iter().collect();
        assert_eq!(set.repeat_remove(2, 3).unwrap(), 2);
        assert!(set.get(&2).is_none());
    }

    #[test]
    fn remove() {
        let mut set: MultiSet<i32> = vec![1, 2, 2, 4].into_iter().collect();
        assert_eq!(set.remove(2).unwrap(), 2);
        assert_eq!(set.remove(1).unwrap(), 1);
        assert!(set.remove(3).is_none());
        assert_eq!(set.get(&2).unwrap(), 1);
    }

    #[test]
    fn equal_empty_multisets() {
        let set: MultiSet<i32> = MultiSet::new();
        let other: MultiSet<i32> = MultiSet::new();
        assert!(set == other);
    }

    #[test]
    fn equal_multisets() {
        let set: MultiSet<i32> = vec![1, 2, 2, 4].into_iter().collect();
        let other: MultiSet<i32> = vec![4, 2, 1, 2].into_iter().collect();
        assert!(set == other);
    }

    #[test]
    fn unequal_multisets() {
        let set: MultiSet<i32> = vec![4, 2, 1, 2].into_iter().collect();
        let other: MultiSet<i32> = vec![4, 2, 2].into_iter().collect();
        assert!(set != other);
    }

    #[test]
    fn order_on_contained_multisets() {
        let set: MultiSet<i32> = vec![4, 2, 1, 2].into_iter().collect();
        let other: MultiSet<i32> = vec![4, 1, 2].into_iter().collect();
        assert!(set > other);
    }

    #[test]
    fn order_on_equal_multisets() {
        let set: MultiSet<i32> = vec![4, 2, 1, 2].into_iter().collect();
        let other: MultiSet<i32> = vec![4, 1, 2, 2].into_iter().collect();
        assert!(!(set > other) && !(set < other));
    }

    #[test]
    fn add_sets() {
        let set: MultiSet<i32> = vec![5, 2].into_iter().collect();
        let other: MultiSet<i32> = vec![5, 3].into_iter().collect();
        let expected_set: MultiSet<i32> = vec![5, 5, 3, 2].into_iter().collect();
        let added_set = set + other;
        assert!(added_set == expected_set);
    }

    #[test]
    fn subtract_sets() {
        let set: MultiSet<i32> = vec![5, 2].into_iter().collect();
        let other: MultiSet<i32> = vec![5, 3].into_iter().collect();
        let expected_set: MultiSet<i32> = vec![2].into_iter().collect();
        let subtracted_set = set - other;
        assert!(subtracted_set == expected_set);
    }
}
