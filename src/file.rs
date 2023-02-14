use image::ImageError;
use image::io::Reader as ImageReader;

use crate::config::Config;
use crate::maze::Maze;

pub fn parse_from_file(config: &Config) -> Result<Maze, ImageError> {
    let img = ImageReader::open(config.maze_path.as_str())?.decode()?;
    return Ok(Maze {
        pixels: image::DynamicImage::ImageRgb8(img.to_rgb8()),
    })
}
