use rocket::{http::Status, response::status, serde::json::Json};
use crate::models::{GetOriginalLinkRequest, GetOriginalLinkResponse, NewShortLinkRequest, NewShortLinkResponse};

//=================================================================================
#[post("/new_short_link", format = "json", data = "<new_short_link_data_json>")]
pub fn new_short_link_api(new_short_link_data_json: Json<NewShortLinkRequest>) -> Result<Json<NewShortLinkResponse>, status::Custom<Json<NewShortLinkResponse>>> {
    let new_short_link_data: NewShortLinkRequest = new_short_link_data_json.into_inner();

    match NewShortLinkResponse::create(new_short_link_data.link) {
        Some(short_lnk) => {
            Ok(Json(NewShortLinkResponse {
                status: "success".to_string(),
                message: "sucesso ao recuperar o link".to_string(),
                data: Some(short_lnk)
            }))
        },
        None => {
            Err(status::Custom(
                Status::InternalServerError, 
                Json(NewShortLinkResponse {
                status: "error".to_string(),
                message: "ERRO: nao foi possivel criar o link".to_string(),     
                data: None
            })))
        }
    }
}
//=================================================================================


//=================================================================================
#[post("/gt", format = "json", data = "<get_link_data_json>")]
pub fn get_original_link_api(get_link_data_json: Json<GetOriginalLinkRequest>) -> Result<Json<GetOriginalLinkResponse>, status::Custom<Json<GetOriginalLinkResponse>>> {
    let get_link_data: GetOriginalLinkRequest = get_link_data_json.into_inner();

    match GetOriginalLinkResponse::get(get_link_data.short_link) {
        Some(or_link) => {
            Ok(Json(GetOriginalLinkResponse {
                status: "success".to_string(),
                message: "sucesso ao recuperar o link".to_string(),
                data: Some(or_link)
            }))
        },
        None => {
            Err(status::Custom(
                Status::InternalServerError,
                Json(GetOriginalLinkResponse {
                status: "error".to_string(),
                message: "ERRO: link nao encontrado".to_string(),     
                data: None
            })))
        }
    }
}
//=================================================================================