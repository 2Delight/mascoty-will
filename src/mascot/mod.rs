pub mod emotion;
pub mod eyes;
pub mod lips;

use emotion::Emotion;
use eyes::Eyes;
use lips::Lips;
use yew::prelude::*;

#[derive(Debug, Default)]
pub struct Mascot {
    pub eyes: Eyes,
    pub lips: Lips,
    pub emotion: Emotion,
}

pub fn get_random_mascot() -> Mascot {
    Mascot {
        eyes: rand::random(),
        lips: rand::random(),
        emotion: rand::random(),
    }
}

pub enum Message {
    GetRandom,
}

impl Component for Mascot {
    type Message = Message;
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Mascot::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::GetRandom => {
                *self = get_random_mascot();
                true
            }
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <main>
                <p>{ format!("Mascot: {:?}", get_random_mascot()) }</p>
                <button onclick={ ctx.link().callback(|_| Message::GetRandom) }>{ "get new mascot!" }</button>
            </main>
        }
    }
}
