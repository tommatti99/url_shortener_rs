use std::collections::HashMap;
use yew::{functional, prelude::*};
use reqwest;
use web_sys;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;

const NEW_SHORT_LINK_API: &str = "http://127.0.0.1:8000/new_short_link/";
const GET_LINK_API: &str = "http://127.0.0.1:8000/gt/";


#[derive(Deserialize, Debug)]
pub struct NewShortLinkResponse {
    pub status: bool,
    pub short_link: String
}

#[derive(Deserialize, Debug)]
pub struct GetOriginalLinkResponse {
    pub status: bool,
    pub original_link: String
}

#[function_component]
pub fn Form() -> Html {
    return html!{

        
    };
}

async fn register_new_link(new_link: String) -> NewShortLinkResponse {
    let client = reqwest::Client::new();
    
    match client
        .get(format!("{}{}", NEW_SHORT_LINK_API, new_link))
        .send()
        .await {
            Ok(response) => {
                match response.json::<NewShortLinkResponse>().await {
                    Ok(short_link) => {
                        return NewShortLinkResponse {
                            status: short_link.status,
                            short_link: short_link.short_link
                        };
                    },
                    Err(_) => {
                        return NewShortLinkResponse {
                            status: false,
                            short_link: "error".to_string()
                    }
                }
            }
            },
        Err(_) => {
            return NewShortLinkResponse {
                status: false,
                short_link: "error".to_string()
            }
        }
    }
}



    async fn get_link(short_link: String) -> GetOriginalLinkResponse {
    let client = reqwest::Client::new();
    
    match client
        .get(format!("{}{}", GET_LINK_API, short_link))
        .send()
        .await {
            Ok(response) => {
                match response.json::<GetOriginalLinkResponse>().await {
                    Ok(original_link) => {
                        return GetOriginalLinkResponse {
                            status: original_link.status,
                            original_link: original_link.original_link
                        };
                    },
                    Err(_) => {
                        return GetOriginalLinkResponse {
                            status: false,
                            original_link: "error".to_string()
                    }
                }
            }
            },
        Err(_) => {
            return GetOriginalLinkResponse {
                status: false,
                original_link: "error".to_string()
            }
        }
    }
}