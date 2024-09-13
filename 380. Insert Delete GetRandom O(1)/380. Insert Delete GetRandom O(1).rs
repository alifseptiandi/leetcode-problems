use std::collections::HashMap;
use rand::Rng;

struct RandomizedSet {
    elements: Vec<i32>,
    index_map: HashMap<i32, usize>,
}

impl RandomizedSet {
    fn new() -> Self {
        RandomizedSet {
            elements: Vec::new(),
            index_map: HashMap::new(),
        }
    }
    
    fn insert(&mut self, val: i32) -> bool {
        if self.index_map.contains_key(&val) {
            return false;
        }
        
        self.elements.push(val);
        self.index_map.insert(val, self.elements.len() - 1);
        true
    }
    
    fn remove(&mut self, val: i32) -> bool {
        if let Some(&index) = self.index_map.get(&val) {
            let last_element = *self.elements.last().unwrap();
            self.elements[index] = last_element;
            self.index_map.insert(last_element, index);
            self.elements.pop();
            self.index_map.remove(&val);
            true
        } else {
            false
        }
    }
    
    fn get_random(&self) -> i32 {
        let random_index = rand::thread_rng().gen_range(0..self.elements.len());
        self.elements[random_index]
    }
}

// For testing purposes
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_randomized_set() {
        let mut set = RandomizedSet::new();
        assert_eq!(set.insert(1), true);
        assert_eq!(set.remove(2), false);
        assert_eq!(set.insert(2), true);
        let random = set.get_random();
        assert!(random == 1 || random == 2);
        assert_eq!(set.remove(1), true);
        assert_eq!(set.insert(2), false);
        assert_eq!(set.get_random(), 2);
    }
}