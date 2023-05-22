//! This module provides a [`Map`] structure and methods for
//! creating and modifying maps.
//!
//! Map generation can be seen as running a series of
//! tranformations over a starting point to incrementally
//! add more detail. I call this a "monad" but it's probably
//! not one. This makes it so that steps can be rearranged,
//! added/removed, and modified to allow for easier experimentation.
//!
//! # Examples
//!
//! ```
//! use ficture_generator::cell::{Cell};
//! use ficture_generator::map::{Map, MapMonad};
//!
//! // Create the initial map with all cells set to have
//! // 0 elevation.
//! let map = Map::return_single(Cell { elevation: 0.0, moisture: 0.0 }, 10, 10);
//!
//! // Run a really simple step that increases the elevation of
//! // each cell by 1.
//! let map = map.and_then(|cell| {
//!     Cell {
//!         elevation: cell.elevation + 1.0,
//!         moisture: cell.moisture
//!     }
//! });
//! ```
use rayon::prelude::*;
use std::{ops::Deref, sync::Arc};

/// Contains all information about a world map.
pub struct Map<T> {
    width: usize,
    height: usize,
    inner: Vec<T>,
}

impl<T> Map<T>
where
    T: Send + Clone,
{
    /// Returns an iterator that iterates over the
    /// cells in a map.
    pub fn iter(&self) -> MapIter<T> {
        MapIter {
            inner: &self.inner,
            index: 0,
        }
    }

    /// Gets the width of the map.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Gets the height of the map.
    pub fn height(&self) -> usize {
        self.height
    }
}

/// A trait created in an attempt to make [`Map`] monadic. Allows
/// you to chain operations on maps instead of mutating the map's
/// state.
pub trait MapMonad<T> {
    /// Create a [`Map`] filled with the value specified by `value`.
    fn return_single(value: T, width: usize, height: usize) -> Map<T>;

    /// Transform each object of type `T` that is stored in the map using
    /// the function `f`.
    fn and_then<F, U>(self, f: F) -> Map<U>
    where
        F: Fn(T) -> U + Send + Sync,
        U: Send;

    /// Transform each object of type `T` that is stored in the map using
    /// the function `f`. This method provides `f` with the x and y coordinates
    /// of the cell that is being transformed.
    fn and_then_with_coordinates<F, U>(self, f: F) -> Map<U>
    where
        F: Fn(&T, usize, usize) -> U + Send + Sync,
        U: Send;

    /// Provides a way to extract information about the cells
    /// in a map in order to use them in another way. For example,
    /// you may extract the cell values in order to generate images
    /// of the world map by using this method.
    fn extract<F, U>(self, f: F) -> U
    where
        F: Fn(Vec<T>, usize, usize) -> U + Send + Sync;
}

impl<T> MapMonad<T> for Map<T>
where
    T: Send + Sync + Sized + Clone,
{
    /// Creates a [`Map`] will every cell set to the same `value`.
    fn return_single(value: T, width: usize, height: usize) -> Self {
        let inner = vec![value; width * height];

        Self {
            width,
            height,
            inner,
        }
    }

    /// Creates a new [`Map`] where every cell is transformed by the
    /// function `f`. This is done concurrently to speed up computation.
    fn and_then<F, U>(self, f: F) -> Map<U>
    where
        F: Fn(T) -> U + Send + Sync,
        U: Send,
    {
        let new_inner: Vec<U> = self.inner.into_par_iter().map(|cell| f(cell)).collect();

        Map {
            width: self.width,
            height: self.height,
            inner: new_inner,
        }
    }

    /// Creates a new [`Map`] where every cell is transformed by the
    /// function `f`. `f` is given the x and y coordinates for the
    /// cell that is being transformed. This is done concurrently to
    /// speed up computation.
    fn and_then_with_coordinates<F, U>(self, f: F) -> Map<U>
    where
        F: Fn(&T, usize, usize) -> U + Send + Sync,
        U: Send,
    {
        let inner_ref = &self.inner;
        let f = Arc::new(f);
        let new_inner: Vec<U> = (0..self.height)
            .into_par_iter()
            .flat_map(move |y| {
                let f = f.clone();
                (0..self.width)
                    .into_par_iter()
                    .map(move |x| f(&inner_ref[y * self.width + x], x, y))
            })
            .collect();

        Map {
            width: self.width,
            height: self.height,
            inner: new_inner,
        }
    }

    /// Tranforms the current [`Map`] into a `U` using the
    /// function `f`.
    fn extract<F, U>(self, f: F) -> U
    where
        F: Fn(Vec<T>, usize, usize) -> U + Send + Sync,
    {
        f(self.inner, self.width, self.height)
    }
}

/// An iterator that iterates over all cells of a
/// [`Map`].
pub struct MapIter<'a, T> {
    inner: &'a Vec<T>,
    index: usize,
}

impl<'a, T> Iterator for MapIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.inner.len() {
            let result = &self.inner[self.index];
            self.index += 1;
            Some(result)
        } else {
            None
        }
    }
}

impl<T> Deref for Map<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::cell::Cell;

    #[test]
    fn test_return_single_fills_map() {
        let example_cell = Cell { elevation: 0.51, moisture: 0.0 };
        let map = Map::return_single(example_cell.clone(), 10, 10);
        let mut size = 0;

        for cell in map.iter() {
            assert_eq!(*cell, example_cell);
            size += 1;
        }

        assert_eq!(size, map.width() * map.height());
    }

    #[test]
    fn test_and_then_maps_cells() {
        let map = Map::return_single(Cell { elevation: 0.0, moisture: 0.0 }, 10, 10);
        let map = map.and_then(|cell| cell.elevation);

        for elevation in map.iter() {
            assert_eq!(*elevation, 0.0);
        }
    }

    #[test]
    fn test_and_then_with_coordinates_maps_cells() {
        let map = Map::return_single(Cell { elevation: 0.0, moisture: 0.0 }, 10, 10);
        let map = map.and_then_with_coordinates(|_, x, y| x * y);
        let mut map_iter = map.iter();

        for y in 0..map.height() {
            for x in 0..map.width() {
                let elevation = map_iter.next().unwrap();

                assert_eq!(*elevation, x * y);
            }
        }
    }

    #[test]
    fn test_extract() {
        let map = Map::return_single(Cell { elevation: 0.0, moisture: 0.0 }, 10, 10);
        let map = map.and_then_with_coordinates(|_, x, y| y * 10 + x);

        map.extract(|values, width, height| {
            let mut values_iter = values.iter();

            for y in 0..height {
                for x in 0..width {
                    assert_eq!(*values_iter.next().unwrap(), y * width + x);
                }
            }
        });
    }
}
