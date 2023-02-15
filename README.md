To run:
```
cargo build && cargo run <path to maze>
```
where <path to maze> is a valid image file containing a maze.  Mazes are assumed to have a start point marked by a white pixel in the first row, and an end point marked by a white pixel in the last row.

The program will automatically solve the maze, and write the modified image file to disk in the same location as the input file with a `_solved` suffix appended to the file name.

There are a few examples in the `/examples` sub-directory, which have been copied from [mikepound/mazesolving](https://github.com/mikepound/mazesolving).
