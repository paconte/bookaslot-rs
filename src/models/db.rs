use diesel::{Insertable, Queryable};
use diesel::prelude::*;
use diesel::pg::upsert::excluded;

use crate::models::schema::slots;
use crate::models::models::Slot;


#[derive(Insertable, Queryable, Debug)]
#[table_name="slots"]
pub struct DbSlot {
    pub id: i32,
    pub state: String,
    pub start: i64,
    pub finish: i64
}


pub fn load_slots(conn: &diesel::PgConnection) -> Vec<Slot> {
    let all_db_slots = load_db_slots(conn);
    let all_slots: Vec<Slot> = DbSlot::into_slots(all_db_slots);
    all_slots
}


fn load_db_slots(conn: &diesel::PgConnection) -> Vec<DbSlot> {
    slots::table.load::<DbSlot>(conn).unwrap()
}


pub fn insert_slots(conn: &diesel::PgConnection, slots: Vec<Slot>) {
    let db_slots: Vec<DbSlot> = DbSlot::into_db_slots(slots);
    insert_db_slots(conn, db_slots)
}


fn insert_db_slot(conn: &diesel::PgConnection, slot: DbSlot) {
    use crate::models::schema::slots::{state, start, finish};
    diesel::insert_into(slots::table)
        .values((state.eq(slot.state), start.eq(slot.start), finish.eq(slot.finish)))
        .execute(conn)
        .unwrap();
}


fn insert_db_slots(conn: &diesel::PgConnection, slots: Vec<DbSlot>) {
    for slot in slots {
        insert_db_slot(conn, slot)
    }
}


fn raw_insert_db_slots(conn: &diesel::PgConnection, slots: Vec<DbSlot>) {
    use crate::models::schema::slots::{id, state};
    diesel::insert_into(slots::table)
        .values(&slots)
        .on_conflict(id)
        .do_update()
        .set(state.eq(excluded(state)))
        .execute(conn)
        .unwrap();
}


pub fn test_database(conn: &diesel::PgConnection) {
    let slots = vec![
        DbSlot{id: 1, state: String::from("FREE"), start: 100, finish: 200},
        DbSlot{id: 2, state: String::from("FREE"), start: 200, finish: 300},
        DbSlot{id: 3, state: String::from("BOOKED"), start: 300, finish: 400}
    ];
    raw_insert_db_slots(conn, slots);

    let all_slots = load_slots(conn);
    println!("{:?}", all_slots);
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

    pub fn into_slots(items: Vec<DbSlot>) -> Vec<Slot> {
        items
            .into_iter()
            .map(|item| item.into())
            .collect()
    }
}


impl From<Slot> for DbSlot {
    fn from(item: Slot) -> Self {
        DbSlot {
            id: item.id,
            state: item.state.to_string(),
            start: item.start,
            finish: item.finish,
        }
    }
}