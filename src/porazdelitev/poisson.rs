use crate::porazdelitev::*;
use crate::utils;

pub struct Poisson {
    pub lambda: f64,
}

impl Poisson {
    pub fn new(lambda: f64) -> Self {
        assert!(lambda >= 0.0);
        Poisson { lambda }
    }
}

impl Diskretna for Poisson {
    fn pmf(&self, x: u64) -> f64 {
        self.lambda.powi(x as i32) * (-self.lambda).exp() / utils::factorial(x)
    }
}

impl Porazdelitev for Poisson {
    fn cdf(&self, x: f64) -> f64 {
        if x < 0.0 {
            return 0.0;
        }
        let k = x.floor() as u64;

        utils::sestej(|i: u64| <Poisson as Diskretna>::pmf(self, i), k)
    }
}

impl PricakovanaVrednost for Poisson {
    fn e(&self) -> f64 {
        self.lambda
    }
}

impl Varianca for Poisson {
    fn var(&self) -> f64 {
        self.lambda
    }
}
