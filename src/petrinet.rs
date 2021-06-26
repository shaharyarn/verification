use crate::multiset::MultiSet;
use std::hash::Hash;

pub struct Transition<T> {
    inputs: MultiSet<T>,
    outputs: MultiSet<T>,
}

impl<T: Eq + Hash + Clone> Transition<T> {
    pub fn new(inputs: MultiSet<T>, outputs: MultiSet<T>) -> Transition<T> {
        Transition {
            inputs,
            outputs,
        }
    }

    pub fn act(&self, multiset: &MultiSet<T>) -> MultiSet<T> {
        multiset.clone() - self.inputs.clone() + self.outputs.clone()
    }

    pub fn reverse_act(&self, multiset: &MultiSet<T>) -> MultiSet<T> {
        multiset.clone() + self.inputs.clone() - self.outputs.clone()
    }
}

#[cfg(test)]
mod tests_transitions {
    use super::Transition;
    use super::MultiSet;

    #[test]
    fn act() {
        let inputs: MultiSet<i32> = vec![1, 2].into_iter().collect();
        let outputs: MultiSet<i32> = vec![3].into_iter().collect();
        let transition = Transition::new(inputs, outputs);
        let state: MultiSet<i32> = vec![1, 2, 3].into_iter().collect();
        let expected_state: MultiSet<i32> = vec![3, 3].into_iter().collect();
        assert!(transition.act(&state) == expected_state);
    }

    #[test]
    fn reverse_act() {
        let inputs: MultiSet<i32> = vec![1, 2].into_iter().collect();
        let outputs: MultiSet<i32> = vec![3].into_iter().collect();
        let transition = Transition::new(inputs, outputs);
        let state: MultiSet<i32> = vec![1, 2, 3].into_iter().collect();
        let expected_state: MultiSet<i32> = vec![1, 1, 2, 2].into_iter().collect();
        assert!(transition.reverse_act(&state) == expected_state);
    }
}
