pub mod emotion;
pub mod eyes;
pub mod lips;

use emotion::Emotion;
use eyes::Eyes;
use lips::Lips;
use yew::prelude::*;

use yew_hooks::prelude::use_interval;

#[derive(Debug, Default, PartialEq, Properties)]
pub struct Mascot {
    pub eyes: Eyes,
    pub lips: Lips,
    pub emotion: Emotion,
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
pub fn get_mascot_html(mascot_state: &Mascot) -> Html {
    html! {
        <>
            <img class="face" src={ mascot_state.get_face_path() } alt="Face" />
            <img class="eyes" src={ mascot_state.get_eyes_path() } alt="Eyes" />
            <img class="lips" src={ mascot_state.get_lips_path() } alt="Lips" />
        </>
    }
}

#[function_component(SadMascot)]
pub fn mascot_interval() -> Html {
    let mascot = Mascot {
        emotion: Emotion::Sad,
        eyes: Eyes::Opened,
        lips: Lips::Closed,
    };

    html! {
        <div class="mascot">
            <MascotState emotion={ mascot.emotion } eyes={ mascot.eyes } lips={ mascot.lips }/>
        </div>
    }
}

#[function_component(MascotInterval)]
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
                <MascotState emotion={ mascot_state.emotion } eyes={ mascot_state.eyes } lips={ mascot_state.lips }/>
            </div>
            <p>{ format!("Mascot: {:?}", mascot_state) }</p>
        </div>
    }
}
