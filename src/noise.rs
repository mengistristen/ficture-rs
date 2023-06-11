//! This module provides noise generators for generating 
//! world maps.
//!
//! This module provides the following generators:
//! - [`SimplexNoiseGenerator`]
//!
//! # Examples
//!
//! ```
//! use ficture_generator::cell::Cell;
//! use ficture_generator::map::{Map, MapMonad};
//! use ficture_generator::noise::{NoiseGeneratorBuilder, SimpleNoiseGenerator, SimplexNoiseGeneratorBuilder};
//!
//! let noise_generator = SimplexNoiseGeneratorBuilder::new(10, 10)
//!     .octaves(6)
//!     .persistence(2.0)
//!     .lacunarity(3.0)
//!     .build();
//! let map = Map::return_single(Cell { elevation: 0.0, moisture: 0.0 }, 10, 10);
//! let map = map.and_then_with_coordinates(|cell, x, y| {
//!     Cell {
//!         elevation: noise_generator.generate(x, y),
//!         moisture: cell.moisture
//!     }
//! });
//! ```
use noise::{Simplex, NoiseFn};

/// A trait describing a generator that generates a single point
/// in a world map given only information about it's location
/// in 2D space. A generator of this type does not know any
/// other context about the map.
pub trait SimpleNoiseGenerator {
    fn generate(&self, x: usize, y: usize) -> f64;
}

/// A noise generator that uses simplex noise to generate
/// values. This will wrap values around the world map on
/// the east-west axis.
pub struct SimplexNoiseGenerator {
    height: usize,
    octaves: usize,
    frequencies: Vec<f64>,
    amplitudes: Vec<f64>,
    circle_coords: Vec<(f64, f64)>,
    noise: Simplex,
}

impl SimplexNoiseGenerator {
    /// Creates a [`SimplexNoiseGenerator`]. Pre-calculates the noise frequencies and
    /// amplitudes as well as the coordinates to use for wrapping the map along the
    /// east-west axis.
    fn new(width: usize, height: usize, octaves: usize, persistence: f64, lacunarity: f64) -> Self {
        let mut amplitude = 1.0;
        let mut frequencies = vec![];
        let mut amplitudes = vec![];
        let aspect_ratio = width as f64 / height as f64;

        // pre-calculate the frequencies and amplitudes to avoid
        // calculating them for every cell
        for octave in 0..octaves {
            frequencies.push(lacunarity.powf(octave as f64));
            amplitudes.push(amplitude);
            amplitude /= persistence;
        }

        // to allow the map to wrap around the east-west edges,
        // pre-calculate the values
        let circle_coords = (0..width)
            .map(|x| {
                let scale_x = x as f64 / width as f64;
                let angle = scale_x * 2.0 * std::f64::consts::PI;
                (angle.cos() / aspect_ratio, angle.sin() / aspect_ratio)
            })
        .collect();

        Self {
            height,
            octaves,
            frequencies,
            amplitudes,
            circle_coords,
            noise: Simplex::new(2),
        }
    }
}

impl SimpleNoiseGenerator for SimplexNoiseGenerator {
    /// Creates a noise values at the coordinates `x` and `y`.
    fn generate(&self, x: usize, y: usize) -> f64 {
        let mut elevation = 0.0;
        let scale_y = y as f64 / self.height as f64;

        let (circle_x, circle_z) = self.circle_coords[x];

        for octave in 0..self.octaves {
            let frequency = self.frequencies[octave];
            let amplitude = self.amplitudes[octave];
            let noise = self.noise.get([
                frequency * circle_x,
                frequency * scale_y,
                frequency * circle_z,
            ]);

            elevation += amplitude * noise;
        }

        elevation = elevation.powf(2.0);

        elevation
    }
}

pub trait NoiseGeneratorBuilder {
    fn new(width: usize, height: usize) -> Self;
    fn octaves(self, octaves: usize) -> Self;
    fn persistence(self, persistence: f64) -> Self;
    fn lacunarity(self, lacunarity: f64) -> Self;
    fn build(self) -> Box<dyn SimpleNoiseGenerator + Send + Sync>;
}

/// A builder for the [`SimplexNoiseGenerator`].
pub struct SimplexNoiseGeneratorBuilder {
    width: usize,
    height: usize,
    octaves: usize,
    persistence: f64,
    lacunarity: f64,
}

impl NoiseGeneratorBuilder for SimplexNoiseGeneratorBuilder {
    /// Creates the [`SimplexNoiseGeneratorBuilder`].
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            octaves: 6,
            persistence: 2.0,
            lacunarity: 3.0,
        }
    }

    /// Set the number of octaves to be used in heightmap
    /// generation.
    fn octaves(mut self, octaves: usize) -> Self {
        self.octaves = octaves;
        self
    }

    /// Set the rate by which the amplitude is divided as
    /// higher octaves are generated. A higher value means that
    /// higher octaves will have less of an effect on the overall
    /// amplitude than lower octaves.
    fn persistence(mut self, persistence: f64) -> Self {
        self.persistence = persistence;
        self
    }

    /// Sets the amount of detail that is added or removed
    /// at each octave. A higher value means that higher octaves
    /// will provide more detail than lower octaves.
    fn lacunarity(mut self, lacunarity: f64) -> Self {
        self.lacunarity = lacunarity;
        self
    }

    /// Construct the [`SimplexNoiseGenerator`] based on
    /// the defined attributes.
    fn build(self) -> Box<dyn SimpleNoiseGenerator + Send + Sync> {
        Box::new(
            SimplexNoiseGenerator::new(
                self.width,
                self.height,
                self.octaves,
                self.persistence,
                self.lacunarity,
            )
        )
    }
}
