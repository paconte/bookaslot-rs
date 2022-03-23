table! {
    bookable (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    slots (id) {
        id -> Int4,
        state -> Varchar,
        start -> Int8,
        finish -> Int8,
        bookable -> Int4,
    }
}

joinable!(slots -> bookable (bookable));

allow_tables_to_appear_in_same_query!(
    bookable,
    slots,
);
