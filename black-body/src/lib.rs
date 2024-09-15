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
//! println!("body color: {:?}", body.color_for_eye());
//! ```
pub mod spectrum;
use spectrum::Spectrum;

// physical constants
const C: f64 = 2.99792458e8; // [m/s] speed of light
const H: f64 = 6.62607015e-34; // [J/Hz] Planck constant
const K: f64 = 1.380649e-23; // [J/K] Boltzmann constant

#[derive(Debug, Clone)]
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
        // plank's law
        // ref: https://en.wikipedia.org/wiki/Planck%27s_law
        let l = wavelength;
        let t = self.temperature;
        let first = 2.0 * H * C.powf(2.0) / l.powf(5.0);
        let second = 1.0 / (((H * C) / (l * K * t)).exp() - 1.0);
        return first * second;
    }

    pub fn color_for_eye(&self) -> [f64; 3] {
        let (r, g, b) = Spectrum::to_rgb(&|l| self.radiance(l));
        let mean = (r + g + b) / 3.0;
        [r / mean, g / mean, b / mean]
    }
}
