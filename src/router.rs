use yew::prelude::*;
use yew_router::prelude::*;

use crate::mascot::{MascotInterval, SadMascot};

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Main,
    #[at("/mascoty")]
    Mascoty,
    #[at("/downloads")]
    Downloads,
    #[not_found]
    #[at("/not-found")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Main => html! {
            <>
                <h1> {"Welcome to Mascoty application official website!"} </h1>
                <MascotInterval/>
            </>
        },
        Route::Mascoty => html! {
            <>
                <h1> {"Here you can check out our app functionality!"} </h1>
            </>
        },
        Route::Downloads => html! {
            <>
                <h1> {"You can see available downloads here!"} </h1>
            </>
        },
        Route::NotFound => html! {
            <>
                <h1> {"Page not found!"} </h1>
                <SadMascot/>
            </>
        },
    }
}
