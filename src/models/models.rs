use chrono::{DateTime, Duration, NaiveDate, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt;
use std::str::FromStr;


/*
 * Structs
 */
#[derive(Debug)]
pub struct Template {
    pub init_day: chrono::Date<chrono::Utc>,
    pub end_day: chrono::Date<chrono::Utc>,
    pub init_time: chrono::NaiveTime,
    pub end_time: chrono::NaiveTime,
    pub duration: chrono::Duration,
}


#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum State {
    FREE,
    BOOKED,
}


#[derive(Serialize, Debug, Copy, Clone, Hash, Eq)]
pub struct TimeRange {
    pub init: i64,
    pub end: i64,
}


#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Slot {
    pub id: i32,
    pub state: State,
    pub start: i64,
    pub finish: i64
}


/**
 *  Slot implementations
 */


 impl Slot {

    pub fn to_hour_map(slots: Vec<Slot>) -> BTreeMap<TimeRange, Vec<Slot>> {
        let mut result: BTreeMap<TimeRange, Vec<Slot>> = BTreeMap::new();

        for slot in slots.into_iter() {
            let time_range = TimeRange::from_slot(slot);
            let entry = result.entry(time_range).or_insert(Vec::new());
            entry.push(slot);
        }
        result
    }

    pub fn to_day_map(slots: Vec<Slot>) -> BTreeMap<NaiveDate, BTreeMap<TimeRange, Vec<Slot>>> {
        let mut result: BTreeMap<NaiveDate, BTreeMap<TimeRange, Vec<Slot>>> = BTreeMap::new();
        let hourly_map: BTreeMap<TimeRange, Vec<Slot>> = Slot::to_hour_map(slots);
        for (time, items) in hourly_map {
            let datetime = chrono::NaiveDateTime::from_timestamp(time.init, 0);
            let entry = result.entry(datetime.date()).or_insert(BTreeMap::new());
            entry.insert(time, items);
        }
        result
    }
}


/*
 * TimeRange implementations
 */


impl fmt::Display for TimeRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.init, self.end)
    }
}

impl Ord for TimeRange {
    fn cmp(&self, other: &Self) -> Ordering {
        self.init.cmp(&other.init)
    }
}

impl PartialOrd for TimeRange {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for TimeRange {
    fn eq(&self, other: &Self) -> bool {
        self.init == other.init
    }
}

impl TimeRange {
    pub fn new(init: DateTime<Utc>, end: DateTime<Utc>) -> TimeRange {
        TimeRange {
            init: init.timestamp(),
            end: end.timestamp(),
        }
    }

    pub fn from_slot(slot: Slot) -> TimeRange {
        TimeRange { init: slot.start, end: slot.finish }
    }
}

/*
 * Template implementations
 */


impl Template {

    pub fn generate_slots(template: Template, size: u8) -> Vec<Slot> {
        if size < 1 {
            panic!("Argument size ({}), must be bigger than 0.", size);
        }

        let mut result: Vec<Slot> = Vec::new();
        let mut init_day = template.init_day;
        let mut init_time;
        let mut id: i32 = 0;
        let mut rng = rand::thread_rng();

        while init_day <= template.end_day {
            init_time = template.init_time;
            while init_time < template.end_time {
                let new_end = init_time + template.duration;
                if new_end <= template.end_time {
                    let new_init_slot = init_day.and_time(init_time).unwrap();
                    let new_end_slot = init_day.and_time(new_end).unwrap();
                    for _ in 0..size {
                        let slot = Slot {
                            id: id,
                            state: State::new(2, rng.gen_range(0..3)),
                            start: new_init_slot.timestamp(),
                            finish: new_end_slot.timestamp(),
                        };
                        result.push(slot);
                        id += 1;
                    }
                }
                init_time += template.duration;
            }
            init_day = init_day + Duration::days(1);
        }

        result
    }
}


/**
 * State implementations
 */


impl State {

    pub fn new(percentage: u8, value: u8) -> State {
        if value < percentage {
            State::FREE
        } else {
            State::BOOKED
        }
    }
}

impl FromStr for State {
    type Err = ();
    fn from_str(input: &str) -> Result<State, Self::Err> {
        match input {
            "FREE"  => Ok(State::FREE),
            "BOOKED"  => Ok(State::BOOKED),
            _      => Err(()),
        }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            State::FREE => write!(f, "FREE"),
            State::BOOKED => write!(f, "BOOKED"),
        }
    }
}