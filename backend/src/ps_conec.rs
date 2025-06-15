use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use diesel_migrations::{EmbeddedMigrations, embed_migrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");


pub fn update_migrations() -> () {
    println!("run migrations");
    
    let mut conn: PgConnection = start_connection();
        conn.run_pending_migrations(MIGRATIONS).unwrap();

    ()
}



pub fn start_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE MUST BE SET");

    return PgConnection::establish(&database_url)
        .expect("Unavailable Data Base");
}
