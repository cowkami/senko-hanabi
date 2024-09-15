//! # Black Body
//!
//! `black-body` is a collection of utilities to visualize black body radiation.
//!
//! ## Usage
//!
//! Add `black-body` to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! black-body = "0.1"
//! ```
//!
//! ## Example
//!
//! ```rust
//! use black_body::BlackBody;
//!
//! let body = BlackBody::new(5800.0);
//! println!("body temperature: {:?}", body.temperature);
//! println!("body radiance for wave length: {:?}", body.radiance(1.0e+3));
//! println!("body color: {:?}", body.calc_color());
//! ```
use itertools_num::linspace;
use std::f64::consts::E;

// physical constants
const C: f64 = 2.99792458e8; // [m/s] speed of light
const H: f64 = 6.62607015e-34; // [J/Hz] Planck constant
const K: f64 = 1.380649e-23; // [J/K] Boltzmann constant

// visible light wave length range
const VISIBLE_LOWER: f64 = 380.0; // [nm]
const VISIBLE_UPPER: f64 = 780.0; // [nm]
const WAVE_LENGTH_STEP: f64 = 1.0; // [nm]

pub struct BlackBody {
    pub temperature: f64,
}

impl BlackBody {
    pub fn new(temperature: f64) -> Self {
        assert!(
            temperature >= 0.0,
            "it requires; temperature >= 0\n\
            temperature must be grater than or equal to 0,\n\
            but got {temperature}"
        );
        Self { temperature }
    }

    pub fn radiance(&self, wavelength: f64) -> f64 {
        let l = wavelength;
        let t = self.temperature;
        let first = 2.0 * H * C.powf(2.0) / l.powf(5.0);
        let second = 1.0 / (E.powf((H * C) / (l * K * t)) - 1.0);
        return first * second;
    }

    pub fn color_for_eye(&self) -> [f64; 3] {
        let wavelength = linspace(
            VISIBLE_LOWER,
            VISIBLE_UPPER,
            ((VISIBLE_UPPER - VISIBLE_LOWER) / WAVE_LENGTH_STEP) as usize,
        );

        let y = wavelength
            .clone()
            .map(|l| self.radiance(l * 1.0e-9) * approx_color_y(l) * WAVE_LENGTH_STEP * 1.0e-9)
            .sum::<f64>();
        let x = wavelength
            .clone()
            .map(|l| self.radiance(l * 1.0e-9) * approx_color_x(l) * WAVE_LENGTH_STEP * 1.0e-9)
            .sum::<f64>();
        let z = wavelength
            .clone()
            .map(|l| self.radiance(l * 1.0e-9) * approx_color_z(l) * WAVE_LENGTH_STEP * 1.0e-9)
            .sum::<f64>();

        let (r, g, b) = xyz_to_rgb(x, y, z);
        let norm = (r + g + b) / 3.0;

        [r / norm, g / norm, b / norm]
    }
}

// CIE 1931 color matching functions
// ref: https://ja.wikipedia.org/wiki/CIE_1931_%E8%89%B2%E7%A9%BA%E9%96%93
pub fn approx_color_x(wavelength: f64) -> f64 {
    let l = wavelength;
    1.056 * segmented_gaussian(l, 599.8, 37.9, 31.0)
        + 0.362 * segmented_gaussian(l, 442.0, 16.0, 26.7)
        - 0.065 * segmented_gaussian(l, 501.1, 20.4, 26.2)
}

pub fn approx_color_y(wavelength: f64) -> f64 {
    let l = wavelength;
    0.821 * segmented_gaussian(l, 568.8, 46.9, 40.5)
        + 0.286 * segmented_gaussian(l, 530.9, 16.3, 31.1)
}

pub fn approx_color_z(wavelength: f64) -> f64 {
    let l = wavelength;
    1.217 * segmented_gaussian(l, 437.0, 11.8, 36.0)
        + 0.681 * segmented_gaussian(l, 459.0, 26.0, 13.8)
}

fn segmented_gaussian(x: f64, mu: f64, sigma1: f64, sigma2: f64) -> f64 {
    if x < mu {
        (-(x - mu).powf(2.0) / (2.0 * sigma1.powf(2.0))).exp()
    } else {
        (-(x - mu).powf(2.0) / (2.0 * sigma2.powf(2.0))).exp()
    }
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
