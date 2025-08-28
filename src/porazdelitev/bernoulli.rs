use crate::porazdelitev::*;

pub struct Bernoulli {
    pub p: f64,
    pub q: f64,
}

impl Bernoulli {
    pub fn new(p: f64) -> Self {
        assert!(p >= 0.0 && p <= 1.0);
        Bernoulli { p, q: 1.0 - p }
    }
}

impl Porazdelitev for Bernoulli {
    fn cdf(&self, x: f64) -> f64 {
        if x < 0.0 {
            0.0
        } else if x < 1.0 {
            self.q
        } else {
            1.0
        }
    }
}

impl Diskretna for Bernoulli {
    fn pmf(&self, x: u64) -> f64 {
        match x {
            0 => self.q,
            1 => self.p,
            _ => 0.0,
        }
    }
}

impl PricakovanaVrednost for Bernoulli {
    fn e(&self) -> f64 {
        self.p
    }
}

impl Varianca for Bernoulli {
    fn var(&self) -> f64 {
        self.p * self.q
    }
}
