use std::{collections::HashMap, fs};

use chrono::{NaiveTime, Weekday};
use serde::Deserialize;

use crate::data::{Availability, Member, ReferJob, TimeRange};

#[derive(Debug, Deserialize)]
struct MemberFormat {
    id: String,
    timezone: String,
    time: HashMap<String, String>,
    jobs: HashMap<String, f32>,
}

impl TryInto<Member> for MemberFormat {
    type Error = String;

    fn try_into(self) -> Result<Member, Self::Error> {
        let timezone: chrono_tz::Tz = self
            .timezone
            .parse()
            .map_err(|_| format!("Invalid timezone: {}", self.timezone))?;

        let mut availability = Vec::new();
        for (day, time) in self.time {
            let day_of_week = match day.as_str().to_lowercase().as_str() {
                "sun" => Weekday::Sun,
                "mon" => Weekday::Mon,
                "tue" => Weekday::Tue,
                "wed" => Weekday::Wed,
                "thu" => Weekday::Thu,
                "fri" => Weekday::Fri,
                "sat" => Weekday::Sat,
                _ => return Err(format!("Invalid day of week: {}", day)),
            };

            let time_ranges = time
                .split(',')
                .map(|range| {
                    let mut split = range.split('-');
                    let start = split.next().expect("Invalid time range");
                    let end = split.next().expect("Invalid time range");

                    TimeRange {
                        start: NaiveTime::parse_from_str(start, "%H:%M")
                            .expect("Invalid time format"),
                        end: NaiveTime::parse_from_str(end, "%H:%M").expect("Invalid time format"),
                    }
                })
                .collect();

            availability.push(Availability {
                day_of_week,
                time_ranges,
            });
        }

        let jobs = self
            .jobs
            .iter()
            .map(|(job, confidence)| {
                let job = job.parse().expect("Invalid job");
                ReferJob {
                    job,
                    confidence: *confidence,
                }
            })
            .collect();

        Ok(Member {
            id: self.id,
            timezone,
            availability,
            jobs,
        })
    }
}

pub fn load_members(file: &str) -> Vec<Member> {
    let json = fs::read(file).expect("Unable to read file");
    let members: Vec<MemberFormat> = serde_json::from_slice(&json).expect("Unable to parse JSON");

    members
        .into_iter()
        .map(|member| member.try_into().expect("Invalid member format"))
        .collect()
}
