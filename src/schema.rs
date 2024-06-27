#[macro_use]
extern crate diesel;

diesel::table! {
    users {
        id -> Integer,
        username -> Text,
        email -> Text,
    }
}
