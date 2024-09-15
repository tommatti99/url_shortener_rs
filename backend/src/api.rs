use rocket::serde::json::Json;
use crate::models::{NewShortLinkResponse, GetOriginalLinkResponse};

//=================================================================================
//  REQUEST: GET /new_short_link/<link>
//    
//  RESPONSE:
//      Header: 
//          Content-Type: application/json
//      Body:
//          {
//              "status": bool,
//              "short_link": String 
//          }
//
#[get("/new_short_link/<original_link>")]
pub fn new_short_link_api(original_link: String) -> Json<NewShortLinkResponse> {
    return Json(NewShortLinkResponse::create(original_link));
}
//=================================================================================




//=================================================================================
//  REQUEST: GET /gt/<link>
//    
//  RESPONSE:
//      Header: 
//          Content-Type: application/json
//      Body:
//          {
//              "status": bool,
//              "original_link": String 
//          }
//
#[get("/gt/<short_link>")]
pub fn get_original_link_api(short_link: String) -> Json<GetOriginalLinkResponse> {
    return Json(GetOriginalLinkResponse::get(short_link));
}
//=================================================================================