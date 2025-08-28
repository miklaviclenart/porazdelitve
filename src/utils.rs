// f64 je bolj priročna pri porazdelitvah
// fakulteta ful na ocaml stil
pub(crate) fn factorial(n: u64) -> f64 {
    (1..=n).fold(1.0, |acc, x| acc * x as f64)
}

pub(crate) fn binomial(n: u64, k: u64) -> u64 {
    if k > n {
        return 0;
    }

    // \binom{n}{k} = \binom{n}{n - k}
    let k = k.min(n - k);
    if k == 0 {
        return 1;
    }

    ((n - k + 1)..=n).product::<u64>() / (1..=k).product::<u64>()
}

pub(crate) fn sestej<F>(fun: F, k: u64) -> f64
where
    F: Fn(u64) -> f64,
{
    let mut sum = 0.0;
    for i in 0..=k {
        sum += fun(i);
    }
    sum
}

pub(crate) fn erf(x: f64) -> f64 {
    if x < 0.0 {
        return erfc_positive(-x) - 1.0;
    } else {
        return 1.0 - erfc_positive(x);
    }
}

// Dia, Yaya D., Approximate Incomplete Integrals,
//              Application to Complementary Error Function (June 21, 2023).
//              http://dx.doi.org/10.2139/ssrn.4487559
fn erfc_positive(x: f64) -> f64 {
    assert!(x >= 0.0);
    let x2 = x * x;

    let term1 = 0.56418958354775629 / (x + 2.06955023132914151);
    let term2 = (x2 + 2.71078540045147805 * x + 5.80755613130301624)
        / (x2 + 3.47954057099518960 * x + 12.06166887286239555);
    let term3 = (x2 + 3.47469513777439592 * x + 12.07402036406381411)
        / (x2 + 3.72068443960225092 * x + 8.44319781003968454);
    let term4 = (x2 + 4.00561509202259545 * x + 9.30596659485887898)
        / (x2 + 3.90225704029924078 * x + 6.36161630953880464);
    let term5 = (x2 + 5.16722705817812584 * x + 9.12661617673673262)
        / (x2 + 4.03296893109262491 * x + 5.13578530585681539);
    let term6 = (x2 + 5.95908795446633271 * x + 9.19435612886969243)
        / (x2 + 4.11240942957450885 * x + 4.48640329523408675);

    term1 * term2 * term3 * term4 * term5 * term6 * (-x2).exp()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1.); // 0! = 1
        assert_eq!(factorial(1), 1.);
        assert_eq!(factorial(5), 120.);
        assert_eq!(factorial(10), 3_628_800.);
    }

    #[test]
    fn test_binomial_basic() {
        assert_eq!(binomial(5, 0), 1); // n choose 0 = 1
        assert_eq!(binomial(5, 1), 5);
        assert_eq!(binomial(5, 2), 10);
        assert_eq!(binomial(5, 5), 1);
    }

    #[test]
    fn test_binomial_edge_cases() {
        assert_eq!(binomial(0, 0), 1);
        assert_eq!(binomial(5, 6), 0); // k > n
        assert_eq!(binomial(10, 0), 1);
        assert_eq!(binomial(10, 10), 1);
    }
}
