use crate::porazdelitev::*;

pub struct EnakomernaInterval {
    pub a: f64,
    pub b: f64,
}

impl EnakomernaInterval {
    pub fn new(a: f64, b: f64) -> Self {
        assert!(a < b);
        EnakomernaInterval { a, b }
    }
}

impl Porazdelitev for EnakomernaInterval {
    fn cdf(&self, x: f64) -> f64 {
        if x < self.a {
            return 0.0;
        } else if x > self.b {
            return 1.0;
        }

        (x - self.a) / (self.a - self.b)
    }
}

impl Zvezna for EnakomernaInterval {
    fn pdf(&self, x: f64) -> f64 {
        if (x < self.a) | (x > self.b) {
            return 0.0;
        }

        1.0 / (self.b - self.a)
    }
}

impl PricakovanaVrednost for EnakomernaInterval {
    fn e(&self) -> f64 {
        0.5 * (self.a + self.b)
    }
}

impl Varianca for EnakomernaInterval {
    fn var(&self) -> f64 {
        (self.b - self.a).powi(2) / 12.0
    }
}
