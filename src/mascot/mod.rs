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

impl Mascot {
    fn get_face_path(&self) -> String {
        format!("assets/images/raw-mascot/{:?}/face.png", self.emotion,).to_lowercase()
    }

    fn get_eyes_path(&self) -> String {
        format!(
            "assets/images/raw-mascot/{:?}/eyes/{:?}.png",
            self.emotion, self.eyes,
        )
        .to_lowercase()
    }

    fn get_lips_path(&self) -> String {
        format!(
            "assets/images/raw-mascot/{:?}/lips/{:?}.png",
            self.emotion, self.lips,
        )
        .to_lowercase()
    }
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
            <div class="mascot">
                <div class="images">
                    <img class="face" src={ self.get_face_path() } alt="Face" />
                    <img class="eyes" src={ self.get_eyes_path() } alt="Eyes" />
                    <img class="lips" src={ self.get_lips_path() } alt="Lips" />
                    <p>{ format!("Mascot: {:?}", self) }</p>
                </div>
                <button onclick={ ctx.link().callback(|_| Message::GetRandom) }>{ "get new mascot!" }</button>
            </div>
        }
    }
}
