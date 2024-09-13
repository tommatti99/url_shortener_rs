use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

dotenv().ok();
pub fn start_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE MUST BE SET");

    return PgConnection::establish(&database_url)
        .expect("Unavailable Data Base");
}

