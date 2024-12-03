use std::{fmt::Debug, str::FromStr};

use chrono::{Duration, NaiveTime, Timelike, Weekday};
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

    /// Get the duration of the time range
    pub fn duration(&self) -> Duration {
        let (start, end) = self.seconds_from_midnight();
        Duration::seconds((end - start) as i64)
    }

    /// Check if another time range is contained within this time range
    pub fn is_contain(&self, other: &TimeRange) -> bool {
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
    pub weekday: Weekday,
    pub time_range: TimeRange,
}

#[derive(Debug, Clone)]
pub struct ReferJob {
    pub job: Job,
    pub confidence: f32,
}

pub type MemberId = String;

#[derive(Debug, Clone)]
pub struct Member {
    pub id: MemberId,
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

    // /// Check if the member is available at the given time slot
    // pub fn is_available(&self, time_slot: &TimeSlot) -> bool {
    //     let start = self
    //         .timezone
    //         .from_utc_datetime(&time_slot.start.naive_utc());
    //     let end = self.timezone.from_utc_datetime(&time_slot.end.naive_utc());

    //     let time_slot_day = start.weekday();
    //     // Apply the timezone to the time slot
    //     let time_slot_time_range = TimeRange {
    //         start: start.time(),
    //         end: end.time(),
    //     };

    //     self.availability.iter().any(|availability| {
    //         availability.day_of_week == time_slot_day
    //             && availability
    //                 .time_ranges
    //                 .iter()
    //                 .any(|time_range| time_range.contains(&time_slot_time_range))
    //     })
    // }
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
