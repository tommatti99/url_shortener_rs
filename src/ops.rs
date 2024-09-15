use diesel::prelude::*;
use crate::ps_conec::start_connection;
use crate::schema;
use rand::{distributions::Alphanumeric, Rng};  
use chrono::Utc;

pub fn clean_string(link: String) -> String {
    let _ = link.replace("www.example.com/", "").replace("\"", "").replace("\'", "").to_lowercase().trim().to_string();
    return link;
}

pub fn short_link_exists(the_original_link: String) -> bool {
    let mut conec: PgConnection = start_connection();

    match 
        diesel::select(diesel::dsl::
            exists(schema::db_links::dsl::db_links
            .filter(schema::db_links::dsl::original_link
                .eq(the_original_link))))
            .get_result::<bool>(&mut conec) {
                Ok(usr_exists) => {
                    return true;
                },
                Err(_) => {
                    return false;
                }            
            };
    }


pub fn get_original_link(the_short_link: String) -> String {
    let mut conec: PgConnection = start_connection();

    let link = schema::db_links::dsl::db_links
        .select(schema::db_links::dsl::original_link)
        .filter(schema::db_links::dsl::short_link.eq(the_short_link.clone()))
        .first::<String>(&mut conec)
        .unwrap();

    let _ = incrase_clicks(the_short_link);

    return link;
}

pub fn get_short_link(the_original_link: String) -> String {
    let mut conec: PgConnection = start_connection();

    let link = schema::db_links::dsl::db_links
        .select(schema::db_links::dsl::short_link)
        .filter(schema::db_links::dsl::original_link.eq(the_original_link))
        .first::<String>(&mut conec)
        .unwrap();

    return link;
}

fn incrase_clicks(the_short_link: String) -> bool {
    let mut conec: PgConnection = start_connection();

    match diesel::update(schema::db_links::dsl::db_links
        .filter(schema::db_links::dsl::short_link.eq(the_short_link)))
        .set(schema::db_links::dsl::clicks.eq(schema::db_links::dsl::clicks + 1))
        .execute(&mut conec) {
            Ok(_) => {
                return true;
            },
            Err(_) => {
                return false;
            }
        }
}

fn generate_random_chars() -> String {
    let mut rng = rand::thread_rng();

    let string_len = rng.gen_range(5..=8);

    let rand_str = (0..=string_len)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect();

    return rand_str;
}

pub fn insert_new_link(the_original_link: String) -> bool {
    if !short_link_exists(the_original_link.clone()) { 
        return false;
    };

    let mut conec: PgConnection = start_connection();

    let random_link = format!("{}",generate_random_chars());

    match diesel::insert_into(schema::db_links::dsl::db_links) 
        .values((
            schema::db_links::dsl::dt.eq(Utc::now().date_naive()),
            schema::db_links::dsl::original_link.eq(the_original_link),
            schema::db_links::dsl::short_link.eq(random_link),
            schema::db_links::dsl::clicks.eq(0)
        )) 
        .execute(&mut conec) {
            Ok(_) => {
                return true;
            },
            Err(_) => {
                return false;
            }
        }
}   