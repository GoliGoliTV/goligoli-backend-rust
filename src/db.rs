use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn db_controller() {
    let _s = establish_connection();
}

pub fn establish_connection() -> PgConnection{
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

// #[macro_use]
// extern crate diesel;
// extern crate dotenv;