use chrono::NaiveDate;
use crate::{ops::{clean_string, get_original_link, get_short_link, insert_new_link, short_link_exists}};
use rocket::serde::{Deserialize, Serialize};
use diesel::{Insertable, Queryable};

#[derive(Debug, PartialEq, Insertable, Deserialize, Serialize, Queryable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = crate::schema::db_links)]
pub struct DbLinks {
    dt: NaiveDate,
    original_link: String, 
    short_link: String,
    clicks: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewShortLinkRequest {
    pub link: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataNewShortLinkResponse {
    pub short_link: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewShortLinkResponse {
    pub status: String,
    pub message: String,
    pub data: Option<DataNewShortLinkResponse>
}

impl NewShortLinkResponse {
    pub fn create(original_lnk: String) -> Option<DataNewShortLinkResponse> {
        let clear_original_link = clean_string(original_lnk);

        if insert_new_link(clear_original_link.clone()) {
            let short_lnk: String = get_short_link(clear_original_link);

            return Some(DataNewShortLinkResponse { short_link: short_lnk} );
        }

        return None;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetOriginalLinkRequest {
    pub short_link: String
}

#[derive(Debug, Serialize, Deserialize)]

pub struct DataGetOriginalLinkResponse {
    pub original_link: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetOriginalLinkResponse {
    pub status: String,
    pub message: String,
    pub data: Option<DataGetOriginalLinkResponse>
}

impl GetOriginalLinkResponse {
    pub fn get(short_link: String) -> Option<DataGetOriginalLinkResponse> {
        let clear_short_link = clean_string(short_link);

        if short_link_exists(clear_short_link.clone()) {
            let or_link: String = get_original_link(clear_short_link);

            return Some(DataGetOriginalLinkResponse { original_link: or_link });
        };
        
        return None;
    }
}