use yew_router::prelude::*;
use yew::prelude::*;
use crate::components::{register::Register, redirect::Redirect};

#[derive(Routable, PartialEq, Clone, Debug)]
enum Route {
    #[at("/gt")]
    Redirect,
    #[at("/register")]
    Register,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Redirect => html! { <Redirect /> },
        Route::Register => html! { <Register /> }
    }
}