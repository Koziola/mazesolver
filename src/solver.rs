use std::{cmp::Reverse, collections::HashMap, panic};

use image::{GenericImageView, GenericImage, Pixel};
use priority_queue::PriorityQueue;

use crate::maze::Maze;

const WHITE_PIXEL: [u8; 4] = [255, 255, 255, 255];
const BLACK_PIXEL: [u8; 4] = [0, 0, 0, 255];
const GREEN_PIXEL: [u8; 4] = [0, 255, 0, 255];

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Point {
    x: i32,
    y: i32
}

pub fn solve(maze: &Maze) -> Result<Maze, &'static str> {
    let start_point = match find_start(maze) {
        Ok(start_point) => start_point,
        Err(e) => return Err(e),
    };

    let end_point = match find_end(maze) {
        Ok(end_point) => end_point,
        Err(e) => return Err(e),
    };

    let path = match solve_a_star(maze, start_point, end_point) {
        Some(path) => path,
        None => return Err("unable to find a path that solves the maze!"),
    };

    let solved_maze = color_path(maze, path);

    return Ok(solved_maze);
}


// Finds the start point of the maze.
// Assumes that the maze contains a white pixel in the first row, which 
// we'll assume is the start point.
fn find_start(maze: &Maze) -> Result<Point, &'static str> {
    for x in 0..maze.pixels.width() {
        let pixel = maze.pixels.get_pixel(x, 0);
        if pixel.0 == WHITE_PIXEL{
            return Ok(Point{
                x: (x as i32),
                y: 0
            })
        }
    }

    return Err("Unable to find start point for maze!")
}

fn find_end(maze: &Maze) -> Result<Point, &'static str> {
    for x in 0..maze.width() {
        let pixel = maze.pixels.get_pixel(x as u32, (maze.height() as u32) - 1);
        if pixel.0 == WHITE_PIXEL{
            return Ok(Point{
                x,
                y: maze.height() - 1,
            })
        }
    }

    return Err("Unable to find end point for maze!")
}

// Solves the maze using the A* algorithm.  Returns a list of coordinates which is the path through
// the maze from start to finish.
fn solve_a_star(maze: &Maze, start_point: Point, end_point: Point) -> Option<Vec<Point>> {
    println!("Finding a path from {}, {} to {}, {}...", start_point.x, start_point.y, end_point.x, end_point.y);

    let mut to_explore: PriorityQueue<Point, Reverse<i32>> = PriorityQueue::new();
    to_explore.push(start_point.clone(), Reverse(0));

    // for every point, keeps track of where that point was explored from.
    let mut came_from: HashMap<Point, Point> = HashMap::new();

    // for each node, what's the cost of the cheapest path we know of to get there.
    let mut path_score: HashMap<Point, i32> = HashMap::new();
    path_score.insert(start_point.clone(), 0);

    while !to_explore.is_empty() {
        //pop the current node we're exploring.
        let current = to_explore.pop().unwrap().0;
        if current == end_point {
            println!("Found the end!");
            let path = reconstruct_path(came_from, end_point);
            return Some(path)
        }
        let current_score = *path_score.get(&current).unwrap();

        for neighbor in find_neighbors(maze, &current) {
            let current_neighbor_score = *path_score
                .get(&neighbor)
                .unwrap_or(&i32::MAX);
            
            // The score to reach one of the neighbor nodes is the score to reach
            // the current node + 1.
            let score = current_score + 1;

            if score < current_neighbor_score {
                came_from.insert(neighbor.clone(), current.clone());
                path_score.insert(neighbor.clone(), score);
                // Pixels that are closer to our goal (lower distance) should have
                // higher priority.
                let priority = score + distance_between(&neighbor, &end_point);
                to_explore.push(neighbor.clone(), Reverse(priority));
            } 
        }
    }

    return None
}

// finds all neighbors of the current node that are not walls
fn find_neighbors(maze: &Maze, current: &Point) -> Vec<Point> {
    let mut neighbors = vec![];
    if is_open(maze, current.x - 1, current.y) {
        neighbors.push(Point{ x: current.x - 1, y: current.y });
    }
    if is_open(maze, current.x + 1, current.y) {
        neighbors.push(Point{ x: current.x + 1, y: current.y });
    }
    if is_open(maze, current.x, current.y - 1) {
        neighbors.push(Point{ x: current.x, y: current.y - 1 });
    }
    if is_open(maze, current.x, current.y + 1) {
        neighbors.push(Point{ x: current.x, y: current.y + 1 });
    }

    return neighbors;
}

fn is_open(maze: &Maze, x: i32, y: i32) -> bool {
    if x < 0 || x > (maze.width() - 1) || y < 0 || y > (maze.height() - 1) {
        return false
    }

    let pixel = maze.pixels.get_pixel(x as u32, y as u32);
    if pixel.0 == BLACK_PIXEL {
        return false
    } else if pixel.0 == WHITE_PIXEL {
        return true
    } else {
        panic!("Unknown pixel at {}, {}: {:?}", x, y, pixel);
    }
}


fn distance_between(point: &Point, end_point: &Point) -> i32 {
    let intermediate: f64 = ((end_point.x - point.x).pow(2) + (end_point.y - point.y).pow(2)).into();
    let result = intermediate.sqrt() as i32;
    return result;
}

fn reconstruct_path(came_from: HashMap<Point, Point>, end_point: Point) -> Vec<Point> {
    let mut path = vec![end_point.clone()];
    let mut current: Option<Point> = Some(end_point);
    // TODO: I bet this could get cleaned up.
    while current.is_some() {
        let parent = came_from.get(&current.unwrap());
        if let Some(i) = parent {
            path.push(i.clone());
            current = Some(i.clone());
        } else {
            current = None;
        }
    }

    return path;
}

fn color_path(maze: &Maze, path: Vec<Point>) -> Maze {
    let mut solved_maze = Maze {
        pixels: maze.pixels.clone(),
    };
    
    for element in path {
        solved_maze.pixels.put_pixel(element.x as u32, element.y as u32, image::Rgba(GREEN_PIXEL));
    }
    return solved_maze;
}
