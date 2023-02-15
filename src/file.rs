use image::ImageError;
use image::io::Reader as ImageReader;
use std::path::Path;

use crate::config::Config;
use crate::maze::Maze;

pub fn parse_from_file(config: &Config) -> Result<Maze, ImageError> {
    let img = ImageReader::open(config.maze_path.as_str())?.decode()?;
    return Ok(Maze {
        pixels: image::DynamicImage::ImageRgb8(img.to_rgb8()),
    })
}

pub fn write_to_file(path: String, maze: &Maze) -> Result<(), ImageError> {
    // What should happen if the output file already exists?
    let path_obj = Path::new(&path);
    
    // Yikes -- there's gotta be a better way
    let solved_path = path_obj.with_file_name(
        format!("{}_solved.{}", 
                path_obj.file_stem().unwrap().to_str().unwrap(),
                path_obj.extension().unwrap().to_str().unwrap())
        );
    return maze.pixels.save(solved_path);
}
