use crate::porazdelitev::*;
use crate::utils;

pub struct Hipergeometrijska {
    pub s: u64,
    pub r: u64,
    pub n: u64,
}

impl Hipergeometrijska {
    pub fn new(s: u64, r: u64, n: u64) -> Self {
        assert!(s <= n && r <= n);
        Hipergeometrijska { s, r, n }
    }
}

impl Diskretna for Hipergeometrijska {
    fn pmf(&self, x: u64) -> f64 {
        ((utils::binomial(self.s, x) * utils::binomial(self.n - self.s, self.r - x)) as f64)
            / (utils::binomial(self.n, self.r) as f64)
    }
}

impl Porazdelitev for Hipergeometrijska {
    fn cdf(&self, x: f64) -> f64 {
        if x < 0.0 {
            return 0.0;
        }
        let k = x.floor() as u64;

        utils::sestej(|i: u64| <Hipergeometrijska as Diskretna>::pmf(self, i), k)
    }
}

impl PricakovanaVrednost for Hipergeometrijska {
    fn e(&self) -> f64 {
        ((self.r * self.s) as f64) / (self.n as f64)
    }
}

impl Varianca for Hipergeometrijska {
    fn var(&self) -> f64 {
        (((self.r * self.s) * (self.n - self.r) * (self.n - self.s)) as f64) / (self.n as f64)
    }
}
