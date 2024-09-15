use itertools_num::linspace;

// visible light wave length range
const VISIBLE_LOWER: f64 = 380.0; // [nm]
const VISIBLE_UPPER: f64 = 780.0; // [nm]
const WAVE_LENGTH_STEP: f64 = 1.0; // [nm]
const NANO: f64 = 1.0e-9;

pub struct Spectrum {}

impl Spectrum {
    pub fn to_rgb(radiance: &dyn Fn(f64) -> f64) -> (f64, f64, f64) {
        // integrate to convert spectrum to xyz color space
        // x = ∫ radiance(λ) * x_color_function(λ) dλ
        let steps = ((VISIBLE_UPPER - VISIBLE_LOWER) / WAVE_LENGTH_STEP) as usize;
        // integration range
        let wavelength = linspace(VISIBLE_LOWER, VISIBLE_UPPER, steps);
        // integration step
        let dl = WAVE_LENGTH_STEP * NANO;

        // integrate
        let x = wavelength
            .clone()
            .map(|l| radiance(l * NANO) * ColorFunction::x(l) * dl)
            .sum::<f64>();
        let y = wavelength
            .clone()
            .map(|l| radiance(l * NANO) * ColorFunction::y(l) * dl)
            .sum::<f64>();

        let z = wavelength
            .clone()
            .map(|l| radiance(l * NANO) * ColorFunction::z(l) * dl)
            .sum::<f64>();

        Self::xyz_to_rgb(x, y, z)
    }

    fn xyz_to_rgb(x: f64, y: f64, z: f64) -> (f64, f64, f64) {
        let m = [
            [0.41847, -0.15866, -0.082835],
            [-0.091169, 0.25243, 0.015708],
            [0.00092090, -0.0025498, 0.17860],
        ];
        let r = m[0][0] * x + m[0][1] * y + m[0][2] * z;
        let g = m[1][0] * x + m[1][1] * y + m[1][2] * z;
        let b = m[2][0] * x + m[2][1] * y + m[2][2] * z;
        (r, g, b)
    }
}

pub struct ColorFunction {}

impl ColorFunction {
    // CIE 1931 color matching functions
    // ref: https://ja.wikipedia.org/wiki/CIE_1931_%E8%89%B2%E7%A9%BA%E9%96%93
    pub fn x(wavelength: f64) -> f64 {
        let l = wavelength;
        let g = Self::segmented_gaussian;
        [
            1.056 * g(l, 599.8, 37.9, 31.0),
            0.362 * g(l, 442.0, 16.0, 26.7),
            -0.065 * g(l, 501.1, 20.4, 26.2),
        ]
        .iter()
        .sum()
    }

    pub fn y(wavelength: f64) -> f64 {
        let l = wavelength;
        let g = Self::segmented_gaussian;
        [
            0.821 * g(l, 568.8, 46.9, 40.5),
            0.286 * g(l, 530.9, 16.3, 31.1),
        ]
        .iter()
        .sum()
    }

    pub fn z(wavelength: f64) -> f64 {
        let l = wavelength;
        let g = Self::segmented_gaussian;
        [
            1.217 * g(l, 437.0, 11.8, 36.0),
            0.681 * g(l, 459.0, 26.0, 13.8),
        ]
        .iter()
        .sum()
    }

    fn segmented_gaussian(x: f64, mu: f64, sigma1: f64, sigma2: f64) -> f64 {
        if x < mu {
            (-(x - mu).powf(2.0) / (2.0 * sigma1.powf(2.0))).exp()
        } else {
            (-(x - mu).powf(2.0) / (2.0 * sigma2.powf(2.0))).exp()
        }
    }
}
