//! This module provides functions for getting the color at
//! certain points in a gradient. The gradients represent
//! the colors used for different biomes on the map and
//! the final colors are chosen based on elevation.
use colorgrad::{Color, CustomGradient, Gradient, ParseColorError};
use image::Rgb;

/// A structure containing all of the gradients
/// used for interpolating colors based on elevation
/// for cells.
struct BiomeGradients {
    ocean: Gradient,
    scorched: Gradient,
    bare: Gradient,
    tundra: Gradient,
    snow: Gradient,
    temperate_desert: Gradient,
    shrubland: Gradient,
    taiga: Gradient,
    grassland: Gradient,
    temperate_deciduous_forest: Gradient,
    temperate_rain_forest: Gradient,
    subtropical_desert: Gradient,
    tropical_seasonal_forest: Gradient,
    tropical_rain_forest: Gradient,
}

lazy_static! {
    static ref BIOME_GRADIENTS: BiomeGradients = {
        BiomeGradients {
            ocean: CustomGradient::new()
                .colors(&[
                    Color::from_html("#0a46ad").unwrap(),
                    Color::from_html("#35d6f2").unwrap(),
                ])
                .build()
                .unwrap(),
            scorched: CustomGradient::new()
                .colors(&[
                    Color::from_html("#3b3b3b").unwrap(),
                    Color::from_html("#6e6e6e").unwrap(),
                ])
                .build()
                .unwrap(),
            bare: CustomGradient::new()
                .colors(&[
                    Color::from_html("#7a7a7a").unwrap(),
                    Color::from_html("#a8a8a8").unwrap(),
                ])
                .build()
                .unwrap(),
            tundra: CustomGradient::new()
                .colors(&[
                    Color::from_html("#adad9c").unwrap(),
                    Color::from_html("#d1cfba").unwrap(),
                ])
                .build()
                .unwrap(),
            snow: CustomGradient::new()
                .colors(&[
                    Color::from_html("#ceced6").unwrap(),
                    Color::from_html("#e6e6f0").unwrap(),
                ])
                .build()
                .unwrap(),
            temperate_desert: CustomGradient::new()
                .colors(&[
                    Color::from_html("#bcc491").unwrap(),
                    Color::from_html("#d4dea2").unwrap(),
                ])
                .build()
                .unwrap(),
            shrubland: CustomGradient::new()
                .colors(&[
                    Color::from_html("#7d8c6d").unwrap(),
                    Color::from_html("#9aad86").unwrap(),
                ])
                .build()
                .unwrap(),
            taiga: CustomGradient::new()
                .colors(&[
                    Color::from_html("#8a996b").unwrap(),
                    Color::from_html("#a1b37d").unwrap(),
                ])
                .build()
                .unwrap(),
            grassland: CustomGradient::new()
                .colors(&[
                    Color::from_html("#81a150").unwrap(),
                    Color::from_html("#99bf5e").unwrap(),
                ])
                .build()
                .unwrap(),
            temperate_deciduous_forest: CustomGradient::new()
                .colors(&[
                    Color::from_html("#5e8751").unwrap(),
                    Color::from_html("#73a663").unwrap(),
                ])
                .build()
                .unwrap(),
            temperate_rain_forest: CustomGradient::new()
                .colors(&[
                    Color::from_html("#3b784b").unwrap(),
                    Color::from_html("#47915b").unwrap(),
                ])
                .build()
                .unwrap(),
            subtropical_desert: CustomGradient::new()
                .colors(&[
                    Color::from_html("#827356").unwrap(),
                    Color::from_html("#a6926c").unwrap(),
                ])
                .build()
                .unwrap(),
            tropical_seasonal_forest: CustomGradient::new()
                .colors(&[
                    Color::from_html("#4d873d").unwrap(),
                    Color::from_html("#5a9e47").unwrap(),
                ])
                .build()
                .unwrap(),
            tropical_rain_forest: CustomGradient::new()
                .colors(&[
                    Color::from_html("#2c694a").unwrap(),
                    Color::from_html("#348059").unwrap(),
                ])
                .build()
                .unwrap(),
        }
    };
}

/// A trait for getting the color associated
/// with an object.
pub trait GetColor {
    fn get_color(&self) -> Rgb<u8>;
}

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

/// Converts the color passed in into an RGB value.
pub fn color_to_rgb(color: impl AsRef<str>) -> Result<Rgb<u8>, ParseColorError> {
    let color = Color::from_html(color)?;

    Ok(Rgb([
        (color.r * 255.0) as u8,
        (color.g * 255.0) as u8,
        (color.b * 255.0) as u8,
    ]))
}

pub mod biomes {
    use image::Rgb;

    use super::{gradient_to_rgb, BIOME_GRADIENTS};

    pub fn ocean(x: f64) -> Rgb<u8> {
        gradient_to_rgb(&BIOME_GRADIENTS.ocean, x)
    }

    pub fn scorched(x: f64) -> Rgb<u8> {
        gradient_to_rgb(&BIOME_GRADIENTS.scorched, x)
    }

    pub fn bare(x: f64) -> Rgb<u8> {
        gradient_to_rgb(&BIOME_GRADIENTS.bare, x)
    }

    pub fn tundra(x: f64) -> Rgb<u8> {
        gradient_to_rgb(&BIOME_GRADIENTS.tundra, x)
    }

    pub fn snow(x: f64) -> Rgb<u8> {
        gradient_to_rgb(&BIOME_GRADIENTS.snow, x)
    }

    pub fn temperate_desert(x: f64) -> Rgb<u8> {
        gradient_to_rgb(&BIOME_GRADIENTS.temperate_desert, x)
    }

    pub fn shrubland(x: f64) -> Rgb<u8> {
        gradient_to_rgb(&BIOME_GRADIENTS.shrubland, x)
    }

    pub fn taiga(x: f64) -> Rgb<u8> {
        gradient_to_rgb(&BIOME_GRADIENTS.taiga, x)
    }

    pub fn grassland(x: f64) -> Rgb<u8> {
        gradient_to_rgb(&BIOME_GRADIENTS.grassland, x)
    }

    pub fn temperate_deciduous_forest(x: f64) -> Rgb<u8> {
        gradient_to_rgb(&BIOME_GRADIENTS.temperate_deciduous_forest, x)
    }

    pub fn temperate_rain_forest(x: f64) -> Rgb<u8> {
        gradient_to_rgb(&BIOME_GRADIENTS.temperate_rain_forest, x)
    }

    pub fn subtropical_desert(x: f64) -> Rgb<u8> {
        gradient_to_rgb(&BIOME_GRADIENTS.subtropical_desert, x)
    }

    pub fn tropical_seasonal_forest(x: f64) -> Rgb<u8> {
        gradient_to_rgb(&BIOME_GRADIENTS.tropical_seasonal_forest, x)
    }

    pub fn tropical_rain_forest(x: f64) -> Rgb<u8> {
        gradient_to_rgb(&BIOME_GRADIENTS.tropical_rain_forest, x)
    }
}
