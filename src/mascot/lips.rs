use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum Lips {
    #[default]
    Closed,
    Opened,
}

impl Distribution<Lips> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Lips {
        match rng.gen_range(0..=1) {
            0 => Lips::Closed,
            _ => Lips::Opened,
        }
    }
}
