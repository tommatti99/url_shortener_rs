use yew::prelude::*;
use serde::Deserialize;

const NEW_SHORT_LINK_API: &str = "http://127.0.0.1:8000/new_short_link/";

#[function_component]
pub fn Register() -> Html {

    let send_new_link

    return html! {
        <>
            <div>
                <input type="text" />
                <button/>
            </div>
        </>
    };
}



#[derive(Deserialize, Debug)]
pub struct NewShortLinkResponse {
    pub status: bool,
    pub short_link: String
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
