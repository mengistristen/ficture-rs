//! This example allows you to see the different colors
//! used in the map generation by creating an image with
//! elevation on the x axis and moisture on the y axis.
use ficture::cell::Cell;
use ficture::config::Config;
use ficture::image::pixel_map_to_image;
use ficture::map::{Map, MapMonad};

fn main() {
    let (width, height) = (1000, 1000);
    let config = Config::from_file("examples/biomes/config.yaml").unwrap();

    config.validate().unwrap();

    let evaluator = config
        .get_color_evaluator("default")
        .expect("default color evaluator to exist");

    let map = Map::return_single(
        Cell {
            elevation: 0.0,
            moisture: 0.0,
        },
        width,
        height,
    );
    let map = map.and_then_with_coordinates(|_, x, y| Cell {
        elevation: x as f64 / width as f64,
        moisture: y as f64 / height as f64,
    });
    let map = map.and_then(|cell| evaluator.evaluate(cell.elevation, cell.moisture));
    let image = map.extract(pixel_map_to_image);

    image.save("biomes.png").expect("image to save");
}
