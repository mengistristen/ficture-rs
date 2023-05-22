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

        if elevation < 0.1 { return ocean(value); }
        if elevation < 0.12 { return color_to_rgb("#01c7dd").expect("color to parse"); }

        if elevation < 0.3 {
            value = normalize(elevation, 0.12, 0.3);

            if moisture < 0.16 { return subtropical_desert(value); }
            if moisture < 0.33 { return grassland(value); }
            if moisture < 0.83 { return tropical_seasonal_forest(value); }
            return tropical_rain_forest(value);
        }

        if elevation < 0.6 {
            value = normalize(elevation, 0.3, 0.6);

            if moisture < 0.16 { return temperate_desert(value); }
            if moisture < 0.5 { return grassland(value); }
            if moisture < 0.83 { return temperate_deciduous_forest(value); }
            return temperate_rain_forest(value);
        }

        if elevation < 0.8 {
            value = normalize(elevation, 0.6, 0.8);

            if moisture < 0.33 { return temperate_desert(value); }
            if moisture < 0.66 { return shrubland(value); }
            return taiga(value);
        }

        value = normalize(elevation, 0.8, 1.0);

        if moisture < 0.1 { return scorched(value); }
        if moisture < 0.2 { return bare(value); }
        if moisture < 0.5 { return tundra(value); }
        return snow(value);
    }
}

// SAFETY: Cell only contains a single f64, which is Send and Sync
// itself, so there should be no issue making Cell Send and Sync.
unsafe impl Send for Cell {}
unsafe impl Sync for Cell {}
