//! This module provides functions for getting the color at
//! certain points in a gradient. The gradients represent
//! the colors used for different biomes on the map and
//! the final colors are chosen based on elevation.
use std::sync::{Arc, Mutex};

use colorgrad::{Color, CustomGradient, Gradient};
use image::Rgb;
use thiserror::Error;

use crate::{config::Biomes, utils::normalize};

/// The error type for color errors. 
#[derive(Error, Debug)]
pub enum ColorError {
    #[error("invalid gradient, could not parse or build gradient")]
    InvalidGradient
}

/// A result type for [`ColorError`].
pub type ColorResult<T> = Result<T, ColorError>;

/// Gets the value at `x` in a gradient and converts it
/// into an RGB value.
fn gradient_to_rgb(gradient: &Gradient, x: f64) -> Rgb<u8> {
    let color = gradient.at(x);

    Rgb([
        (color.r * 255.0) as u8,
        (color.g * 255.0) as u8,
        (color.b * 255.0) as u8,
    ])
}

/// A type for a function that can get a color from a gradient.
pub(crate) type ColorFunc = Arc<Mutex<dyn Fn(f64) -> Rgb<u8> + Send + Sync>>;

/// Converts a gradient into a function that can get a color from that gradient.
pub(crate) fn get_color_func(gradient: &Vec<String>) -> ColorResult<ColorFunc> {
    let mut colors: Vec<Color> = vec![];

    for color in gradient {
        colors.push(Color::from_html(color).map_err(|_| ColorError::InvalidGradient)?);
    }

    let gradient = CustomGradient::new()
        .colors(&colors)
        .build()
        .expect("failed to build gradient");

    Ok(Arc::new(Mutex::new(move |x| gradient_to_rgb(&gradient, x))))
}

/// A structure containing information for a single
/// elevation range, or the x axis of a biome map.
struct ElevationRange {
    /// The value of the primary factor for the evaluator.
    elevation: f64,
    /// The moisture gradients associated with this range.
    moisture_gradients: Vec<MoistureGradient> 
}

/// A structure containing information for a single 
/// moisture range, or the y axis of a biome map.
struct MoistureGradient {
    /// The value of the secondary factor for the evaluator.
    moisture: f64,
    /// A function pointer for getting the color in this gradient.
    get_color: ColorFunc 
}

/// A structure for evaluating colors from biome maps. The primary 
/// example of this structure's usage is in getting colors based on
/// a cell's elevation and moisture levels. Despite using the terms
/// "elevation" and "moisture", this can be used with any two 
/// factors to get a color.
pub struct ColorEvaluator {
    /// The ranges for the "elevation" factor of the
    /// color evaluator.
    elevation_ranges: Vec<ElevationRange>
}

impl ColorEvaluator {
    /// Creates a [`ColorEvaluator`] from a biome map loaded from
    /// a config file.
    pub(crate) fn from_biomes(biomes: &Biomes) -> ColorResult<Self> {
        let total_elevation = biomes.total_elevation();
        let mut elevation_ranges: Vec<ElevationRange> = vec![];
        let mut cumulative_elevation = 0.0;

        for elevation_level in &biomes.elevation_levels {
            let total_moisture = elevation_level.total_moisture();
            let mut moisture_gradients: Vec<MoistureGradient> = vec![];
            let mut cumulative_moisture = 0.0;

            for moisture_level in &elevation_level.moisture_levels {
                cumulative_moisture += moisture_level.moisture;
                moisture_gradients.push(MoistureGradient { 
                    moisture: cumulative_moisture / total_moisture, 
                    get_color: get_color_func(&moisture_level.gradient)? 
                });
            }

            cumulative_elevation += elevation_level.elevation;
            elevation_ranges.push(ElevationRange { 
                elevation: cumulative_elevation / total_elevation, 
                moisture_gradients 
            });
        }

        Ok(
            Self {
                elevation_ranges
            }
        )
    }

    /// Gets a color from a biome map based on two factors. These
    /// are called "elevation" and "moisture" for simplicity. In 
    /// reality, these arguments can be used to describe many other
    /// factor of map generation. For example, elevation and moisture
    /// may instead represent temperature and moisture instead in
    /// a particular map.
    pub fn evaluate(&self, elevation: f64, moisture: f64) -> Rgb<u8> {
        let mut final_color = Rgb([0, 0, 0]);
        let mut cumulative_elevation = 0.0;

        for elevation_range in &self.elevation_ranges {
            if elevation <= elevation_range.elevation {
                for moisture_gradient in &elevation_range.moisture_gradients {
                    if moisture <= moisture_gradient.moisture {
                        let normalized_elevation = normalize(elevation, 
                            cumulative_elevation, 
                            cumulative_elevation + elevation_range.elevation);
                        let get_color = &moisture_gradient.get_color.lock().expect("failed to acquire lock");

                        final_color = get_color(normalized_elevation);
                        break;
                    }
                }
                break;
            }
            cumulative_elevation += elevation_range.elevation;
        }

        final_color
    } 
}
