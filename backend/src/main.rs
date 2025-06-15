pub mod ps_conec;
pub mod api;
pub mod models;
pub mod ops;
pub mod schema;
use crate::ps_conec::update_migrations;

#[macro_use] extern crate rocket;
use api::{new_short_link_api, get_original_link_api};
use rocket::figment::Figment;


#[launch]
fn rocket() -> _ {
    let _ = update_migrations();

    let figment: Figment = Figment::from(rocket::Config::default())
        .merge(("port", std::env::var("PORT").unwrap().parse::<u16>().unwrap()))
        .merge(("address", "0.0.0.0"));

    rocket::custom(figment)
        .mount("/api/", routes![new_short_link_api])
        .mount("/api/", routes![get_original_link_api])
}