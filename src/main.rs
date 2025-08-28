use porazdelitve::*;

fn main() {
    let primer = Normalna::new(0., 1.);
    for i in 1..=10 {
        let x: f64 = (i as f64) / 10.0;
        let rez = primer.cdf(x);
        println!("Phi({x}) = {rez}")
    }
}
