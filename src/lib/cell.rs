//! This module provides a [`Cell`] representing a single point
//! on a 2D world map.

#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    /// The elevation at a point on the map. Usually
    /// normalized from 0-1.
    pub elevation: f64,
    /// The moisture at a point on the map. Usually
    /// normalized from 0-1.
    pub moisture: f64,
}

// SAFETY: Cell only contains a single f64, which is Send and Sync
// itself, so there should be no issue making Cell Send and Sync.
unsafe impl Send for Cell {}
unsafe impl Sync for Cell {}
