extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use dotenv::dotenv;


mod db;
mod models;
mod schema;

use db::{establish_connection, PgPooledConnection};
use schema::users::dsl::*;

fn main() {
    dotenv().ok();
    let pool = establish_connection();
    let mut connection: PgPooledConnection = pool.get().expect("Couldn't get DB connection from pool");


    let users_list = users.load::<models::User>(&mut connection).expect("Error loading users");

    println!("Users: {:?}", users_list);
}
