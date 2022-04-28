use chrono::{DateTime, Duration, NaiveDate, Utc};
use diesel::backend::Backend;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
//use diesel::{Insertable, Queryable};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashSet};
use std::fmt;
use std::hash::Hash;
use std::io::Write;
use std::str::FromStr;

use crate::models::schema::bookable;
use crate::models::schema::slots;


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


#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub enum State {
    FREE,
    BOOKED,
    TOBEBOOKED,
}


#[derive(Serialize, Debug, Clone, Copy, Hash, Eq)]
pub struct TimeRange {
    pub init: i64,
    pub end: i64,
}


#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct Slot {
    pub id: i32,
    pub state: State,
    pub start: i64,
    pub finish: i64,
    pub bookable: Bookable,
}


#[derive(Insertable, Queryable, Debug)]
#[table_name="slots"]
pub struct DbSlot {
    pub id: i32,
    pub state: String,
    pub start: i64,
    pub finish: i64,
    pub bookable_id: i32,
}


#[derive(Clone, Debug, Eq, Hash, Insertable, Queryable, Deserialize, Serialize)]
#[table_name="bookable"]
pub struct Bookable {
    pub id: i32,
    pub name: String,
}


/**
 *  DbSlot implementations
 */

impl DbSlot {

    pub fn into_db_slots(items: Vec<Slot>) -> Vec<DbSlot> {
        items
            .into_iter()
            .map(|item| DbSlot::from(item))
            .collect()
    }

    pub fn into_slots(items: Vec<(Bookable, DbSlot)>) -> Vec<Slot> {
        items
            .into_iter()
            .map(|item| item.into())
            .collect()
    }
}


impl From<Slot> for DbSlot {
    fn from(item: Slot) -> Self {
        /*
        let slot_state: State = match item.state {
            State::TOBEBOOKED => State::BOOKED,
            other => other,
        };
        */
        DbSlot {
            id: item.id,
            state: item.state.to_string(),
            start: item.start,
            finish: item.finish,
            bookable_id: item.bookable.id,
        }
    }
}


/**
 *  Slot implementations
 */


 impl Slot {

    pub fn time_eq(&self, other: &Slot) -> bool {
        self.bookable.id == other.bookable.id &&
        self.start == other.start &&
        self.finish == other.finish
    }

    pub fn to_hour_map(slots: Vec<Slot>) -> BTreeMap<TimeRange, Vec<Slot>> {
        let mut result: BTreeMap<TimeRange, Vec<Slot>> = BTreeMap::new();

        for slot in slots.into_iter() {
            let time_range = TimeRange::from_slot(slot.clone());
            let entry = result.entry(time_range).or_insert(Vec::new());
            entry.push(slot.clone());
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

impl From<(Bookable, DbSlot)> for Slot {
    fn from(item: (Bookable, DbSlot)) -> Self {
        Slot {
            id: item.1.id,
            state: State::from_str(&item.1.state).unwrap(),
            start: item.1.start,
            finish: item.1.finish,
            bookable: Bookable {id: item.0.id, name: item.0.name},
        }
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

    pub fn generate_slots(
        template: &Template, bookables: &Vec<Bookable>, slots: HashSet<Slot>
    ) -> Vec<Slot> {

        if bookables.len() < 1 {
            panic!("Argument size ({}), must be bigger than 0.", bookables.len());
        }

        let mut result: Vec<Slot> = Vec::new();
        let mut init_day = template.init_day;
        let mut init_time;
        let mut id: i32 = 0;
        //let mut rng = rand::thread_rng();

        while init_day <= template.end_day {
            init_time = template.init_time;
            while init_time < template.end_time {
                let new_end = init_time + template.duration;
                if new_end <= template.end_time {
                    let new_init_slot = init_day.and_time(init_time).unwrap();
                    let new_end_slot = init_day.and_time(new_end).unwrap();
                    for bookable in bookables.iter() {
                        let mut slot = Slot {
                            id: id,
                            //state: State::new(2, rng.gen_range(0..3)),
                            state: State::FREE,
                            start: new_init_slot.timestamp(),
                            finish: new_end_slot.timestamp(),
                            bookable: bookable.clone(),
                        };
                        if None != slots.iter().find(|x| x.time_eq(&slot)) {
                            slot.state = State::BOOKED;
                        }

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
            "FREE" => Ok(State::FREE),
            "BOOKED" => Ok(State::BOOKED),
            "TOBEBOOKED" => Ok(State::BOOKED),
            //"TOBEBOOKED" => Ok(State::TOBEBOOKED),
            _  => Err(()),
        }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            State::FREE => write!(f, "FREE"),
            State::BOOKED => write!(f, "BOOKED"),
            State::TOBEBOOKED => write!(f, "BOOKED"),
            //State::TOBEBOOKED => write!(f, "TOBEBOOKED"),
        }
    }
}


/**
 * Bookable implementations
 */
impl PartialEq for Bookable {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}


/**
 * Language implementations
 */


impl<Db: Backend> ToSql<State, Db> for State {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Db>) -> serialize::Result {
        match *self {
            State::FREE => out.write_all(b"FREE")?,
            State::BOOKED => out.write_all(b"BOOKED")?,
            State::TOBEBOOKED => out.write_all(b"BOOKED")?,
        }
        Ok(IsNull::No)
    }
}


impl FromSql<State, Pg> for State {
    fn from_sql(bytes: Option<&<Pg as Backend>::RawValue>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"FREE" => Ok(State::FREE),
            b"BOOKED" => Ok(State::BOOKED),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}