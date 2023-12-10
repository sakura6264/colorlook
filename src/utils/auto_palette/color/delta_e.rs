use super::super::lab::Lab;
use super::super::math::number::Float;
use super::super::white_point::WhitePoint;

/// Enum representing the Delta E formula.
///
/// # Examples
/// ```
/// use auto_palette::delta_e::DeltaE;
/// use auto_palette::lab::Lab;
/// use auto_palette::white_point::D65;
///
/// let lab1 = Lab::<_, D65>::new(0.0, 0.0, 0.0);
/// let lab2 = Lab::<_, D65>::new(1.0, 1.0, 1.0);
/// let delta_e = DeltaE::CIE2000.measure(&lab1, &lab2);
/// assert!(delta_e > 0.0);
/// ```
///
/// # References
/// * [Delta E 101](http://zschuessler.github.io/DeltaE/learn/)
/// * [Color difference](https://en.wikipedia.org/wiki/Color_difference)
#[derive(Debug)]
pub enum DeltaE {
    /// The CIE76 formula.
    #[allow(unused)]
    CIE76,
    /// The CIE94 formula.
    #[allow(unused)]
    CIE94,
    /// The CIEDE2000 formula.
    #[allow(unused)]
    CIE2000,
}

impl DeltaE {
    /// Measures the distance between two colors.
    ///
    /// # Type Parameters
    /// * `F` - The floating point type.
    /// * `WP` - The white point.
    ///
    /// # Arguments
    /// * `lab1` - The 1st color in CIE L*a*b* color space.
    /// * `lab2` - The 2nd color in CIE L*a*b* color space.
    ///
    /// # Returns
    /// The distance between the two colors.
    #[allow(unused)]
    pub fn measure<F, WP>(&self, lab1: &Lab<F, WP>, lab2: &Lab<F, WP>) -> F
    where
        F: Float,
        WP: WhitePoint<F>,
    {
        match *self {
            DeltaE::CIE76 => cie76(lab1, lab2),
            DeltaE::CIE94 => cie94(
                lab1,
                lab2,
                F::from_f64(1.0),
                F::from_f64(0.045),
                F::from_f64(0.015),
            ),
            DeltaE::CIE2000 => ciede2000(lab1, lab2),
        }
    }
}

#[allow(unused)]
fn cie76<F, WP>(lab1: &Lab<F, WP>, lab2: &Lab<F, WP>) -> F
where
    F: Float,
    WP: WhitePoint<F>,
{
    let delta_l = lab1.l - lab2.l;
    let delta_a = lab1.a - lab2.a;
    let delta_b = lab1.b - lab2.b;
    (delta_l.powi(2) + delta_a.powi(2) + delta_b.powi(2)).sqrt()
}

#[allow(unused)]
fn cie94<F, WP>(lab1: &Lab<F, WP>, lab2: &Lab<F, WP>, k_l: F, k1: F, k2: F) -> F
where
    F: Float,
    WP: WhitePoint<F>,
{
    let delta_l = lab1.l - lab2.l;
    let delta_a = lab1.a - lab2.a;
    let delta_b = lab1.b - lab2.b;

    let c1 = (lab1.a.powi(2) + lab1.b.powi(2)).sqrt();
    let c2 = (lab2.a.powi(2) + lab2.b.powi(2)).sqrt();
    let delta_c = c1 - c2;
    let delta_h = (delta_a.powi(2) + delta_b.powi(2) - delta_c.powi(2)).sqrt();

    let s_l = F::one();
    let s_c = F::one() + k1 * c1;
    let s_h = F::one() + k2 * c1;
    (delta_l / (k_l * s_l)).powi(2) + (delta_c / s_c).powi(2) + (delta_h / s_h).powi(2)
}

#[allow(unused)]
fn ciede2000<F, WP>(lab1: &Lab<F, WP>, lab2: &Lab<F, WP>) -> F
where
    F: Float,
    WP: WhitePoint<F>,
{
    let l_bar = (lab1.l + lab2.l) / F::from_f64(2.0);
    let delta_l_prime = lab2.l - lab1.l;

    let c1 = (lab1.a.powi(2) + lab1.b.powi(2)).sqrt();
    let c2 = (lab2.a.powi(2) + lab2.b.powi(2)).sqrt();
    let c_bar = (c1 + c2) / F::from_f64(2.0);

    let g = (c_bar.powi(7) / (c_bar.powi(7) + F::from_u32(25).powi(7))).sqrt();
    let a1_prime = lab1.a + (lab1.a / F::from_f64(2.0)) * (F::one() - g);
    let a2_prime = lab2.a + (lab2.a / F::from_f64(2.0)) * (F::one() - g);

    let c1_prime = (a1_prime.powi(2) + lab1.b.powi(2)).sqrt();
    let c2_prime = (a2_prime.powi(2) + lab2.b.powi(2)).sqrt();
    let c_bar_prime = (c1_prime + c2_prime) / F::from_f64(2.0);
    let delta_c_prime = c2_prime - c1_prime;

    let h_prime = |x: F, y: F| {
        if x.is_zero() && y.is_zero() {
            return F::zero();
        }

        let mut angle = y.atan2(x).to_degrees();
        if angle < F::zero() {
            angle += F::from_f64(360.0);
        }
        angle
    };

    let h1_prime = h_prime(a1_prime, lab1.b);
    let h2_prime = h_prime(a2_prime, lab2.b);

    let delta_h_prime = if c1_prime.is_zero() || c2_prime.is_zero() {
        F::zero()
    } else {
        let delta = h2_prime - h1_prime;
        if delta.abs() <= F::from_f64(180.0) {
            delta
        } else if h2_prime <= h1_prime {
            delta + F::from_f64(360.0)
        } else {
            delta - F::from_f64(360.0)
        }
    };
    #[allow(non_snake_case)]
    let delta_H_prime = F::from_f64(2.0)
        * (c1_prime * c2_prime).sqrt()
        * (delta_h_prime.to_radians() / F::from_f64(2.0)).sin();

    let h_bar_prime = if (h1_prime - h2_prime).abs() > F::from_f64(180.0) {
        (h1_prime + h2_prime + F::from_f64(360.0)) / F::from_f64(2.0)
    } else {
        (h1_prime + h2_prime) / F::from_f64(2.0)
    };

    let t = F::one() - F::from_f64(0.17) * (h_bar_prime - F::from_f64(30.0)).to_radians().cos()
        + F::from_f64(0.24) * (F::from_f64(2.0) * h_bar_prime).to_radians().cos()
        + F::from_f64(0.32)
            * (F::from_f64(3.0) * h_bar_prime + F::from_f64(6.0))
                .to_radians()
                .cos()
        - F::from_f64(0.20)
            * (F::from_f64(4.0) * h_bar_prime - F::from_f64(63.0))
                .to_radians()
                .cos();

    let s_l = F::one()
        + F::from_f64(0.015) * (l_bar - F::from_f64(50.0)).powi(2)
            / (F::from_f64(20.0) + (l_bar - F::from_f64(50.0)).powi(2)).sqrt();
    let s_c = F::one() + F::from_f64(0.045) * c_bar_prime;
    let s_h = F::one() + F::from_f64(0.015) * c_bar_prime * t;

    let r_t = F::from_f64(-2.0)
        * (c_bar_prime.powi(7) / (c_bar_prime.powi(7) + F::from_u32(25).powi(7))).sqrt()
        * (F::from_f64(60.0)
            * (-((h_bar_prime - F::from_f64(275.0)) / F::from_f64(25.0)).powi(2)).exp())
        .to_radians()
        .sin();

    let l = delta_l_prime / (F::from_f64(1.0) * s_l);
    let c = delta_c_prime / (F::from_f64(1.0) * s_c);
    let h = delta_H_prime / (F::from_f64(1.0) * s_h);
    (l * l + c * c + h * h + r_t * c * h).sqrt()
}