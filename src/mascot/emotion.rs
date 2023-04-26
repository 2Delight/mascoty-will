use std::fmt::Debug;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

/// Represents 1 of 7 basic emotions supported by model.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum Emotion {
    Angry = 0,
    Disgust,
    Fear,
    Happy,
    #[default]
    Neutral,
    Sad,
    Surprise,
}

impl Emotion {
    /// Converts number into emotion.
    /// ```
    /// use mascoty_will::mascot::emotion::Emotion;
    ///
    /// let emotion = Emotion::from_num(0);
    /// assert!(emotion == Emotion::Angry);
    ///
    /// let emotion = Emotion::from_num(100);
    /// assert!(emotion == Emotion::Neutral);
    /// ```
    pub fn from_num(i: u8) -> Emotion {
        match i {
            0 => Emotion::Angry,
            1 => Emotion::Disgust,
            2 => Emotion::Fear,
            3 => Emotion::Happy,
            4 => Emotion::Neutral,
            5 => Emotion::Sad,
            6 => Emotion::Surprise,
            _ => Emotion::Neutral,
        }
    }

    /// Converts emotion into number.
    /// ```
    /// use mascoty_will::mascot::emotion::Emotion;
    ///
    /// let emotion = Emotion::Fear;
    /// assert!(emotion.to_num() == 2);
    /// ```
    pub fn to_num(&self) -> u8 {
        *self as u8
    }
}

impl Distribution<Emotion> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Emotion {
        match rng.gen_range(0..=6) {
            0 => Emotion::Angry,
            1 => Emotion::Disgust,
            2 => Emotion::Fear,
            3 => Emotion::Happy,
            4 => Emotion::Neutral,
            5 => Emotion::Sad,
            _ => Emotion::Surprise,
        }
    }
}
