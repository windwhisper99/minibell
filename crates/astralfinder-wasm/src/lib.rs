mod utils;

use std::{collections::HashMap, convert::TryInto};

use astralfinder::{
    combination,
    data::{Member, ReferJob, Role},
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[derive(Debug, Deserialize)]
struct MemberInput {
    id: String,
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

        Ok(Member {
            id,
            jobs,
            timezone: chrono_tz::Tz::UTC,
            availability: Vec::new(),
        })
    }
}

#[derive(Debug, Deserialize)]
struct Input {
    roles: HashMap<String, usize>,
    members: Vec<MemberInput>,
}

#[derive(Debug, Serialize)]
struct Assigned {
    id: String,
    job: String,
}

#[derive(Debug, Serialize)]
struct CombinationResult {
    assigned: Vec<Assigned>,
    score: f32,
}

#[derive(Debug, Serialize)]
struct Output {
    combinations: Vec<CombinationResult>,
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

    let combinations = combination::resolve_combinations(&members, roles);
    let result = combinations.into_iter().map(|combination| {
        let mut assigned = Vec::new();
        for (member_id, job) in combination.assigned {
            assigned.push(Assigned {
                id: member_id,
                job: job.to_string(),
            });
        }

        CombinationResult {
            assigned,
            score: combination.score,
        }
    });

    serde_wasm_bindgen::to_value(&Output {
        combinations: result.collect(),
    })
    .expect("Failed to serialize output")
}
