use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

use crate::{
    backtrack::{iterate_backtrack, Backtrack},
    data::{Job, Member, Role},
};

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Assignment(usize, Job);

#[derive(Debug, Clone)]
struct State {
    member_index: usize,
    assignments: Vec<Assignment>,
}

impl Default for State {
    fn default() -> Self {
        State {
            member_index: 0,
            assignments: Vec::new(),
        }
    }
}

struct Resolver<'a> {
    roles: HashMap<Role, usize>,
    members: &'a Vec<Member>,

    result: Vec<Combination>,
}

impl<'a> Resolver<'a> {
    fn new(members: &'a Vec<Member>) -> Self {
        Resolver {
            roles: vec![
                (Role::Tank, 2),
                (Role::PureHealer, 1),
                (Role::ShieldHealer, 1),
                (Role::Melee, 2),
                (Role::Caster, 1),
                (Role::Ranged, 1),
            ]
            .into_iter()
            .collect::<HashMap<_, _>>(),
            members,
            result: Vec::new(),
        }
    }
}

impl<'a> Backtrack for Resolver<'a> {
    type State = State;
    type Candidate = Assignment;

    fn is_solution(&self, state: &State) -> bool {
        state.assignments.len() >= self.members.len()
    }

    fn advance(&self, state: &Self::State, candidate: Self::Candidate) -> Option<Self::State> {
        let mut new_state = state.clone();
        new_state.assignments.push(candidate);
        new_state.member_index += 1;

        Some(new_state)
    }

    fn advance_empty(&self, state: &Self::State) -> Option<Self::State> {
        let mut new_state = state.clone();
        new_state.member_index += 1;

        Some(new_state)
    }

    fn generate_candidates(&self, state: &Self::State) -> Vec<Self::Candidate> {
        let Some(member) = self.members.get(state.member_index) else {
            // No more members
            return vec![];
        };

        let mut candidates = Vec::new();
        // Collect assigned jobs and roles
        let mut assigned_roles = HashMap::new();
        let mut assigned_jobs = HashSet::new();

        for assignment in state.assignments.iter() {
            assigned_roles
                .entry(assignment.1.role())
                .and_modify(|count| *count += 1)
                .or_insert(1);
            assigned_jobs.insert(assignment.1.clone());
        }

        // Check if roles are filled
        let mut available_roles = HashSet::new();
        for (role, count) in self.roles.iter() {
            if let Some(role_count) = assigned_roles.get(role) {
                if role_count >= count {
                    continue;
                }
            }

            available_roles.insert(role);
        }

        // If no roles are available, skip the member
        if available_roles.is_empty() {
            return vec![];
        }

        for job in member.jobs.iter() {
            let job = job.job.clone();

            // Check if role is filled
            if !available_roles.contains(&job.role()) {
                continue;
            }

            // Check if the job is already assigned
            if assigned_jobs.contains(&job) {
                continue;
            }
            candidates.push(Assignment(state.member_index, job));
        }

        candidates
    }

    fn process_solution(&mut self, state: &Self::State) {
        self.result.push(Combination::from_assignment(
            &state.assignments,
            &self.members,
        ));
    }
}

pub fn resolve_combinations(members: &Vec<Member>) -> Vec<Combination> {
    let mut resolver = Resolver::new(members);
    iterate_backtrack(&mut resolver);

    // Sort the result
    resolver
        .result
        .sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    resolver.result
}

#[derive(Clone)]
pub struct Combination {
    pub assigned: Vec<(String, Job)>,
    pub score: f32,
}

impl PartialEq for Combination {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}
impl Eq for Combination {}

// Implement ordering for sorting
impl PartialOrd for Combination {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl Ord for Combination {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.partial_cmp(&other.score).unwrap()
    }
}

impl Combination {
    fn from_assignment(assignments: &Vec<Assignment>, members: &Vec<Member>) -> Self {
        let mut score = 0.0;
        let mut assigned = Vec::new();
        for Assignment(member_index, job) in assignments {
            let member = members.get(*member_index).unwrap();
            score += member
                .jobs
                .iter()
                .find(|j| j.job == *job)
                .unwrap()
                .confidence;
            assigned.push((member.id.clone(), job.clone()));
        }

        Combination {
            score: score / assigned.len() as f32,
            assigned,
        }
    }
}
