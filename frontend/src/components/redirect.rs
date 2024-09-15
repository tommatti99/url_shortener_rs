use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use serde::Deserialize;
use web_sys::window;


const GET_LINK_API: &str = "http://127.0.0.1:8000/gt/";


#[derive(PartialEq, Properties)]
pub struct RedirectProps {
    short_link: String
}

#[function_component]
pub fn Redirect(props: &RedirectProps) -> Html {
    let original_link = use_state(|| String::new());
    let props_clone: String = props.short_link.clone();
    let original_link_effect = original_link.clone();
    
    {   spawn_local(async move { 
            let mut_original_link = original_link.clone();

            let link = get_link(&props_clone).await;

            mut_original_link.set(link.original_link);
        })
    };

        use_effect(move ||  {
            let window = window().unwrap();
            
            window.location().set_href(&original_link_effect).unwrap();
            || {}
        });
    

    html! {
        <div>{ "Redirecionando..." }</div>
    }
}


#[derive(Deserialize, Debug)]
pub struct GetOriginalLinkResponse {
    pub status: bool,
    pub original_link: String
}


async fn get_link(short_link: &String) -> GetOriginalLinkResponse {
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