use crate::porazdelitev::*;
use crate::utils;

pub struct Binomska {
    pub n: u64,
    pub p: f64,
    pub q: f64,
}

impl Binomska {
    pub fn new(n: u64, p: f64) -> Self {
        assert!(p >= 0.0 && p <= 1.0);

        Binomska { n, p, q: 1.0 - p }
    }
}

impl Diskretna for Binomska {
    fn pmf(&self, x: u64) -> f64 {
        let bin = utils::binomial(self.n, x) as f64;

        bin * self.p.powi(x as i32) * self.q.powi((self.n - x) as i32)
    }
}

impl Porazdelitev for Binomska {
    fn cdf(&self, x: f64) -> f64 {
        if x < 0.0 {
            return 0.0;
        }
        let k = x.floor() as u64;

        utils::sestej(|i: u64| <Binomska as Diskretna>::pmf(self, i), k)
    }
}

impl PricakovanaVrednost for Binomska {
    fn e(&self) -> f64 {
        (self.n as f64) * self.p
    }
}

impl Varianca for Binomska {
    fn var(&self) -> f64 {
        (self.n as f64) * self.p * self.q
    }
}
