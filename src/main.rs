#[macro_use]
extern crate rocket;

use chrono::{Duration, TimeZone, Utc};
use rocket::serde::json::Json;
use std::collections::BTreeMap;
use std::net::{IpAddr, Ipv4Addr};

mod models;

pub use models::models::{Slot, State, Template, TimeRange};
pub use models::responses::{TimeItems, DailySortedSlots};


#[get("/")]
fn index() -> &'static str {
    "This is my Rust reservation REST API"
}


#[get("/getBookings1")]
fn get_booking_state_1() -> Json<Vec<Vec<Slot>>> {
    let slots = get_booking_data_day_1();
    let result = vec![
        vec![
            Slot {
                id: 2,
                start: slots[0].init,
                finish: slots[0].end,
                state: State::FREE,
            },
            Slot {
                id: 3,
                start: slots[0].init,
                finish: slots[0].end,
                state: State::FREE,
            },
        ],
        vec![
            Slot {
                id: 5,
                start: slots[1].init,
                finish: slots[1].end,
                state: State::FREE,
            },
            Slot {
                id: 6,
                start: slots[1].init,
                finish: slots[1].end,
                state: State::FREE,
            },
        ],
        vec![
            Slot {
                id: 8,
                start: slots[2].init,
                finish: slots[2].end,
                state: State::FREE,
            },
            Slot {
                id: 9,
                start: slots[2].init,
                finish: slots[2].end,
                state: State::FREE,
            },
        ],
        vec![
            Slot {
                id: 11,
                start: slots[3].init,
                finish: slots[3].end,
                state: State::FREE,
            },
            Slot {
                id: 12,
                start: slots[3].init,
                finish: slots[3].end,
                state: State::FREE,
            },
        ],
        vec![
            Slot {
                id: 14,
                start: slots[4].init,
                finish: slots[4].end,
                state: State::FREE,
            },
            Slot {
                id: 15,
                start: slots[4].init,
                finish: slots[4].end,
                state: State::FREE,
            },
        ],
    ];

    Json(result)
}


#[get("/getBookings2")]
fn get_booking_state_2() -> Json<BTreeMap<String, Vec<Slot>>> {
    let slots = get_booking_data_day_1();

    let result = BTreeMap::from([
        (
            slots[0].to_string(),
            vec![
                Slot {
                    id: 1,
                    start: slots[0].init,
                    finish: slots[0].end,
                    state: State::FREE,
                },
                Slot {
                    id: 2,
                    start: slots[0].init,
                    finish: slots[0].end,
                    state: State::FREE,
                },
            ],
        ),
        (
            slots[1].to_string(),
            vec![
                Slot {
                    id: 3,
                    start: slots[1].init,
                    finish: slots[1].end,
                    state: State::FREE,
                },
                Slot {
                    id: 4,
                    start: slots[1].init,
                    finish: slots[1].end,
                    state: State::FREE,
                },
            ],
        ),
        (
            slots[2].to_string(),
            vec![
                Slot {
                    id: 5,
                    start: slots[2].init,
                    finish: slots[2].end,
                    state: State::FREE,
                },
                Slot {
                    id: 6,
                    start: slots[2].init,
                    finish: slots[2].end,
                    state: State::FREE,
                },
            ],
        ),
    ]);

    Json(result)
}


#[get("/getBookings4")]
fn get_booking_state_4() -> Json<Vec<TimeItems>> {

    let slots = get_booking_data_day_1();
    let result = vec![
        TimeItems {
            time: slots[0],
            items: vec![
                Slot {
                    id: 1,
                    start: slots[0].init,
                    finish: slots[0].end,
                    state: State::FREE,
                },
                Slot {
                    id: 2,
                    start: slots[0].init,
                    finish: slots[0].end,
                    state: State::FREE,
                },
            ],
        },
        TimeItems {
            time: slots[1],
            items: vec![
                Slot {
                    id: 3,
                    start: slots[1].init,
                    finish: slots[1].end,
                    state: State::FREE,
                },
                Slot {
                    id: 4,
                    start: slots[1].init,
                    finish: slots[1].end,
                    state: State::FREE,
                },
            ],
        },
        TimeItems {
            time: slots[2],
            items: vec![
                Slot {
                    id: 5,
                    start: slots[2].init,
                    finish: slots[2].end,
                    state: State::FREE,
                },
                Slot{
                    id: 6,
                    start: slots[2].init,
                    finish: slots[2].end,
                    state: State::FREE,
                },
            ],
        },
        TimeItems {
            time: slots[3],
            items: vec![
                Slot{
                    id: 7,
                    start: slots[3].init,
                    finish: slots[3].end,
                    state: State::FREE,
                },
                Slot {
                    id: 8,
                    start: slots[3].init,
                    finish: slots[3].end,
                    state: State::FREE,
                },
            ],
        },
        TimeItems {
            time: slots[4],
            items: vec![
                Slot {
                    id: 9,
                    start: slots[4].init,
                    finish: slots[4].end,
                    state: State::FREE,
                },
                Slot{
                    id: 10,
                    start: slots[4].init,
                    finish: slots[4].end,
                    state: State::FREE,
                },
            ],
        },
    ];

    Json(result)
}


#[get("/getBookings5")]
fn get_bookings_state_5() -> Json<Vec<TimeItems>> {
    let template = create_template(30);
    let slots = Template::generate_slots(template, 2);
    let hourly_slots = Slot::to_hour_map(slots);
    let result = TimeItems::to_hour_response(hourly_slots);

    Json(result)
}


#[get("/getBookings6")]
fn get_bookings_state_6() -> Json<Vec<DailySortedSlots>>  {
    let template = create_template(30);
    let slots = Template::generate_slots(template, 2);
    let daily_slots = Slot::to_day_map(slots);
    let result = DailySortedSlots::to_day_response(daily_slots);

    Json(result)
}


fn get_booking_data_day_1() -> Vec<TimeRange> {
    let init_1 = Utc.ymd(2022, 1, 1).and_hms(9, 0, 0);
    let end_1 = Utc.ymd(2022, 1, 1).and_hms(10, 0, 0);
    let slot_1 = TimeRange::new(init_1, end_1);

    let init_2 = Utc.ymd(2022, 1, 1).and_hms(10, 0, 0);
    let end_2 = Utc.ymd(2022, 1, 1).and_hms(11, 0, 0);
    let slot_2 = TimeRange::new(init_2, end_2);

    let init_3 = Utc.ymd(2022, 1, 1).and_hms(11, 0, 0);
    let end_3 = Utc.ymd(2022, 1, 1).and_hms(12, 0, 0);
    let slot_3 = TimeRange::new(init_3, end_3);

    let init_4 = Utc.ymd(2022, 1, 1).and_hms(12, 0, 0);
    let end_4 = Utc.ymd(2022, 1, 1).and_hms(13, 0, 0);
    let slot_4 = TimeRange::new(init_4, end_4);

    let init_5 = Utc.ymd(2022, 1, 1).and_hms(13, 0, 0);
    let end_5 = Utc.ymd(2022, 1, 1).and_hms(14, 0, 0);
    let slot_5 = TimeRange::new(init_5, end_5);

    vec![slot_1, slot_2, slot_3, slot_4, slot_5]
}


fn create_template(add: i64) -> Template {
    let template = Template {
        init_day: Utc::today(),
        end_day: Utc::today() + Duration::days(add),
        init_time: chrono::NaiveTime::from_hms(9, 0, 0),
        end_time: chrono::NaiveTime::from_hms(14, 0, 0),
        duration: Duration::minutes(30),
    };

    template
}


#[rocket::main]
async fn main() {
    let mut config = rocket::config::Config::default();
    config.address = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));

    let _ = rocket::build()
        .configure(config)
        .mount(
            "/",
            routes![
                index,
                get_booking_state_1,
                get_booking_state_2,
                get_booking_state_4,
                get_bookings_state_5,
                get_bookings_state_6,
            ],
        )
        .launch()
        .await;
}
