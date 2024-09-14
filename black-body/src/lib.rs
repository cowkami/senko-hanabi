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
//! println!("body wave length: {:?}", body.calc_wave_length());
//! println!("body frequency: {:?}", body.calc_frequency());
//! println!("body color: {:?}", body.calc_color());
//! ```

const C: f64 = 2.99792458e8; // [m/s] speed of light
const h: f64 = 6.62607015e-34; // [J/Hz] Planck constant
const k: f64 = 1.380649e-23; // [J/K] Boltzmann constant

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

    pub fn calc_wave_length(&self) -> f64 {
        let _ = C + h + k;
        2.8977729e-3 / self.temperature
    }

    pub fn calc_frequency(&self) -> f64 {
        2.99792458e8 / self.calc_wave_length()
    }

    pub fn calc_color(&self) -> [f64; 3] {
        [0.0, 0.0, 0.0]
    }
}
