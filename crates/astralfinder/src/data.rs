use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use chrono::{DateTime, Datelike, Duration, NaiveTime, TimeZone, Timelike, Utc, Weekday};
use chrono_tz::Tz;

#[derive(Debug, Clone)]
pub struct TimeRange {
    pub start: NaiveTime,
    pub end: NaiveTime,
}

impl TimeRange {
    fn seconds_from_midnight(&self) -> (u32, u32) {
        let start = self.start.num_seconds_from_midnight();
        let end = if self.end < self.start {
            self.end.num_seconds_from_midnight() + (Duration::days(1).num_seconds() as u32)
        } else {
            self.end.num_seconds_from_midnight()
        };

        (start, end)
    }

    /// Check if another time range is contained within this time range
    pub fn contains(&self, other: &TimeRange) -> bool {
        let (tr1_start, tr1_end) = self.seconds_from_midnight();
        let (tr2_start, tr2_end) = other.seconds_from_midnight();

        tr1_start <= tr2_start && tr1_end >= tr2_end
    }

    /// Check if the time range overlaps with another time range
    pub fn is_overlap(&self, other: &TimeRange) -> bool {
        let (tr1_start, tr1_end) = self.seconds_from_midnight();
        let (tr2_start, tr2_end) = other.seconds_from_midnight();

        // Check for overlap
        tr1_start < tr2_end && tr1_end > tr2_start
    }
}

#[derive(Debug, Clone)]
pub struct Availability {
    pub day_of_week: Weekday,
    pub time_ranges: Vec<TimeRange>,
}

#[derive(Debug, Clone)]
pub struct ReferJob {
    pub job: Job,
    pub confidence: f32,
}

#[derive(Debug, Clone)]
pub struct Member {
    pub id: String,
    // pub name: String,
    pub timezone: Tz,
    pub availability: Vec<Availability>,
    pub jobs: Vec<ReferJob>,
}

impl PartialEq for Member {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Member {}

impl Member {
    // /// Check if the member can fill the given role
    // /// Return list of jobs that the member can fill as the given role
    // pub fn fillable(&self, role: &Role) -> Vec<Job> {
    //     self.jobs
    //         .iter()
    //         .filter(|job| job.role() == *role)
    //         .cloned()
    //         .collect()
    // }

    /// Check if the member is available at the given time slot
    pub fn is_available(&self, time_slot: &TimeSlot) -> bool {
        let start = self
            .timezone
            .from_utc_datetime(&time_slot.start.naive_utc());
        let end = self.timezone.from_utc_datetime(&time_slot.end.naive_utc());

        let time_slot_day = start.weekday();
        // Apply the timezone to the time slot
        let time_slot_time_range = TimeRange {
            start: start.time(),
            end: end.time(),
        };

        self.availability.iter().any(|availability| {
            availability.day_of_week == time_slot_day
                && availability
                    .time_ranges
                    .iter()
                    .any(|time_range| time_range.contains(&time_slot_time_range))
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Job {
    // Tanks
    Pld,
    War,
    Gnb,
    Drk,

    // Healer
    Whm,
    Sch,
    Ast,
    Sge,

    // Melees
    Drg,
    Mnk,
    Nin,
    Sam,
    Rpr,
    Vpr,

    // Ranged
    Brd,
    Mch,
    Dnc,

    // Caster
    Blm,
    Smn,
    Rdm,
    Pct,
}

impl ToString for Job {
    fn to_string(&self) -> String {
        match self {
            Job::Pld => "pld".to_string(),
            Job::War => "war".to_string(),
            Job::Gnb => "gnb".to_string(),
            Job::Drk => "drk".to_string(),
            Job::Whm => "whm".to_string(),
            Job::Sch => "sch".to_string(),
            Job::Ast => "ast".to_string(),
            Job::Sge => "sge".to_string(),
            Job::Drg => "drg".to_string(),
            Job::Mnk => "mnk".to_string(),
            Job::Nin => "nin".to_string(),
            Job::Sam => "sam".to_string(),
            Job::Rpr => "rpr".to_string(),
            Job::Vpr => "vpr".to_string(),
            Job::Brd => "brd".to_string(),
            Job::Mch => "mch".to_string(),
            Job::Dnc => "dnc".to_string(),
            Job::Blm => "blm".to_string(),
            Job::Smn => "smn".to_string(),
            Job::Rdm => "rdm".to_string(),
            Job::Pct => "pct".to_string(),
        }
    }
}

impl FromStr for Job {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        match &s.as_str() {
            &"pld" | &"paladin" => Ok(Job::Pld),
            &"war" | &"warrior" => Ok(Job::War),
            &"gnb" | &"gunbreaker" => Ok(Job::Gnb),
            &"drk" | &"darkknight" => Ok(Job::Drk),
            &"whm" | &"white mage" => Ok(Job::Whm),
            &"sch" | &"scholar" => Ok(Job::Sch),
            &"ast" | &"astrologian" => Ok(Job::Ast),
            &"sge" | &"sage" => Ok(Job::Sge),
            &"drg" | &"dragoon" => Ok(Job::Drg),
            &"mnk" | &"monk" => Ok(Job::Mnk),
            &"nin" | &"ninja" => Ok(Job::Nin),
            &"sam" | &"samurai" => Ok(Job::Sam),
            &"rpr" | &"reaper" => Ok(Job::Rpr),
            &"vpr" | &"viper" => Ok(Job::Vpr),
            &"brd" | &"bard" => Ok(Job::Brd),
            &"mch" | &"machinist" => Ok(Job::Mch),
            &"dnc" | &"dancer" => Ok(Job::Dnc),
            &"blm" | &"black mage" => Ok(Job::Blm),
            &"smn" | &"summoner" => Ok(Job::Smn),
            &"rdm" | &"red mage" => Ok(Job::Rdm),
            &"pct" | &"pictomancer" => Ok(Job::Pct),
            _ => Err(format!("Invalid job: {}", s)),
        }
    }
}

impl Job {
    pub fn satifies(&self, role: &Role) -> bool {
        match role {
            Role::Tank => match self {
                Job::Pld | Job::War | Job::Gnb | Job::Drk => true,
                _ => false,
            },
            Role::PureHealer => match self {
                Job::Whm | Job::Ast => true,
                _ => false,
            },
            Role::ShieldHealer => match self {
                Job::Sch | Job::Sge => true,
                _ => false,
            },
            Role::Healer => match self {
                Job::Whm | Job::Ast | Job::Sch | Job::Sge => true,
                _ => false,
            },
            Role::Melee => match self {
                Job::Drg | Job::Mnk | Job::Nin | Job::Sam | Job::Rpr | Job::Vpr => true,
                _ => false,
            },
            Role::Ranged => match self {
                Job::Brd | Job::Mch | Job::Dnc => true,
                _ => false,
            },
            Role::Caster => match self {
                Job::Blm | Job::Smn | Job::Rdm | Job::Pct => true,
                _ => false,
            },
            Role::DPS => match self {
                Job::Drg
                | Job::Mnk
                | Job::Nin
                | Job::Sam
                | Job::Rpr
                | Job::Vpr
                | Job::Brd
                | Job::Mch
                | Job::Dnc
                | Job::Blm
                | Job::Smn
                | Job::Rdm
                | Job::Pct => true,
                _ => false,
            },
        }
    }

    pub fn role(&self) -> Role {
        match self {
            Job::Pld | Job::War | Job::Gnb | Job::Drk => Role::Tank,
            Job::Whm | Job::Ast => Role::PureHealer,
            Job::Sch | Job::Sge => Role::ShieldHealer,
            Job::Drg | Job::Mnk | Job::Nin | Job::Sam | Job::Rpr | Job::Vpr => Role::Melee,
            Job::Brd | Job::Mch | Job::Dnc => Role::Ranged,
            Job::Blm | Job::Smn | Job::Rdm | Job::Pct => Role::Caster,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Role {
    Tank,
    ShieldHealer,
    PureHealer,
    Melee,
    Ranged,
    Caster,

    Healer,
    DPS,
}

impl FromStr for Role {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        match &s.as_str() {
            &"tank" => Ok(Role::Tank),
            &"shield_healer" => Ok(Role::ShieldHealer),
            &"pure_healer" => Ok(Role::PureHealer),
            &"melee" => Ok(Role::Melee),
            &"ranged" => Ok(Role::Ranged),
            &"caster" => Ok(Role::Caster),
            &"healer" => Ok(Role::Healer),
            &"dps" => Ok(Role::DPS),
            _ => Err(format!("Invalid role: {}", s)),
        }
    }
}

#[derive(Clone)]
pub struct TimeSlot {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

impl Display for TimeSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} - {}",
            self.start.format("%a %Y-%b-%d %H:%M"),
            self.end.format("%a %Y-%b-%d %H:%M")
        )
    }
}

impl Debug for TimeSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl TimeSlot {
    pub fn gen_time_slots(
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        gap: Duration,
        duration: Duration,
    ) -> Vec<TimeSlot> {
        // Normalize the start and end to :00 or :30
        let start = start
            .with_minute((start.minute() / 30) * 30)
            .unwrap()
            .with_second(0)
            .unwrap();
        let end = end
            .with_minute((end.minute() / 30) * 30)
            .unwrap()
            .with_second(0)
            .unwrap();

        let mut time_slots = Vec::new();
        let mut current = start;

        while current < end {
            time_slots.push(TimeSlot {
                start: current,
                end: current + duration,
            });
            current = current + gap;
        }
        time_slots
    }

    pub fn is_overlap(&self, other: &TimeSlot) -> bool {
        self.start < other.end && self.end > other.start
    }
}

#[derive(Debug)]
pub struct Event {
    pub time_slot: TimeSlot,
    pub assignments: Vec<MemberAssignment>,
}

#[derive(Debug)]
pub struct MemberAssignment {
    pub member: u32,
    pub job: Vec<Job>,
    pub role: Role,
}

#[cfg(test)]
mod tests {
    use super::{TimeRange, TimeSlot};
    use chrono::{Duration, TimeZone, Utc};

    #[test]
    fn test_time_range_contains() {
        let a = TimeRange {
            start: chrono::NaiveTime::from_hms_opt(17, 0, 0).unwrap(),
            end: chrono::NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        };
        let b = TimeRange {
            start: chrono::NaiveTime::from_hms_opt(17, 0, 0).unwrap(),
            end: chrono::NaiveTime::from_hms_opt(19, 0, 0).unwrap(),
        };

        assert_eq!(a.contains(&b), true);
    }

    #[test]
    fn test_gen_time_slots() {
        let start = Utc.with_ymd_and_hms(2024, 11, 24, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2024, 11, 30, 0, 0, 0).unwrap();
        let duration = Duration::minutes(120);
        let gap = Duration::minutes(30);

        let time_slots = TimeSlot::gen_time_slots(start, end, gap, duration);
        println!("{:#?}", time_slots);
    }
}
