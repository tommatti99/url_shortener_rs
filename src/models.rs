use chrono::NaiveDate;
use crate::schema;
use crate::ops::{get_original_link, generate_short_link, get_short_link, short_link_exists, insert_new_link};

#[derive(Debug, PartialEq, Insertable, Deserialize, Serialize, Queryable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = "crate::schema::db_links")]
pub struct DbLinks {
    dt: NaiveDate,
    original_link: String, 
    short_link: String,
    clicks: i64
}


pub struct NewShortLinkResponse {
    pub status: bool,
    pub short_link: String
}
impl NewShortLinkResponse {
    pub fn create(original_link: String) -> Self {
        if !insert_new_link(original_link) {
            return Self {
                status: false,
                short_link: "ERRO: nao foi possivel criar o link"        
            }
        }
        
        return Self {
            status: true,
            short_link: get_short_link(original_link)
        }
    }
}




pub struct GetOriginalLinkResponse {
    pub status: bool,
    pub original_link: String
}
impl GetOriginalLinkResponse {
    pub fn get(short_link: String) -> Self {
        if short_link_exists
            return Self { 
                status: true,
                original_link: get_original_link(short_link)
            }
        
        return Self {
            status: false,
            original_link: "ERRO: link nao encontrado".to_string()
        }
    }
}