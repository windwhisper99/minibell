use std::fmt::Debug;

pub trait Backtrack {
    type State: Debug + Default;
    type Candidate;

    /// Check if the current state is a complete solution
    fn is_solution(&self, state: &Self::State) -> bool;

    /// Generate possible candidates from the current state
    fn generate_candidates(&self, state: &Self::State) -> Vec<Self::Candidate>;

    /// Advance the state to the next candidate
    fn advance(&self, state: &Self::State, candidate: Self::Candidate) -> Option<Self::State>;
    /// Advance the state without a candidate
    fn advance_empty(&self, state: &Self::State) -> Option<Self::State>;

    /// Process a complete solution
    fn process_solution(&mut self, state: &Self::State);
}

pub fn iterate_backtrack<P: Backtrack>(problem: &mut P) {
    let mut stack = vec![P::State::default()];

    while let Some(state) = stack.pop() {
        if problem.is_solution(&state) {
            problem.process_solution(&state);
        } else {
            let candidates = problem.generate_candidates(&state);
            let is_empty = candidates.is_empty();
            for candidate in candidates {
                if let Some(next_state) = problem.advance(&state, candidate) {
                    stack.push(next_state);
                }
            }

            if !is_empty {
                if let Some(next_state) = problem.advance_empty(&state) {
                    stack.push(next_state);
                }
            }
        }
    }
}
