use std::collections::HashMap;

use chrono::Duration;

use crate::{
    combination::CombinationConfig, data::Member, resolve_combinations, timeslot::WeeklyTimeslot,
    Combination, MemberId,
};

#[derive(Debug, Clone)]
pub struct SchedulingOptions<'a> {
    /// Gap between each timeslot start time
    pub gap: Duration,
    /// Duration of each timeslot
    pub duration: Duration,

    /// Minimum number of members required for each timeslot
    pub min_members: Option<usize>,

    /// Combination required for each timeslot
    pub combination: &'a CombinationConfig,

    /// List of members
    pub members: &'a Vec<Member>,
}

#[derive(Clone)]
pub struct AssignedTimeSlot {
    pub timeslot: WeeklyTimeslot,
    pub members: Vec<MemberId>,
    pub combinations: Vec<Combination>,
    pub score: f32,
}

pub fn scheduling(
    options: SchedulingOptions,
) -> (HashMap<String, Vec<WeeklyTimeslot>>, Vec<AssignedTimeSlot>) {
    let slots = WeeklyTimeslot::gen_weekly_timeslots(options.gap, options.duration);
    let min_members = options.min_members.unwrap_or(options.members.len());

    let member_availability = options
        .members
        .iter()
        .map(|member| {
            let timeslots = WeeklyTimeslot::from_member(member);
            (member.id.clone(), timeslots)
        })
        .collect::<HashMap<_, _>>();

    // Assigning timeslots
    let mut assigned = Vec::new();
    for slot in slots {
        // Filter members based on availability
        let available_members = options
            .members
            .clone()
            .into_iter()
            .filter(|member| {
                member_availability
                    .get(&member.id)
                    .unwrap()
                    .iter()
                    .any(|availability| availability.is_contain(&slot))
            })
            .collect::<Vec<_>>();

        if available_members.len() < min_members {
            continue;
        }

        let (score, combinations) = resolve_combinations(
            &available_members,
            options.combination,
            1.0,
            0.8,
            options.min_members,
        );
        if combinations.is_empty() {
            continue;
        }

        assigned.push(AssignedTimeSlot {
            timeslot: slot,
            members: available_members
                .iter()
                .map(|member| member.id.clone())
                .collect(),
            combinations,
            score,
        });
    }

    (member_availability, assigned)
}
