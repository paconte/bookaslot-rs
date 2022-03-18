use diesel::{Insertable, Queryable};
use diesel::prelude::*;
use diesel::pg::upsert::excluded;
use diesel::result::Error;

use crate::models::schema::slots;
use crate::models::schema::slots::{id, state, start, finish};
use crate::models::models::Slot;


#[derive(Insertable, Queryable, Debug)]
#[table_name="slots"]
pub struct DbSlot {
    pub id: i32,
    pub state: String,
    pub start: i64,
    pub finish: i64
}


/*
fn error_status(error: Error) -> Failure {
    Failure(match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    })
}
*/

fn fetch_db_slots_by_time(begin: i64, end: i64, conn: &diesel::PgConnection) -> Result<Vec<Slot>, Error> {
    let db_slots = slots::table.filter(start.ge(begin)).filter(finish.le(end)).load::<DbSlot>(conn)?;
    let slots: Vec<Slot> = DbSlot::into_slots(db_slots);
    Ok(slots)
}


pub fn fetch_all_slots(conn: &diesel::PgConnection) ->  Result<Vec<Slot>, Error> {
    let all_db_slots = slots::table.load::<DbSlot>(conn).unwrap();
    let all_slots: Vec<Slot> = DbSlot::into_slots(all_db_slots);
    Ok(all_slots)
}


pub fn insert_slots(conn: &diesel::PgConnection, slots: Vec<Slot>) -> QueryResult<usize> {
    let db_slots: Vec<DbSlot> = DbSlot::into_db_slots(slots);
    insert_db_slots(conn, db_slots)
}


fn insert_db_slot(conn: &diesel::PgConnection, slot: DbSlot) -> QueryResult<usize> {
    diesel::insert_into(slots::table)
        .values((state.eq(slot.state), start.eq(slot.start), finish.eq(slot.finish)))
        .execute(conn)
}


fn insert_db_slots(conn: &diesel::PgConnection, slots: Vec<DbSlot>) -> QueryResult<usize> {
    let mut new_slots = Vec::new();
    for slot in slots {
        new_slots.push((state.eq(slot.state), start.eq(slot.start), finish.eq(slot.finish)));
    }
    diesel::insert_into(slots::table)
        .values(&new_slots)
        .execute(conn)
    /*
    for slot in slots {
        insert_db_slot(conn, slot)
    }
    */
}


fn raw_insert_db_slots(conn: &diesel::PgConnection, slots: Vec<DbSlot>) {
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

    let all_slots = fetch_all_slots(conn).unwrap();
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
        }
    }
}