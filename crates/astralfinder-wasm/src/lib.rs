mod utils;

use std::{collections::HashMap, convert::TryInto};

use astralfinder::{
    scheduling, timeslot::WeeklyTimeslot, Availability, Combination, CombinationConfig, Member,
    MemberId, ReferJob, Role, SchedulingOptions, TimeRange,
};
use chrono::Duration;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(value: JsValue);
}

#[derive(Debug, Deserialize)]
struct MemberInput {
    id: String,
    timezone: String,
    availability: HashMap<String, String>,
    jobs: HashMap<String, f32>,
}

impl TryInto<Member> for MemberInput {
    type Error = String;

    fn try_into(self) -> Result<Member, Self::Error> {
        let id = self.id;
        let mut jobs = Vec::new();

        for (job, confidence) in self.jobs {
            let job = job.parse().map_err(|_| format!("Invalid job: {}", job))?;
            jobs.push(ReferJob { job, confidence });
        }

        let mut availability = Vec::new();
        for (weekday, time) in self.availability {
            let weekday = weekday.parse().map_err(|_| "Invalid weekday")?;
            let time_ranges = time
                .split(",")
                .map(|time| -> Result<TimeRange, String> {
                    let time: Vec<&str> = time.trim().split("-").collect();
                    let start = time
                        .get(0)
                        .ok_or("Invalid time")?
                        .parse()
                        .map_err(|_| "Invalid time")?;
                    let end = time
                        .get(1)
                        .ok_or("Invalid time")?
                        .parse()
                        .map_err(|_| "Invalid time")?;
                    Ok(TimeRange { start, end })
                })
                .collect::<Result<Vec<_>, _>>()
                .expect("Expected valid time ranges");

            for time_range in time_ranges {
                availability.push(Availability {
                    weekday,
                    time_range,
                });
            }
        }

        Ok(Member {
            id,
            jobs,
            timezone: self.timezone.parse().map_err(|_| "Invalid timezone")?,
            availability,
        })
    }
}

#[derive(Debug, Deserialize)]
struct Input {
    roles: HashMap<String, usize>,
    members: Vec<MemberInput>,

    min_members: Option<usize>,
    /// Duration in minutes
    gap: Option<u32>,
    /// Duration in minutes
    duration: Option<u32>,
}

#[derive(Debug, Serialize)]
struct Assigned {
    id: String,
    job: String,
}

#[derive(Debug, Serialize)]
struct AssignedTimeSlot {
    weekday: String,
    start: String,
    end: String,
    /// Duration in minutes
    duration: u32,
    members: Vec<MemberId>,

    score: f32,
    combinations: Vec<CombinationResult>,
}

#[derive(Debug, Serialize)]
struct CombinationResult {
    assigned: Vec<Assigned>,
    score: f32,
}

impl From<Combination> for CombinationResult {
    fn from(value: Combination) -> Self {
        let mut assigned = Vec::new();
        for (member_id, job) in value.assigned {
            assigned.push(Assigned {
                id: member_id,
                job: job.to_string(),
            });
        }

        CombinationResult {
            assigned,
            score: value.score,
        }
    }
}

#[derive(Debug, Serialize)]
struct Output {
    combinations: Vec<CombinationResult>,
    timeslots: Vec<AssignedTimeSlot>,
}

#[wasm_bindgen(js_name = resolve)]
pub fn resolve_js(input: JsValue) -> JsValue {
    utils::set_panic_hook();

    let input: Input = serde_wasm_bindgen::from_value(input).expect("Failed to deserialize input");
    let members: Vec<Member> = input
        .members
        .into_iter()
        .map(|member| member.try_into())
        .collect::<Result<Vec<_>, _>>()
        .expect("Failed to convert members");

    let mut roles: HashMap<Role, usize> = HashMap::new();
    for (role, count) in input.roles {
        let role = role.parse().expect("Invalid role");
        roles.insert(role, count);
    }

    let combination = CombinationConfig::new(roles);
    let (demo, timeslots) = scheduling(SchedulingOptions {
        gap: Duration::minutes(input.gap.unwrap_or(30).into()),
        duration: Duration::minutes(input.duration.unwrap_or(120).into()),
        min_members: input.min_members,
        combination: &combination,
        members: &members,
    });

    #[derive(Debug, Serialize)]
    struct Demo {
        weekday: String,
        start: String,
        end: String,
    }

    impl From<WeeklyTimeslot> for Demo {
        fn from(value: WeeklyTimeslot) -> Self {
            Demo {
                weekday: value.weekday.to_string(),
                start: value.start.to_string(),
                end: value.end.to_string(),
            }
        }
    }
    let demos = demo
        .into_iter()
        .map(|(member, slot)| (member, slot.into_iter().map(Into::into).collect::<Vec<_>>()))
        .collect::<HashMap<String, Vec<Demo>>>();
    log(serde_wasm_bindgen::to_value(&demos).expect("Failed to serialize demo"));

    serde_wasm_bindgen::to_value(&Output {
        combinations: vec![],
        timeslots: timeslots
            .into_iter()
            .map(|timeslot| AssignedTimeSlot {
                weekday: timeslot.timeslot.weekday.to_string(),
                start: timeslot.timeslot.start.to_string(),
                end: timeslot.timeslot.end.to_string(),
                // duration: timeslot.timeslot.duration.num_minutes() as u32,
                duration: 120,
                members: timeslot.members,

                score: timeslot.score,
                combinations: timeslot.combinations.into_iter().map(Into::into).collect(),
            })
            .collect(),
    })
    .expect("Failed to serialize output")
}
