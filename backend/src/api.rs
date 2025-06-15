use rocket::serde::json::Json;
use crate::models::{GetOriginalLinkRequest, GetOriginalLinkResponse, NewShortLinkRequest, NewShortLinkResponse};

//=================================================================================
#[post("/new_short_link", format = "json", data = "<new_short_link_data_json>")]
pub fn new_short_link_api(new_short_link_data_json: Json<NewShortLinkRequest>) -> Json<NewShortLinkResponse> {
    let new_short_link_data: NewShortLinkRequest = new_short_link_data_json.into_inner();

    return Json(NewShortLinkResponse::create(new_short_link_data.link));
}
//=================================================================================


//=================================================================================
#[post("/gt", format = "json", data = "<get_link_data_json>")]
pub fn get_original_link_api(get_link_data_json: Json<GetOriginalLinkRequest>) -> Json<GetOriginalLinkResponse> {
    let get_link_data: GetOriginalLinkRequest = get_link_data_json.into_inner();

    return Json(GetOriginalLinkResponse::get(get_link_data.short_link));
}
//=================================================================================