use chrono::NaiveDate;
use crate::ops::{get_original_link, get_short_link, clean_string, short_link_exists, insert_new_link};
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
pub struct NewShortLinkResponse {
    pub status: bool,
    pub short_link: String
}
impl NewShortLinkResponse {
    pub fn create(original_link: String) -> Self {
        let clear_original_link = clean_string(original_link);

        if !insert_new_link(clear_original_link.clone()) {
            return Self {
                status: false,
                short_link: "ERRO: nao foi possivel criar o link".to_string()      
            }
        }
        
        return Self {
            status: true,
            short_link: get_short_link(clear_original_link)
        }
    }
}




#[derive(Debug, Serialize, Deserialize)]
pub struct GetOriginalLinkResponse {
    pub status: bool,
    pub original_link: String
}
impl GetOriginalLinkResponse {
    pub fn get(short_link: String) -> Self {
        let clear_short_link = clean_string(short_link);

        if short_link_exists(clear_short_link.clone()) {
            return Self { 
                status: true,
                original_link: clean_string(get_original_link(clear_short_link))
            }
        };
        
        return Self {
            status: false,
            original_link: "ERRO: link nao encontrado".to_string()
        }
    }
}