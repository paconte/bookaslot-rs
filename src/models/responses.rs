use chrono::NaiveDate;
use serde::Serialize;
use std::collections::BTreeMap;

use crate::models::models::{Slot, TimeRange};


#[derive(Serialize)]
pub struct TimeItems {
    pub time: TimeRange,
    pub items: Vec<Slot>,
}


impl TimeItems {

    pub fn to_hour_response(slots_by_hour: BTreeMap<TimeRange, Vec<Slot>>) -> Vec<TimeItems> {
        let mut result = Vec::new();
        for (time, items) in slots_by_hour {
            result.push(TimeItems{time, items});
        }
        result
    }


    pub fn to_day_response(slots_by_day: BTreeMap<NaiveDate, BTreeMap<TimeRange, Vec<Slot>>>) -> BTreeMap<String, Vec<TimeItems>> {
        let mut result: BTreeMap<String, Vec<TimeItems>> = BTreeMap::new();
        for (time, items) in slots_by_day {
            let mut response_by_hour: Vec<TimeItems> = TimeItems::to_hour_response(items);
            let entry = result.entry(time.format("%Y-%m-%d").to_string()).or_insert(Vec::new());
            entry.append(&mut response_by_hour);
        }
        result
    }
}