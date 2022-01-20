use chrono::{DateTime, Utc};
use serde::Serialize;
use std::fmt;

#[derive(Serialize)]
pub enum SlotStatus {
    FREE,
    BOOKED,
}

#[derive(Serialize, Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct SlotTime {
    init: i64,
    end: i64,
}

#[derive(Serialize, Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct SlotTableTime {
    id: i8,
    init: i64,
    end: i64,
}

#[derive(Serialize)]
pub struct Slot {
    pub id: u64,
    pub time: SlotTime,
    pub status: SlotStatus,
}

#[derive(Serialize)]
pub enum SlotTable {
    Item(Slot),
    Time(SlotTableTime),
}

#[derive(Serialize)]
pub struct TimeItems {
    pub time: SlotTime,
    pub items: Vec<Slot>
}

impl fmt::Display for SlotTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.init, self.end)
    }
}

impl SlotTime {
    pub fn new(init: DateTime<Utc>, end: DateTime<Utc>) -> SlotTime {
        SlotTime {
            init: init.timestamp(),
            end: end.timestamp(),
        }
    }
}

impl SlotTableTime {
    pub fn new(time: SlotTime) -> SlotTableTime {
        SlotTableTime {
            id: -1,
            init: time.init,
            end: time.end,
        }
    }
}
