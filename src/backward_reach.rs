use std::fmt::Debug;

pub trait PredBasis<T> {
    fn pred_basis(&self, state: &T) -> Vec<T>;
}

fn exists_le_than<T: PartialOrd + Eq>(items: &Vec<T>, value: &T) -> bool {
    items.into_iter().any(|item| item <= value)
}

fn exists_ge_than<T: PartialOrd + Eq>(items: &Vec<T>, value: &T) -> bool {
    items.into_iter().any(|item| item >= value)
}

pub fn is_backward_reach<T: PartialOrd + Eq + Debug>(ts: &dyn PredBasis<T>, fin: T, init: Vec<T>) -> bool {
    let mut to_explore: Vec<T> = vec![fin];
    let mut explored: Vec<T> = Vec::new();

    while to_explore.len() > 0 {
        let curr = to_explore.pop().unwrap();
        println!("{:?}", curr);

        if exists_le_than(&explored, &curr) {
            continue;
        }

        if exists_ge_than(&init, &curr) {
            return true;
        }

        to_explore.extend(ts.pred_basis(&curr));
        explored.push(curr)
    }

    false
}


#[cfg(test)]
mod test_backward_reach {
    use crate::petrinet::{PetriNet, Transition};
    use crate::multiset::MultiSet;
    use super::is_backward_reach;

    #[test]
    fn test_lock_backward_reach() {
        let lock = String::from("Lock");
        let waiting = String::from("Waiting");
        let current = String::from("Current");

        let get_lock_inputs: MultiSet<&String> = vec![&lock, &waiting].into_iter().collect();
        let get_lock_outputs: MultiSet<&String> = vec![&current].into_iter().collect();
        let get_lock_transition = Transition::new(get_lock_inputs, get_lock_outputs);
        let release_lock_inputs: MultiSet<&String> = vec![&current].into_iter().collect();
        let release_lock_outputs: MultiSet<&String> = vec![&lock, &waiting].into_iter().collect();
        let release_lock_transition = Transition::new(release_lock_inputs, release_lock_outputs);
        let lock_petri_net = PetriNet::new(vec![get_lock_transition, release_lock_transition]);

        let initial_state: MultiSet<&String> = vec![&lock, &waiting, &waiting].into_iter().collect();
        let bad_state_gen: MultiSet<&String> = vec![&current, &current].into_iter().collect();

        assert!(!is_backward_reach(&lock_petri_net, bad_state_gen, vec![initial_state]))
    }
}
