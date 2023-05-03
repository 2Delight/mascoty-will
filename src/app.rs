use crate::router::{switch, Route};
use yew::prelude::*;
use yew_router::{BrowserRouter, Switch};

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
