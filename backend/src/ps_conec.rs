use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn start_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE MUST BE SET");

    return PgConnection::establish(&database_url)
        .expect("Unavailable Data Base");
}

