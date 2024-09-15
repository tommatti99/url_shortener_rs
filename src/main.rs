pub mod ps_conec;
pub mod api;
pub mod models;
pub mod ops;
pub mod schema;

#[macro_use] extern crate rocket;
use api::{new_short_link_api, get_original_link_api};


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![new_short_link_api])
        .mount("/", routes![get_original_link_api])
}