use chrono::{DateTime, Duration, NaiveDate, Utc};
use serde::Serialize;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt;


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


#[derive(Serialize, Debug)]
pub enum Status {
    FREE,
    BOOKED,
}


#[derive(Serialize, Debug, Copy, Clone, Hash, Eq)]
pub struct TimeRange {
    pub init: i64,
    pub end: i64,
}


#[derive(Serialize, Debug)]
pub struct Slot {
    pub id: u64,
    pub time: TimeRange,
    pub status: Status,
}


/**
 *  Slot implementations
 */


 impl Slot {

    pub fn to_hour_map(slots: Vec<Slot>) -> BTreeMap<TimeRange, Vec<Slot>> {
        let mut result: BTreeMap<TimeRange, Vec<Slot>> = BTreeMap::new();

        for slot in slots.into_iter() {
            let entry = result.entry(slot.time).or_insert(Vec::new());
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
        let mut id: u64 = 0;

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
                            status: Status::FREE,
                            time: TimeRange {
                                init: new_init_slot.timestamp(),
                                end: new_end_slot.timestamp(),
                            },
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

