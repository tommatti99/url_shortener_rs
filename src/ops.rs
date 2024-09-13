use crate::models::DbLinks;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use crate::ps_conec::start_connection;
use chrono::Utc;
use rand::{distributions::Alphanumeric, Rng};

pub fn short_link_exists(the_original_link: String) -> bool {
    let mut conec: PgConnection = start_connection();
    
    let usr_exists: bool = schema::db_links::dsl::db_links
    .select(diesel::exists(schema::db_links::dsl::short_link
        .filter(schema::db_links::dsl::original_link.eq(the_original_link))))
        .get_result::<bool>(&mut conec) {
            .exception("Ocorreu um erro durante a geração do link")
        };
        
        return usr_exists;
    }


pub fn get_original_link(the_short_link: String) -> String {
    let mut conec: PgConnection = start_connection();

    let link = schema::db_links::dsl::db_links
        .select(schema::db_links::dsl::the_original_link)
        .filter(schema::dn_links::dsl::short_link.eq(the_short_link))
        .first(&mut conec)
        .exception("Ocorreu um erro durante a busca do link");

    return link;
}

pub fn get_short_link(the_original_link: String) -> String {
    let mut conec: PgConnection = start_connection();

    let link = schema::db_links::dsl::db_links
        .select(schema::db_links::dsl::short_link)
        .filter(schema::dn_links::dsl::original_link.eq(the_original_link))
        .first(&mut conec)
        .exception("Ocorreu um erro durante a busca do link");

    return link;
}

fn incrase_clicks(the_short_link: String) -> () {
    diesel::update(schema::db_links::dsl::db_links
        .filter(schema::db_links::dsl::short_link.eq(the_short_link))
        .set(schema::db_links::dsl::clicks.eq(schema::db_links::dsl::clicks + 1)))
        .execute(&mut conec)
        .exception(format!("ERRO AO ADICIONAR CLIQUE NAS ESTATISTICAS DO LINK: {}", the_short_link))
}

fn generate_random_chars() -> String {
    let rng = rand::thread_rng();
    let string_len = rng.gen_range(5..=8);
    let rand_str = (0..=string_len)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect();

    return rand_str;
}

pub fn insert_new_link(the_original_link: String) -> bool {
    if if !short_link_exists(original_link) { 
        return false;
    }

    let mut conec: PgConnection = start_connection();
    dotenv().ok();
    let domain = env::var("DOMAIN").expect("DOMAIN MUST BE SET");
    let random_link = format!("{}{}", domain, generate_random_chars());

    match diesel::insert_into(schema::db_links::dsl::db_links) 
        .values((
            schema::db_links::dsl::dt.eq(Utc::now())
            schema::db_links::dsl::original_link.eq(the_original_link)
            schema::db_links::dsl::short_link.eq(random_link)
            schema::db_links::dsl::clicks.eq(0)
        )) 
        .execute(&mut conec) {
            Ok => {
                return true;
            },
            Err(_) => {
                return false;
            }
        }
}   