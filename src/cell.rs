//! This module provides a [`Cell`] representing a single point
//! on a 2D world map.
use image::Rgb;

use crate::color::{GetColor, 
    biomes::{ocean, subtropical_desert, grassland, tropical_seasonal_forest, 
        tropical_rain_forest, temperate_desert, temperate_deciduous_forest, temperate_rain_forest, 
        shrubland, taiga, scorched, bare, 
        tundra, snow}, 
    color_to_rgb};

/// A struct representing a single point on a world map.
#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    /// The elevation at a point on the map. Usually
    /// normalized from 0-1.
    pub elevation: f64,
    /// The moisture at a point on the map. Usually
    /// normalized from 0-1.
    pub moisture: f64,
}

impl GetColor for Cell {
    /// Uses the elevation and moisture to get a color
    /// for representing the biome for this cell.
    fn get_color(&self) -> Rgb<u8> {
        fn normalize(value: f64, min: f64, max: f64) -> f64 {
            (value - min) / (max - min)
        }

        let (elevation, moisture) = (self.elevation, self.moisture);
        let mut value = normalize(elevation, 0.0, 0.1);

        match (elevation, moisture) {
            (e, _) if e < 0.1 => ocean(value),
            (e, _) if e < 0.12 => color_to_rgb("#01c7dd").expect("color to parse"),
            (e, m) if e < 0.3 => {
                value = normalize(e, 0.12, 0.3);

                match m {
                    m if m < 0.16 => subtropical_desert(value),
                    m if m < 0.33 => grassland(value),
                    m if m < 0.83 => tropical_seasonal_forest(value),
                    _ => tropical_rain_forest(value)
                }
            },
            (e, m) if e < 0.6 => {
                value = normalize(e, 0.3, 0.6);

                match m {
                    m if m < 0.16 => temperate_desert(value),
                    m if m < 0.5 => grassland(value),
                    m if m < 0.83 => temperate_deciduous_forest(value),
                    _ => temperate_rain_forest(value)
                }
            },
            (e, m) if e < 0.8 => {
                value = normalize(e, 0.6, 0.8);

                match m {
                    m if m < 0.33 => temperate_desert(value),
                    m if m < 0.66 => shrubland(value),
                    _ => taiga(value)
                }
            }
            (e, m) => {
                value = normalize(e, 0.8, 1.0);

                match m {
                    m if m < 0.1 => scorched(value),
                    m if m < 0.2 => bare(value),
                    m if m < 0.5 => tundra(value),
                    _ => snow(value)
                }
            }
        }
    }
}

// SAFETY: Cell only contains a single f64, which is Send and Sync
// itself, so there should be no issue making Cell Send and Sync.
unsafe impl Send for Cell {}
unsafe impl Sync for Cell {}
