use verf::petrinet::{PetriNet, Transition};
use verf::multiset::MultiSet;
use verf::backward_reach::is_backward_reach;


fn main() {
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

    println!("{:?}", is_backward_reach(&lock_petri_net, bad_state_gen, vec![initial_state]))
}
