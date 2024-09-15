use yew::prelude::*;
use crate::components::{footer::Footer, form::Form, header::Header};

pub mod components {
    pub mod footer;
    pub mod form;
    pub mod header;
}


#[function_component(Main)]
pub fn app() -> Html {
    return html! {
        <div style = {format!("background-image: url('/static/leaves_background.png'); background-repeat: no-repeat;  background-size: cover; background-position: center;  margin-left: -8em; height: 110vh; width: 110vw;")}>
            <Header/>
            <Form/>
            <Footer/>
        </div>
    }
}


pub fn main() {
    yew::Renderer::<Main>::new().render();
}