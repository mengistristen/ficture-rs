//! This module provides a structure for loading information from config files.
use std::{collections::HashMap, fs::File};

use colorgrad::Color;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    color::{get_color_func, ColorEvaluator, ColorFunc},
    noise::{NoiseGeneratorBuilder, SimpleNoiseGenerator},
};

/// The error type returned from validation of the
/// config file.
#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("invalid file (couldn't open the file at {0})")]
    InvalidFilePath(String),
    #[error("invalid persistence (expected a value greater than 0, but found {0})")]
    InvalidPersistence(f64),
    #[error("invalid lacunarity (expected a value greater than 0, but found {0})")]
    InvalidLacunarity(f64),
    #[error("invalid elevation (expected a value greater than 0, but found {0})")]
    InvalidElevation(f64),
    #[error("invalid moisture (expected a value greater than 0, but found {0})")]
    InvalidMoisture(f64),
    #[error("invalid color (expected a valid html color, but found {0})")]
    InvalidColor(String),
    #[error("expected multiple elevation levels to be present, but found none")]
    MissingElevationLevels,
    #[error("expected multiple moisture levels to be present, but found none")]
    MissingMoistureLevels,
    #[error("expected multiple colors to be present, but found none")]
    MissingColors,
    #[error("failed to parse config file")]
    FailedToParse,
}

/// A Result type for [`ConfigError`].
pub type ConfigResult<T> = Result<T, ConfigError>;

/// The structure representing the configuration
/// of the program.
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// A mapping of strings to the gradient for
    /// a single biome.
    pub biomes: HashMap<String, SimpleBiome>,
    /// A mapping of strings to a set of noise generation
    /// parameters.
    pub noise_generators: HashMap<String, Noise>,
    /// A mapping of strings to a set of biomes.
    pub biome_maps: HashMap<String, Biomes>,
}

/// The config structure for a single biome gradient.
#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleBiome {
    pub gradient: Vec<String>,
}

/// The config structure for noise generation.
#[derive(Debug, Serialize, Deserialize)]
pub struct Noise {
    pub octaves: usize,
    pub persistence: f64,
    pub lacunarity: f64,
}

/// The config structure for a set of biome gradients.
/// These are sets of elevation levels which contain
/// moisture levels and a gradient.
#[derive(Debug, Serialize, Deserialize)]
pub struct Biomes {
    pub elevation_levels: Vec<ElevationLevel>,
}

///  The config structure for a single elevation level.
#[derive(Debug, Serialize, Deserialize)]
pub struct ElevationLevel {
    pub elevation: f64,
    pub moisture_levels: Vec<MoistureLevel>,
}

/// The config structure for a single moisture level.
#[derive(Debug, Serialize, Deserialize)]
pub struct MoistureLevel {
    pub moisture: f64,
    pub gradient: Vec<String>,
}

impl Config {
    /// Validate the entire configuration.
    pub fn validate(&self) -> ConfigResult<()> {
        for pair in self.biomes.iter() {
            let (_, simple_biome) = pair;
            simple_biome.validate()?;
        }
        for pair in self.noise_generators.iter() {
            let (_, noise_generator) = pair;
            noise_generator.validate()?;
        }
        for pair in self.biome_maps.iter() {
            let (_, biome) = pair;
            biome.validate()?;
        }
        Ok(())
    }

    /// Loads the configuration for a file.
    pub fn from_file(filename: impl AsRef<str>) -> ConfigResult<Self> {
        let file = File::open(filename.as_ref())
            .map_err(|_| ConfigError::InvalidFilePath(String::from(filename.as_ref())))?;
        let config: Self = serde_yaml::from_reader(file).map_err(|_| ConfigError::FailedToParse)?;

        Ok(config)
    }

    /// Returns the associated noise generator for a given [`Noise`].
    ///
    /// Type parameters:
    /// - B - the noise generator builder type to use to
    ///     construct the noise generator.
    pub fn get_noise_generator<B: NoiseGeneratorBuilder>(
        &self,
        name: impl AsRef<str>,
        width: usize,
        height: usize,
    ) -> Option<Box<dyn SimpleNoiseGenerator + Send + Sync>> {
        if let Some(noise) = self.noise_generators.get(name.as_ref()) {
            Some(
                B::new(width, height)
                    .octaves(noise.octaves)
                    .persistence(noise.persistence)
                    .lacunarity(noise.lacunarity)
                    .build(),
            )
        } else {
            None
        }
    }

    /// Returns a color evaluator for a given set of biome mappings.
    pub fn get_color_evaluator(&self, name: impl AsRef<str>) -> Option<ColorEvaluator> {
        if let Some(biomes) = self.biome_maps.get(name.as_ref()) {
            if let Ok(evaluator) = ColorEvaluator::from_biomes(biomes) {
                Some(evaluator)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Returns a color function for a given biome.
    pub fn get_color_func(&self, name: impl AsRef<str>) -> Option<ColorFunc> {
        if let Some(biome) = self.biomes.get(name.as_ref()) {
            if let Ok(color_func) = get_color_func(&biome.gradient) {
                Some(color_func)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl SimpleBiome {
    /// Validate the parameters for a single biome.
    fn validate(&self) -> ConfigResult<()> {
        if self.gradient.is_empty() {
            return Err(ConfigError::MissingColors);
        }
        for color in &self.gradient {
            Color::from_html(color).map_err(|_| ConfigError::InvalidColor(color.to_string()))?;
        }
        Ok(())
    }
}

impl Noise {
    /// Validate the noise generation config items.
    fn validate(&self) -> ConfigResult<()> {
        if self.persistence <= 0.0 {
            return Err(ConfigError::InvalidPersistence(self.persistence));
        }
        if self.lacunarity <= 0.0 {
            return Err(ConfigError::InvalidLacunarity(self.lacunarity));
        }
        Ok(())
    }
}

impl Biomes {
    /// Validate the biomes.
    fn validate(&self) -> ConfigResult<()> {
        if self.elevation_levels.is_empty() {
            return Err(ConfigError::MissingElevationLevels);
        } else {
            for elevation_level in &self.elevation_levels {
                elevation_level.validate()?;
            }
        }
        Ok(())
    }

    /// Gets the total elevation in the biome mapping.
    pub(crate) fn total_elevation(&self) -> f64 {
        self.elevation_levels
            .iter()
            .fold(0.0, |acc, level| acc + level.elevation)
    }
}

impl ElevationLevel {
    /// Validate the elevation level.
    fn validate(&self) -> ConfigResult<()> {
        if self.elevation <= 0.0 {
            return Err(ConfigError::InvalidElevation(self.elevation));
        }
        if self.moisture_levels.is_empty() {
            return Err(ConfigError::MissingMoistureLevels);
        }
        for moisture_level in &self.moisture_levels {
            moisture_level.validate()?;
        }
        Ok(())
    }

    /// Gets the total moisture in the elevation level.
    pub(crate) fn total_moisture(&self) -> f64 {
        self.moisture_levels
            .iter()
            .fold(0.0, |acc, level| acc + level.moisture)
    }
}

impl MoistureLevel {
    /// Validate the moisture level.
    fn validate(&self) -> ConfigResult<()> {
        if self.moisture <= 0.0 {
            return Err(ConfigError::InvalidMoisture(self.moisture));
        }
        if self.gradient.is_empty() {
            return Err(ConfigError::MissingColors);
        }
        for color in &self.gradient {
            Color::from_html(color).map_err(|_| ConfigError::InvalidColor(color.to_string()))?;
        }
        Ok(())
    }
}
