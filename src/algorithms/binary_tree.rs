use crate::random::Random;
use crate::point::Direction::*;
use crate::point::Point;
use crate::maze::Maze;

pub fn binary_tree<F>(maze: &mut Maze, rand: &mut Random, _weights: Vec<u32>, call_back: &mut F) -> ()
    where F: FnMut(&mut Maze) -> () {
    for x in 1..maze.width() {
        maze.set_pos(Some(Point::new(x, 0)));
        maze.carve_passage(Left);
        call_back(maze);
    }
    for y in 1..maze.height() {
        maze.set_pos(Some(Point::new(0, y)));
        maze.carve_passage(Up);
        call_back(maze);
        for x in 1..maze.width() {
            maze.set_pos(Some(Point::new(x, y)));
            if rand.rand_usize(0, 2) == 1 {
                maze.carve_passage(Up);
            } else {
                maze.carve_passage(Left);
            }
            call_back(maze);
        }
    }

    maze.set_pos(None);
}
