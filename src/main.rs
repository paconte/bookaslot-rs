#[macro_use]
extern crate rocket;

use chrono::{TimeZone, Utc};
use rocket::serde::json::Json;
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};

mod responses;

#[get("/")]
fn index() -> &'static str {
    "This is my Rust reservation REST API"
}

#[get("/getBookings1")]
fn get_booking_status_1() -> Json<Vec<Vec<responses::Slot>>> {
    let slots = get_booking_data();
    let result = vec![
        vec![
            responses::Slot {
                id: 2,
                time: slots[0],
                status: responses::SlotStatus::FREE,
            },
            responses::Slot {
                id: 3,
                time: slots[0],
                status: responses::SlotStatus::FREE,
            },
        ],
        vec![
            responses::Slot {
                id: 5,
                time: slots[1],
                status: responses::SlotStatus::FREE,
            },
            responses::Slot {
                id: 6,
                time: slots[1],
                status: responses::SlotStatus::FREE,
            },
        ],
        vec![
            responses::Slot {
                id: 8,
                time: slots[2],
                status: responses::SlotStatus::FREE,
            },
            responses::Slot {
                id: 9,
                time: slots[2],
                status: responses::SlotStatus::FREE,
            },
        ],
        vec![
            responses::Slot {
                id: 11,
                time: slots[3],
                status: responses::SlotStatus::FREE,
            },
            responses::Slot {
                id: 12,
                time: slots[3],
                status: responses::SlotStatus::FREE,
            },
        ],
        vec![
            responses::Slot {
                id: 14,
                time: slots[4],
                status: responses::SlotStatus::FREE,
            },
            responses::Slot {
                id: 15,
                time: slots[4],
                status: responses::SlotStatus::FREE,
            },
        ],
    ];

    Json(result)
}

#[get("/getBookings2")]
fn get_booking_status_2() -> Json<HashMap<String, Vec<responses::Slot>>> {
    let slots = get_booking_data();

    let result = HashMap::from([
        (
            slots[0].to_string(),
            vec![
                responses::Slot {
                    id: 1,
                    time: slots[0],
                    status: responses::SlotStatus::FREE,
                },
                responses::Slot {
                    id: 2,
                    time: slots[0],
                    status: responses::SlotStatus::FREE,
                },
            ],
        ),
        (
            slots[1].to_string(),
            vec![
                responses::Slot {
                    id: 3,
                    time: slots[1],
                    status: responses::SlotStatus::FREE,
                },
                responses::Slot {
                    id: 4,
                    time: slots[1],
                    status: responses::SlotStatus::FREE,
                },
            ],
        ),
        (
            slots[2].to_string(),
            vec![
                responses::Slot {
                    id: 5,
                    time: slots[2],
                    status: responses::SlotStatus::FREE,
                },
                responses::Slot {
                    id: 6,
                    time: slots[2],
                    status: responses::SlotStatus::FREE,
                },
            ],
        ),
    ]);

    Json(result)
}

#[get("/getBookings3")]
fn get_booking_status_3() -> Json<Vec<Vec<responses::SlotTable>>> {
    let slots = get_booking_data();
    let result = vec![
        vec![
            responses::SlotTable::Time(responses::SlotTableTime::new(slots[0])),
            responses::SlotTable::Item(responses::Slot {
                id: 1,
                time: slots[0],
                status: responses::SlotStatus::FREE,
            }),
            responses::SlotTable::Item(responses::Slot {
                id: 2,
                time: slots[0],
                status: responses::SlotStatus::FREE,
            }),
        ],
        vec![
            responses::SlotTable::Time(responses::SlotTableTime::new(slots[1])),
            responses::SlotTable::Item(responses::Slot {
                id: 3,
                time: slots[1],
                status: responses::SlotStatus::FREE,
            }),
            responses::SlotTable::Item(responses::Slot {
                id: 4,
                time: slots[1],
                status: responses::SlotStatus::FREE,
            }),
        ],
        vec![
            responses::SlotTable::Time(responses::SlotTableTime::new(slots[2])),
            responses::SlotTable::Item(responses::Slot {
                id: 5,
                time: slots[2],
                status: responses::SlotStatus::FREE,
            }),
            responses::SlotTable::Item(responses::Slot {
                id: 6,
                time: slots[2],
                status: responses::SlotStatus::FREE,
            }),
        ],
        vec![
            responses::SlotTable::Time(responses::SlotTableTime::new(slots[3])),
            responses::SlotTable::Item(responses::Slot {
                id: 7,
                time: slots[3],
                status: responses::SlotStatus::FREE,
            }),
            responses::SlotTable::Item(responses::Slot {
                id: 8,
                time: slots[3],
                status: responses::SlotStatus::FREE,
            }),
        ],
        vec![
            responses::SlotTable::Time(responses::SlotTableTime::new(slots[4])),
            responses::SlotTable::Item(responses::Slot {
                id: 9,
                time: slots[4],
                status: responses::SlotStatus::FREE,
            }),
            responses::SlotTable::Item(responses::Slot {
                id: 10,
                time: slots[4],
                status: responses::SlotStatus::FREE,
            }),
        ],
    ];

    Json(result)
}

/*
TODO: make getBookings4 for the following json structure
{
    [
        {
            time: {},
            items: []
        }
    ]
}
*/

fn get_booking_data() -> Vec<responses::SlotTime> {
    let init_1 = Utc.ymd(2022, 1, 1).and_hms(9, 0, 0);
    let end_1 = Utc.ymd(2022, 1, 1).and_hms(10, 0, 0);
    let slot_1 = responses::SlotTime::new(init_1, end_1);
    let init_2 = Utc.ymd(2022, 1, 1).and_hms(10, 0, 0);
    let end_2 = Utc.ymd(2022, 1, 1).and_hms(11, 0, 0);
    let slot_2 = responses::SlotTime::new(init_2, end_2);
    let init_3 = Utc.ymd(2022, 1, 1).and_hms(11, 0, 0);
    let end_3 = Utc.ymd(2022, 1, 1).and_hms(12, 0, 0);
    let slot_3 = responses::SlotTime::new(init_3, end_3);
    let init_4 = Utc.ymd(2022, 1, 1).and_hms(12, 0, 0);
    let end_4 = Utc.ymd(2022, 1, 1).and_hms(13, 0, 0);
    let slot_4 = responses::SlotTime::new(init_4, end_4);
    let init_5 = Utc.ymd(2022, 1, 1).and_hms(13, 0, 0);
    let end_5 = Utc.ymd(2022, 1, 1).and_hms(14, 0, 0);
    let slot_5 = responses::SlotTime::new(init_5, end_5);

    vec![slot_1, slot_2, slot_3, slot_4, slot_5]
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
                get_booking_status_1,
                get_booking_status_2,
                get_booking_status_3
            ],
        )
        .launch()
        .await;
}
