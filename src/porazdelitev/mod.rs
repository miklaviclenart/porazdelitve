pub trait Porazdelitev {
    // P(X <= x) (comulatice distribution function)
    fn cdf(&self, x: f64) -> f64;
}

pub trait Diskretna: Porazdelitev {
    // P(X = x) (probability mass function)
    fn pmf(&self, x: u64) -> f64;
}

pub trait Zvezna: Porazdelitev {
    // f_X (x) (probability density function)
    fn pdf(&self, x: f64) -> f64;
}

pub trait PricakovanaVrednost: Porazdelitev {
    fn e(&self) -> f64;
}

pub trait Varianca: Porazdelitev {
    fn var(&self) -> f64;
}

mod bernoulli;
mod binomska;
mod eksponentna;
mod enakomerna_interval;
mod geometrijska;
mod hipergeometrijska;
mod normalna;
mod poisson;

pub use self::bernoulli::Bernoulli;
pub use self::binomska::Binomska;
pub use self::eksponentna::Eksponentna;
pub use self::enakomerna_interval::EnakomernaInterval;
pub use self::geometrijska::Geometrijska;
pub use self::hipergeometrijska::Hipergeometrijska;
pub use self::normalna::Normalna;
pub use self::poisson::Poisson;
