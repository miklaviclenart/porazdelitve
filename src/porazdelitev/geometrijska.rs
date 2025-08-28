use crate::porazdelitev::*;

pub struct Geometrijska {
    pub p: f64,
    pub q: f64,
}

impl Geometrijska {
    pub fn new(p: f64) -> Self {
        assert!(p >= 0.0 && p <= 1.0);
        Geometrijska { p, q: 1.0 - p }
    }
}

impl Porazdelitev for Geometrijska {
    fn cdf(&self, x: f64) -> f64 {
        if x < 1.0 {
            return 0.0;
        }
        1.0 - self.q.powi(x.floor() as i32)
    }
}

impl Diskretna for Geometrijska {
    fn pmf(&self, x: u64) -> f64 {
        if x == 0 {
            return 0.0;
        }

        self.p * self.q.powi((x - 1) as i32)
    }
}

impl PricakovanaVrednost for Geometrijska {
    fn e(&self) -> f64 {
        1.0 / self.p
    }
}

impl Varianca for Geometrijska {
    fn var(&self) -> f64 {
        self.q / self.p.powi(2)
    }
}
