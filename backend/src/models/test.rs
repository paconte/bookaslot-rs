#[cfg(test)]
use super::*;
#[cfg(test)]
use chrono::{Duration, TimeZone, Utc};
#[cfg(test)]
use std::collections::{HashSet};


#[test]
fn test_slot_time_eq() {
    let start = Utc.ymd(2022, 1, 1).and_hms(9, 0, 0).timestamp();
    let finish = Utc.ymd(2022, 1, 1).and_hms(10, 0, 0).timestamp();
    let bookable_1 = models::Bookable {id:1, name: String::from("Padel 1")};

    let slot_1 = models::Slot {
        id: 1,
        start: start,
        finish: finish,
        state: models::State::FREE,
        bookable: bookable_1.clone(),
    };
    let mut slot_2 = slot_1.clone();

    // assert true == slot.time_eq(slot2)
    assert_eq!(true, slot_1.time_eq(&slot_2));
    slot_2.id = 2;
    assert_eq!(true, slot_1.time_eq(&slot_2));
    slot_2.state = models::State::BOOKED;
    assert_eq!(true, slot_1.time_eq(&slot_2));
    slot_2.bookable.name = String::from("Padel 2");
    assert_eq!(true, slot_1.time_eq(&slot_2));

    // assert false == slot.time_eq(slot2)
    slot_2.bookable.id = 2;
    assert_eq!(false, slot_1.time_eq(&slot_2));
    slot_2.bookable.id = slot_1.bookable.id;
    slot_2.start += 1;
    assert_eq!(false, slot_1.time_eq(&slot_2));
    slot_2.start = slot_1.start;
    slot_2.finish += 1;
    assert_eq!(false, slot_1.time_eq(&slot_2));
}

#[test]
fn test_generate_slots() {
    let mut template = models::Template {
        init_day: Utc::today(),
        end_day: Utc::today() + Duration::days(1),
        init_time: chrono::NaiveTime::from_hms(9, 0, 0),
        end_time: chrono::NaiveTime::from_hms(14, 0, 0),
        duration: Duration::minutes(30),
    };
    let bookables = vec![
        models::Bookable {id: 1, name: String::from("Pista 1")},
        models::Bookable {id: 2, name: String::from("Pista 2")}
    ];

    let mut slots = models::Template::generate_slots(&template, &bookables, HashSet::new());
    assert_eq!(slots.len(), 10*2*2);

    template.duration = Duration::minutes(60);
    slots = models::Template::generate_slots(&template, &bookables, HashSet::new());
    assert_eq!(slots.len(), 10*2);

    template.end_day = template.init_day;
    slots = models::Template::generate_slots(&template, &bookables, HashSet::new());
    assert_eq!(slots.len(), 10);

    let slot = models::Slot {
        id: 1,
        state: models::State::BOOKED,
        start: slots[0].start,
        finish: slots[0].finish,
        bookable: slots[0].bookable.clone()
    };
    assert_ne!(None, slots.iter().find(|x| x.time_eq(&slot)));
    slots = models::Template::generate_slots(&template, &bookables, HashSet::from([slot]));
    assert_eq!(slots[0].state, models::State::BOOKED);
}