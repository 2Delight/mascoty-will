use yew::prelude::*;
use yew_router::{BrowserRouter, Switch};
use crate::router::{Route, switch};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </main>
    }
}
