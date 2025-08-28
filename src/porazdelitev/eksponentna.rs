use crate::porazdelitev::*;

pub struct Eksponentna {
    pub lambda: f64,
}

impl Eksponentna {
    pub fn new(lambda: f64) -> Self {
        assert!(lambda > 0.0);
        Eksponentna { lambda }
    }
}

impl Porazdelitev for Eksponentna {
    fn cdf(&self, x: f64) -> f64 {
        if x < 0.0 {
            return 0.0;
        }

        1.0 - (-self.lambda * x).exp()
    }
}

impl Zvezna for Eksponentna {
    fn pdf(&self, x: f64) -> f64 {
        if x < 0.0 {
            return 0.0;
        }

        self.lambda * (-self.lambda * x).exp()
    }
}

impl PricakovanaVrednost for Eksponentna {
    fn e(&self) -> f64 {
        1.0 / self.lambda
    }
}

impl Varianca for Eksponentna {
    fn var(&self) -> f64 {
        1.0 / self.lambda.powi(2)
    }
}
