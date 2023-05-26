//! This example allows you to see the different colors
//! used in the map generation by creating an image with
//! elevation on the x axis and moisture on the y axis.
use ficture_generator::map::{Map, MapMonad};
use ficture_generator::cell::Cell;
use ficture_generator::color::GetColor;
use ficture_generator::image::pixel_map_to_image;

fn main() {
    let (width, height) = (1000, 1000);
    let map = Map::return_single(Cell { elevation: 0.0, moisture: 0.0 }, width, height);
    let map = map.and_then_with_coordinates(|_, x, y| {
        Cell {
            elevation: x as f64 / width as f64,
            moisture: y as f64 / height as f64
        }
    });
    let map = map.and_then(|cell| cell.get_color());
    let image = map.extract(pixel_map_to_image);

    image.save("biomes.png").expect("image to save");
}
