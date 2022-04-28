use diesel::prelude::*;
use diesel::pg::upsert::excluded;
use diesel::result::Error;

use crate::models::models::{Bookable, DbSlot, Slot};

use crate::models::schema::bookable;
use crate::models::schema::slots;
use crate::models::schema::slots::{id, state, start, finish, bookable_id};


/*
fn error_status(error: Error) -> Failure {
    Failure(match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    })
}


fn fetch_db_slots_by_time(begin: i64, end: i64, conn: &diesel::PgConnection) -> Result<Vec<Slot>, Error> {
    let db_slots = slots::table.filter(start.ge(begin)).filter(finish.le(end)).load::<DbSlot>(conn)?;
    let slots: Vec<Slot> = DbSlot::into_slots(db_slots);
    Ok(slots)
}
*/


fn fetch_db_slots_by_time(begin: i64, end: i64, conn: &diesel::PgConnection) -> Result<Vec<Slot>, Error> {
    let data = bookable::table.inner_join(slots::table)
        .filter(start.ge(begin))
        .filter(finish.le(end))
        .load::<(Bookable, DbSlot)>(conn)?;
    let slots: Vec<Slot> = DbSlot::into_slots(data);
    Ok(slots)
}


pub fn fetch_all_slots(conn: &diesel::PgConnection) ->  Result<Vec<Slot>, Error> {
    let data = bookable::table.inner_join(slots::table)
        .load::<(Bookable, DbSlot)>(conn)?;
    let slots: Vec<Slot> = DbSlot::into_slots(data);
    Ok(slots)
}


pub fn insert_slots(conn: &diesel::PgConnection, slots: Vec<Slot>) -> QueryResult<usize> {
    let db_slots: Vec<DbSlot> = DbSlot::into_db_slots(slots);
    insert_db_slots(conn, db_slots)
}


fn insert_db_slots(conn: &diesel::PgConnection, slots: Vec<DbSlot>) -> QueryResult<usize> {
    let mut new_slots = Vec::new();
    for slot in slots {
        new_slots.push(
            (state.eq(slot.state), start.eq(slot.start), finish.eq(slot.finish), bookable_id.eq(slot.bookable_id))
        );
    }
    diesel::insert_into(slots::table)
        .values(&new_slots)
        .execute(conn)
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


/**
 * Bookable methods
 */
fn insert_bookables(conn: &diesel::PgConnection, bookables: Vec<&str>) -> QueryResult<usize> {
    let mut new_bookables = Vec::new();
    for item in bookables {
        new_bookables.push(bookable::name.eq(item));
    }
    diesel::insert_into(bookable::table)
        .values(&new_bookables)
        .execute(conn)
}


/**
 * TEST METHODS (to be deleted)
 */

pub fn test_database(conn: &diesel::PgConnection) {
    let slots = vec![
        DbSlot{id: 1, state: String::from("FREE"), start: 100, finish: 200, bookable_id: 1},
        DbSlot{id: 2, state: String::from("FREE"), start: 200, finish: 300, bookable_id: 1},
        DbSlot{id: 3, state: String::from("BOOKED"), start: 300, finish: 400, bookable_id: 1}
    ];
    raw_insert_db_slots(conn, slots);

    let all_slots = fetch_all_slots(conn).unwrap();
}


/// Initizialize the database with some bookable objects.
/// The bookable objects can be used to book slots.
pub fn init_database(conn: &diesel::PgConnection, bookables: (&str, &str, &str)) -> QueryResult<usize> {
    let mut new_bookables = Vec::new();
    new_bookables.push(bookables.0);
    new_bookables.push(bookables.1);
    new_bookables.push(bookables.2);
    insert_bookables(conn, new_bookables)
}