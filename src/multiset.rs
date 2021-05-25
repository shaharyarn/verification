use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::Hash;

pub struct MultiSet<T> {
    map: HashMap<T, i32>,
}

impl<T: Eq + Hash> MultiSet<T> {
    pub fn new() -> MultiSet<T> {
        MultiSet {
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, item: T) -> Option<i32> {
        let new_count = self.get(&item).unwrap_or(0) + 1;
        self.map.insert(item, new_count)
    }

    pub fn remove(&mut self, item: T) -> Option<i32> {
        let current_count = self.get(&item);
        match current_count {
            None => None,
            Some(1) => self.map.remove(&item),
            Some(value) => self.map.insert(item, value - 1),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_and_get() {
        let mut set: MultiSet<i32> = MultiSet::new();
        set.insert(5);
        assert_eq!(set.get(&5).unwrap(), 1);
    }

    #[test]
    fn multiple_insert_and_get() {
        let mut set: MultiSet<i32> = MultiSet::new();
        set.insert(5);
        set.insert(5);
        set.insert(5);
        assert_eq!(set.get(&5).unwrap(), 3);
    }

    #[test]
    fn multiple_insert_and_get_with_different_elements() {
        let mut set: MultiSet<i32> = MultiSet::new();
        set.insert(5);
        set.insert(2);
        set.insert(5);
        set.insert(3);
        assert_eq!(set.get(&5).unwrap(), 2);
        assert_eq!(set.get(&2).unwrap(), 1);
        assert_eq!(set.get(&3).unwrap(), 1);
    }

    #[test]
    fn remove_and_insert() {
        let mut set: MultiSet<i32> = MultiSet::new();
        set.insert(5);
        set.insert(2);
        set.insert(5);
        assert_eq!(set.remove(5).unwrap(), 2);
        assert_eq!(set.remove(2).unwrap(), 1);
        assert!(set.remove(3).is_none());
        assert_eq!(set.get(&5).unwrap(), 1);
    }

    #[test]
    fn equal_empty_multisets() {
        let set: MultiSet<i32> = MultiSet::new();
        let other: MultiSet<i32> = MultiSet::new();
        assert!(set == other);
    }

    #[test]
    fn equal_multisets() {
        let mut set: MultiSet<i32> = MultiSet::new();
        set.insert(3);
        set.insert(5);
        set.insert(5);
        set.insert(2);
        let mut other: MultiSet<i32> = MultiSet::new();
        other.insert(5);
        other.insert(2);
        other.insert(5);
        other.insert(3);
        assert!(set == other);
    }

    #[test]
    fn unequal_multisets() {
        let mut set: MultiSet<i32> = MultiSet::new();
        set.insert(3);
        set.insert(5);
        set.insert(5);
        set.insert(2);
        let mut other: MultiSet<i32> = MultiSet::new();
        other.insert(5);
        other.insert(5);
        other.insert(3);
        assert!(set != other);
    }

    #[test]
    fn order_on_multisets() {
        let mut set: MultiSet<i32> = MultiSet::new();
        set.insert(5);
        set.insert(3);
        set.insert(5);
        let mut other: MultiSet<i32> = MultiSet::new();
        other.insert(5);
        assert!(set > other);
        other.insert(2);
        assert!(!(set > other) && !(set < other));
    }
}
