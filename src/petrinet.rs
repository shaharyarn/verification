use crate::backward_reach::PredBasis;
use crate::multiset::MultiSet;
use std::hash::Hash;

#[derive(Clone)]
pub struct Transition<T: Eq + Hash + Clone> {
    inputs: MultiSet<T>,
    outputs: MultiSet<T>,
}

impl<T: Eq + Hash + Clone> Transition<T> {
    pub fn new(inputs: MultiSet<T>, outputs: MultiSet<T>) -> Transition<T> {
        Transition { inputs, outputs }
    }

    pub fn act(&self, multiset: &MultiSet<T>) -> MultiSet<T> {
        multiset.clone() - self.inputs.clone() + self.outputs.clone()
    }

    pub fn reverse_act(&self, multiset: &MultiSet<T>) -> MultiSet<T> {
        multiset.clone() + self.inputs.clone() - self.outputs.clone()
    }
}

pub struct PetriNet<'a> {
    transitions: Vec<Transition<&'a String>>,
}

impl<'a> PetriNet<'a> {
    pub fn new(transitions: Vec<Transition<&'a String>>) -> PetriNet<'a> {
        PetriNet {transitions}
    }
}

impl<'a> PredBasis<MultiSet<&'a String>> for PetriNet<'a> {
    fn pred_basis(&self, state: &MultiSet<&'a String>) -> Vec<MultiSet<&'a String>> {
        self.transitions
            .clone()
            .into_iter()
            .map(|transition| transition.reverse_act(state))
            .collect()
    }
}

#[cfg(test)]
mod tests_transitions {
    use super::MultiSet;
    use super::Transition;

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

#[cfg(test)]
mod tests_petri_net {
    use super::MultiSet;
    use super::Transition;
    use super::PetriNet;
    use super::PredBasis;

    fn compare_unordered_vec<T: Eq + Clone>(first: Vec<T>, second: Vec<T>) -> bool {
        for i in &first {
            let mut found_match = false;
            for j in &second {
                if j == i {
                    found_match = true;
                }
            }
            if !found_match {
                return false;
            }
        }
        true
    }
    
    #[test]
    fn pred_basis() {
        let first_place = String::from("First");
        let second_place = String::from("Second");
        let third_place = String::from("Third");
        let first_inputs: MultiSet<&String> = vec![&first_place, &second_place].into_iter().collect();
        let first_outputs: MultiSet<&String> = vec![&third_place].into_iter().collect();
        let first_transition = Transition::new(first_inputs, first_outputs);
        let second_inputs: MultiSet<&String> = vec![&third_place].into_iter().collect();
        let second_outputs: MultiSet<&String> = vec![&first_place, &second_place].into_iter().collect();
        let second_transition = Transition::new(second_inputs, second_outputs);
        let petri_net = PetriNet::new(vec![first_transition, second_transition]);

        let initial_state: MultiSet<&String> = vec![&second_place, &second_place].into_iter().collect();
        let first_expected_state: MultiSet<&String> = vec![&second_place, &third_place].into_iter().collect();
        let second_expected_state: MultiSet<&String> = vec![&second_place, &second_place, &second_place, &first_place].into_iter().collect();

        assert!(compare_unordered_vec(petri_net.pred_basis(&initial_state), vec![first_expected_state, second_expected_state]))

    }
}
