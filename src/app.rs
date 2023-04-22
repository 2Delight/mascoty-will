use yew::prelude::*;

use crate::mascot::{Mascot};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <h1> {"Welcome to Mascoty application official website!"} </h1>
            <Mascot/>
        </main>
    }
}
