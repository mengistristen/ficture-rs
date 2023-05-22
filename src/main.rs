use ficture_generator::args::{Args, Parser};
use ficture_generator::cell::Cell;
use ficture_generator::color::GetColor;
use ficture_generator::image::pixel_map_to_image;
use ficture_generator::map::{Map, MapMonad};
use ficture_generator::noise::{SimpleNoiseGenerator, SimplexNoiseGeneratorBuilder};

fn main() {
    let args = Args::parse();
    let elevation_noise_generator = SimplexNoiseGeneratorBuilder::new(args.width, args.height)
        .octaves(6)
        .persistence(2.0)
        .lacunarity(3.0)
        .build();
    let moisture_noise_generator = SimplexNoiseGeneratorBuilder::new(args.width, args.height)
        .octaves(10)
        .persistence(3.0)
        .lacunarity(7.0)
        .build();

    let map: Map<Cell> = Map::return_single(
        Cell {
            elevation: 0.0,
            moisture: 0.0,
        },
        args.width,
        args.height,
    );

    // use noise to create a heightmap
    let map = map.and_then_with_coordinates(|cell, x, y| Cell {
        elevation: elevation_noise_generator.generate(x, y),
        moisture: cell.moisture,
    });

    // use noise to create a moisture map
    let map = map.and_then_with_coordinates(|cell, x, y| Cell {
        elevation: cell.elevation,
        moisture: moisture_noise_generator.generate(x, y),
    });

    // get the min and max elevation for normalization
    let (min_elevation, max_elevation) = map.iter().fold(
        (f64::MAX, f64::MIN),
        |(min_elevation, max_elevation), cell| {
            (
                min_elevation.min(cell.elevation),
                max_elevation.max(cell.elevation),
            )
        },
    );

    // normalize the elevation
    let map = map.and_then(|cell| {
        let elevation = (cell.elevation - min_elevation) / (max_elevation - min_elevation);

        Cell {
            elevation,
            moisture: cell.moisture,
        }
    });

    // get the min and max moisture for normalization
    let (min_moisture, max_moisture) = map.iter().fold(
        (f64::MAX, f64::MIN),
        |(min_moisture, max_moisture), cell| {
            (
                min_moisture.min(cell.moisture),
                max_moisture.max(cell.moisture),
            )
        },
    );

    // normalize the moisture
    let map = map.and_then(|cell| {
        let moisture = (cell.moisture - min_moisture) / (max_moisture - min_moisture);

        Cell {
            elevation: cell.elevation,
            moisture,
        }
    });

    let map = map.and_then(|cell| cell.get_color());
    let image = map.extract(pixel_map_to_image);

    image.save("image.png").unwrap();
}
