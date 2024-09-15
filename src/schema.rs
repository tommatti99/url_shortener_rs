// @generated automatically by Diesel CLI.

diesel::table! {
    db_links (original_link) {
        dt -> Date,
        original_link -> Varchar,
        short_link -> Varchar,
        clicks -> Int4,
    }
}
