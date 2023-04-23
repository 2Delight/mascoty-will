pub mod emotion;
pub mod eyes;
pub mod lips;

use emotion::Emotion;
use eyes::Eyes;
use lips::Lips;
use yew::prelude::*;

use yew_hooks::prelude::use_interval;

#[derive(Debug, Default)]
pub struct Mascot {
    pub eyes: Eyes,
    pub lips: Lips,
    pub emotion: Emotion,
}

pub enum Message {
    GetRandom,
}

impl Mascot {
    pub fn get_random_mascot() -> Mascot {
        Mascot {
            eyes: rand::random(),
            lips: rand::random(),
            emotion: rand::random(),
        }
    }

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

#[function_component(MascotState)]
pub fn mascot_interval() -> Html {
    let mascot_state = use_state(|| Mascot::default());
    {
        let mascot_state = mascot_state.clone();
        use_interval(
            move || {
                mascot_state.set(Mascot::get_random_mascot());
            },
            500,
        );
    }

    html! {
        <div class="mascot">
            <div class="images">
                <img class="face" src={ mascot_state.get_face_path() } alt="Face" />
                <img class="eyes" src={ mascot_state.get_eyes_path() } alt="Eyes" />
                <img class="lips" src={ mascot_state.get_lips_path() } alt="Lips" />
            </div>
            <p>{ format!("Mascot: {:?}", mascot_state) }</p>
        </div>
    }
}
