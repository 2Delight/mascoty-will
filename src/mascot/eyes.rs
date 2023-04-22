use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Debug, Default)]
pub enum Eyes {
    Closed,
    #[default]
    Opened,
}

impl Distribution<Eyes> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Eyes {
        match rng.gen_range(0..=1) {
            0 => Eyes::Closed,
            _ => Eyes::Opened,
        }
    }
}
