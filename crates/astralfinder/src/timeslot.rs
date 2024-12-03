use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Weekday};
use chrono_tz::Tz;

use crate::{Availability, Member};

#[derive(Debug, Clone, Copy)]
pub struct WeeklyTimeslot {
    pub weekday: Weekday,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
}

impl WeeklyTimeslot {
    /// Use a static date to get the weekday
    fn get_naive_date(weekday: Weekday) -> NaiveDate {
        NaiveDate::from_ymd_opt(2024, 12, weekday.num_days_from_monday() + 2).unwrap()
    }

    pub fn from_weekday_time(weekday: Weekday, time: NaiveTime, duration: Duration) -> Self {
        let start = Self::get_naive_date(weekday).and_time(time);
        let end = start + duration;

        WeeklyTimeslot {
            weekday,
            start,
            end,
        }
    }

    pub fn gen_weekly_timeslots(gap: Duration, duration: Duration) -> Vec<WeeklyTimeslot> {
        let mut slots = vec![];

        let start_date = Self::get_naive_date(Weekday::Mon);
        let mut current_time = start_date.and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap());
        loop {
            slots.push(WeeklyTimeslot {
                weekday: current_time.weekday(),
                start: current_time,
                end: current_time + duration,
            });

            current_time = current_time + gap;
            // Check if current date is the next week
            if current_time.date() - start_date > Duration::days(6) {
                break;
            }
        }

        slots
    }

    pub fn from_availability(timezone: Tz, availability: &Availability) -> WeeklyTimeslot {
        let date =
            NaiveDate::from_ymd_opt(2024, 12, availability.weekday.num_days_from_monday() + 2)
                .unwrap();
        let naive_local_dt = date.and_time(availability.time_range.start);
        let local_dt = timezone.from_local_datetime(&naive_local_dt).unwrap();

        let duration = availability.time_range.duration();

        let utc_dt = local_dt.naive_utc();
        let utc_weekday = utc_dt.weekday();

        WeeklyTimeslot {
            weekday: utc_weekday,
            start: utc_dt,
            end: utc_dt + duration,
        }
    }

    pub fn from_member(member: &Member) -> Vec<WeeklyTimeslot> {
        member
            .availability
            .iter()
            .map(|availability| Self::from_availability(member.timezone, availability))
            .collect()
    }

    /// Check if the timeslot is available in the availability with the timezone
    pub fn is_contain(&self, timeslot: &WeeklyTimeslot) -> bool {
        self.start <= timeslot.start && self.end >= timeslot.end
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, vec};

    use chrono::Duration;
    use chrono_tz::{America, Asia};

    use crate::{Member, TimeRange};

    use super::WeeklyTimeslot;

    #[test]
    fn test_demo() {
        let liri = Member {
            id: "liri".to_string(),
            jobs: vec![],
            timezone: Asia::Ho_Chi_Minh,
            availability: vec![
                crate::Availability {
                    weekday: chrono::Weekday::Mon,
                    time_range: TimeRange {
                        start: chrono::NaiveTime::from_hms_opt(7, 0, 0).unwrap(),
                        end: chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
                    },
                },
                crate::Availability {
                    weekday: chrono::Weekday::Tue,
                    time_range: TimeRange {
                        start: chrono::NaiveTime::from_hms_opt(7, 0, 0).unwrap(),
                        end: chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
                    },
                },
                crate::Availability {
                    weekday: chrono::Weekday::Wed,
                    time_range: TimeRange {
                        start: chrono::NaiveTime::from_hms_opt(7, 0, 0).unwrap(),
                        end: chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
                    },
                },
            ],
        };

        let elizabeth = Member {
            id: "elizabeth".to_string(),
            jobs: vec![],
            timezone: America::New_York,
            availability: vec![
                crate::Availability {
                    weekday: chrono::Weekday::Mon,
                    time_range: TimeRange {
                        start: chrono::NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
                        end: chrono::NaiveTime::from_hms_opt(17, 30, 0).unwrap(),
                    },
                },
                crate::Availability {
                    weekday: chrono::Weekday::Mon,
                    time_range: TimeRange {
                        start: chrono::NaiveTime::from_hms_opt(18, 0, 0).unwrap(),
                        end: chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
                    },
                },
                crate::Availability {
                    weekday: chrono::Weekday::Tue,
                    time_range: TimeRange {
                        start: chrono::NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
                        end: chrono::NaiveTime::from_hms_opt(17, 30, 0).unwrap(),
                    },
                },
                crate::Availability {
                    weekday: chrono::Weekday::Tue,
                    time_range: TimeRange {
                        start: chrono::NaiveTime::from_hms_opt(18, 0, 0).unwrap(),
                        end: chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
                    },
                },
                crate::Availability {
                    weekday: chrono::Weekday::Wed,
                    time_range: TimeRange {
                        start: chrono::NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
                        end: chrono::NaiveTime::from_hms_opt(17, 30, 0).unwrap(),
                    },
                },
                crate::Availability {
                    weekday: chrono::Weekday::Wed,
                    time_range: TimeRange {
                        start: chrono::NaiveTime::from_hms_opt(18, 0, 0).unwrap(),
                        end: chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
                    },
                },
            ],
        };

        let astrid = Member {
            id: "astrid".to_string(),
            jobs: vec![],
            timezone: America::Los_Angeles,
            availability: vec![
                crate::Availability {
                    weekday: chrono::Weekday::Mon,
                    time_range: TimeRange {
                        start: chrono::NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
                        end: chrono::NaiveTime::from_hms_opt(17, 0, 0).unwrap(),
                    },
                },
                crate::Availability {
                    weekday: chrono::Weekday::Tue,
                    time_range: TimeRange {
                        start: chrono::NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
                        end: chrono::NaiveTime::from_hms_opt(17, 0, 0).unwrap(),
                    },
                },
                crate::Availability {
                    weekday: chrono::Weekday::Wed,
                    time_range: TimeRange {
                        start: chrono::NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
                        end: chrono::NaiveTime::from_hms_opt(17, 0, 0).unwrap(),
                    },
                },
            ],
        };

        let members = vec![liri, elizabeth, astrid];
        let member_slots = members
            .iter()
            .map(|member| (member.id.clone(), WeeklyTimeslot::from_member(member)))
            .collect::<HashMap<_, _>>();

        let slots =
            WeeklyTimeslot::gen_weekly_timeslots(Duration::minutes(30), Duration::minutes(120));
        let filter = slots
            .into_iter()
            .filter_map(|slot| {
                let filtered_members = members
                    .iter()
                    .filter(|member| {
                        member_slots
                            .get(&member.id)
                            .unwrap()
                            .iter()
                            .any(|member_slot| member_slot.is_contain(&slot))
                    })
                    .collect::<Vec<_>>();

                if filtered_members.len() >= 2 {
                    Some((
                        slot,
                        filtered_members
                            .into_iter()
                            .map(|member| member.id.clone())
                            .collect::<Vec<_>>()
                            .join(", "),
                    ))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        println!("{:#?}", filter);
    }
}
