use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

use crate::{
    backtrack::{iterate_backtrack, Backtrack},
    data::{Job, Member, Role},
};

#[derive(Debug, Clone)]
pub struct CombinationConfig {
    pub roles: HashMap<Role, usize>,
    pub nslots: usize,
}

impl CombinationConfig {
    pub fn new(roles: HashMap<Role, usize>) -> Self {
        let nslots = roles.values().sum();
        CombinationConfig { roles, nslots }
    }
}

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
    combination: &'a CombinationConfig,
    members: &'a Vec<Member>,

    member_weight: f32,
    confidence_weight: f32,

    min_members: Option<usize>,
    highest_score: f32,
    combinations: Vec<Combination>,
}

impl<'a> Resolver<'a> {
    fn new(
        members: &'a Vec<Member>,
        combination: &'a CombinationConfig,

        member_weight: f32,
        confidence_weight: f32,

        min_members: Option<usize>,
    ) -> Self {
        Resolver {
            combination,
            members,

            member_weight,
            confidence_weight,

            min_members,
            highest_score: 0.0,
            combinations: Vec::new(),
        }
    }
}

impl<'a> Backtrack for Resolver<'a> {
    type State = State;
    type Candidate = Assignment;

    fn is_solution(&self, state: &State) -> bool {
        if state.assignments.len() == self.combination.nslots {
            return true;
        }

        // Check no more members are available and minimum members are satisfied
        if let Some(min_members) = self.min_members {
            if state.assignments.len() >= self.members.len()
                && state.assignments.len() >= min_members
            {
                return true;
            }
        }

        false
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
            let Some(role) = self
                .combination
                .roles
                .keys()
                .find(|role| assignment.1.satifies(role))
            else {
                continue;
            };
            assigned_roles
                .entry(role)
                .and_modify(|count| *count += 1)
                .or_insert(1);
            assigned_jobs.insert(assignment.1.clone());
        }

        // Check if roles are filled
        let mut available_roles = HashSet::new();
        for (role, count) in self.combination.roles.iter() {
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

            // Check if the job is already assigned
            if assigned_jobs.contains(&job) {
                continue;
            }

            // Check if job is fill available
            let available = available_roles.iter().any(|role| job.satifies(role));
            if available {
                candidates.push(Assignment(state.member_index, job));
            }
        }

        candidates
    }

    fn process_solution(&mut self, state: &Self::State) {
        let mut confidence_score = 0.0;
        let mut assigned = Vec::new();
        for Assignment(member_index, job) in &state.assignments {
            let member = self.members.get(*member_index).unwrap();
            confidence_score += member
                .jobs
                .iter()
                .find(|j| j.job == *job)
                .unwrap()
                .confidence;
            assigned.push((member.id.clone(), job.clone()));
        }

        // Percent of members assigned compared to total slots
        let member_score = assigned.len() as f32 / self.combination.nslots as f32;
        let confidence_score = confidence_score / assigned.len() as f32;

        let combination = Combination {
            score: member_score * self.member_weight + confidence_score * self.confidence_weight,
            assigned,
        };

        // Replace if the score is higher
        if combination.score > self.highest_score {
            self.highest_score = combination.score;
            self.combinations = vec![combination];
        } else if combination.score == self.highest_score {
            self.combinations.push(combination);
        }
    }
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

pub fn resolve_combinations(
    members: &Vec<Member>,
    combination: &CombinationConfig,
    member_weight: f32,
    confidence_weight: f32,
    min_members: Option<usize>,
) -> (f32, Vec<Combination>) {
    let mut resolver = Resolver::new(
        members,
        combination,
        member_weight,
        confidence_weight,
        min_members,
    );
    iterate_backtrack(&mut resolver);

    (resolver.highest_score, resolver.combinations)
}
