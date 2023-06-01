use crate::router::{switch, Route};
use yew::prelude::*;
use yew_router::{HashRouter, Switch};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <HashRouter>
                <Switch<Route> render={switch} />
            </HashRouter>
        </main>
    }
}
