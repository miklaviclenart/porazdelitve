use crate::porazdelitev::*;
use crate::utils;
use std::f64::consts::{PI, SQRT_2};

pub struct Normalna {
    pub mu: f64,
    pub sigma2: f64,
}

impl Normalna {
    pub fn new(mu: f64, sigma2: f64) -> Self {
        assert!(sigma2 > 0.0);
        Normalna { mu, sigma2 }
    }
}

impl Porazdelitev for Normalna {
    fn cdf(&self, x: f64) -> f64 {
        let sigma = self.sigma2.sqrt();
        let transf = (x - self.mu) / (sigma * SQRT_2);

        0.5 * (1.0 + utils::erf(transf))
    }
}

impl Zvezna for Normalna {
    fn pdf(&self, x: f64) -> f64 {
        let coef = 1.0 / (2.0 * PI * self.sigma2).sqrt();
        let exponent = -((x - self.mu).powi(2)) / (2.0 * self.sigma2);
        coef * exponent.exp()
    }
}

impl PricakovanaVrednost for Normalna {
    fn e(&self) -> f64 {
        self.mu
    }
}

impl Varianca for Normalna {
    fn var(&self) -> f64 {
        self.sigma2
    }
}
