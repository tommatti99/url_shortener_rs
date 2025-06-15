use rocket::serde::json::Json;
use crate::models::{NewShortLinkResponse, GetOriginalLinkResponse};

//=================================================================================
#[get("/new_short_link/<original_link>")]
pub fn new_short_link_api(original_link: String) -> Json<NewShortLinkResponse> {
    return Json(NewShortLinkResponse::create(original_link));
}
//=================================================================================


//=================================================================================
#[get("/gt/<short_link>")]
pub fn get_original_link_api(short_link: String) -> Json<GetOriginalLinkResponse> {
    return Json(GetOriginalLinkResponse::get(short_link));
}
//=================================================================================