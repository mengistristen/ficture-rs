//! This module contains helper functions for converting
//! [`Map`](crate::map::Map)s into images.
//!
//! This module provides the following helper functions: 
//! - [`pixel_map_to_image`]
//!
//! # Examples
//!
//! ```
//! use ficture_generator::cell::Cell;
//! use ficture_generator::color::GetColor;
//! use ficture_generator::image::pixel_map_to_image;
//! use ficture_generator::map::{MapMonad, Map};
//!
//! let map = Map::return_single(Cell { elevation: 0.0, moisture: 0.0 }, 10, 10);
//! let map = map.and_then(|cell| cell.get_color());
//! let image = map.extract(pixel_map_to_image);
//! ```
use image::{Rgb, RgbImage};

/// A helper function for use in `extract` to turn a [`Map`](crate::map::Map) of
/// `Rgb<u8>` into an RGB image.
pub fn pixel_map_to_image(pixels: Vec<Rgb<u8>>, width: usize, height: usize) -> RgbImage {
    let mut pixel_iter = pixels.iter();
    let mut image = RgbImage::new(width as u32, height as u32);

    for y in 0..height {
        for x in 0..width {
            image.put_pixel(x as u32, y as u32, *pixel_iter.next().unwrap());
        }
    }

    image
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{map::{Map, MapMonad}, cell::Cell, color::GetColor};

    #[test]
    fn test_image_matches_map_dimensions() {
        let width = 1920;
        let height = 1080;
        let map = Map::return_single(Cell { elevation: 0.0, moisture: 0.0 }, width, height);
        let map = map.and_then(|cell| cell.get_color());
        let image = map.extract(pixel_map_to_image);

        assert_eq!(image.width(), width as u32);
        assert_eq!(image.height(), height as u32);
    }
}
