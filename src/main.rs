extern crate image;
use std::env;
use std::process;

mod config;
mod file;
mod maze;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: config::Config = config::Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let maze: maze::Maze = file::parse_from_file(&config).unwrap_or_else(|err| {
        println!("Error parsing maze image from file: {}", err);
        process::exit(1);
    });
    println!("Maze dimensions: {} x {}", 
             maze.pixels.height(),
             maze.pixels.width());
}
