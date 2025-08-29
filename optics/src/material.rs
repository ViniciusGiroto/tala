use std::num::NonZeroU32;

use crate::ray::Wavelength;

pub type MaterialIndex = NonZeroU32;

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Formula {
    Constant {
        cte: f32,
    },
    Schott {
        a: [f32; 6],
    },
    Sellmeier1 {
        k: [f32; 3],
        l: [f32; 3],
    },
    Sellmeier2 {
        a: f32,
        b: [f32; 2],
        ref_lambda: [f32; 2],
    },
    Sellmeier3 {
        k: [f32; 4],
        l: [f32; 4],
    },
    Sellmeier4 {
        a: f32,
        k: [f32; 2],
        l: [f32; 2],
    },
    Sellmeier5 {
        k: [f32; 5],
        l: [f32; 5],
    },
    Herzberger {
        c: [f32; 6],
    },
    Conrady {
        n0: f32,
        a: f32,
        b: f32,
    },
    Handbook1 {
        a: f32,
        b: f32,
        c: f32,
        d: f32,
    },
    Handbook2 {
        a: f32,
        b: f32,
        c: f32,
        d: f32,
    },
    Extended {
        a: [f32; 8],
    },
}

impl Formula {
    fn compute(&self, wavelength: Wavelength) -> f32 {
        match self {
            Self::Constant { cte } => *cte,
            Self::Schott { a } => (a[0]
                + a[1] * wavelength.powi(2)
                + a[2] * wavelength.powi(-2)
                + a[3] * wavelength.powi(-4)
                + a[4] * wavelength.powi(-6)
                + a[5] * wavelength.powi(-8))
            .sqrt(),
            Self::Sellmeier1 { l, k } => {
                let lambda2 = wavelength * wavelength;

                ((0..=2)
                    .map(|i| k[i] * lambda2 / (lambda2 - l[i]))
                    .sum::<f32>()
                    + 1.0)
                    .sqrt()
            }
            Self::Sellmeier2 { a, b, ref_lambda } => {
                let lambda2 = wavelength * wavelength;
                (a + b[0] * lambda2 / (lambda2 - ref_lambda[0].powi(2))
                    + b[1] / (lambda2 - ref_lambda[1].powi(2)))
                .sqrt()
            }
            Self::Sellmeier3 { l, k } => {
                let lambda2 = wavelength * wavelength;

                ((0..=3)
                    .map(|i| k[i] * lambda2 / (lambda2 - l[i]))
                    .sum::<f32>()
                    + 1.0)
                    .sqrt()
            }
            Self::Sellmeier4 { a, k, l } => {
                let lambda2 = wavelength * wavelength;

                (a + (k[0] * lambda2 / (lambda2 - l[0])) + (k[1] * lambda2 / (lambda2 - l[1])))
                    .sqrt()
            }
            Self::Sellmeier5 { k, l } => {
                let lambda2 = wavelength * wavelength;

                ((0..=3)
                    .map(|i| k[i] * lambda2 / (lambda2 - l[i]))
                    .sum::<f32>()
                    + 1.0)
                    .sqrt()
            }
            Self::Herzberger { c } => {
                let lambda2 = wavelength * wavelength;
                let l = (lambda2 - 0.028).recip();

                c[0] + c[1] * l
                    + c[2] * l * l
                    + c[3] * lambda2
                    + c[4] * lambda2 * lambda2
                    + c[5] * lambda2 * lambda2 * lambda2
            }
            Self::Conrady { n0, a, b } => n0 + a / wavelength + b / wavelength.powf(3.5),
            Self::Handbook1 { a, b, c, d } => {
                (a + b / (wavelength * wavelength - c) - d * wavelength * wavelength).sqrt()
            }
            Self::Handbook2 { a, b, c, d } => (a
                + (b * wavelength * wavelength) / (wavelength * wavelength - c)
                - d * wavelength * wavelength)
                .sqrt(),
            Self::Extended { a } => (a[0]
                + a[1] * wavelength.powi(2)
                + a[2..]
                    .iter()
                    .enumerate()
                    .map(|(i, a)| a * wavelength.powi(-2 * (i as i32 + 1)))
                    .sum::<f32>())
            .sqrt(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Material {
    name: String,
    formula: Formula,
}

impl Material {
    pub const fn new(name: String, formula: Formula) -> Self {
        Material { name, formula }
    }

    pub fn refractive_index(&self, wavelength: Wavelength) -> f32 {
        self.formula.compute(wavelength)
    }

    pub const fn name(&self) -> &str {
        self.name.as_str()
    }
}
