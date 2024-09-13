pub mod ps_conec;
pub mod api;
pub mod models;
pub mod ops;


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![api::new_short_link_api])
        .mount("/api", routes![api::get_original_link])
}