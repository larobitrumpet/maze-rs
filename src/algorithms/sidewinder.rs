use crate::random::Random;
use crate::point::Direction::*;
use crate::point::Point;
use crate::maze::Maze;

pub fn sidewinder<F>(maze: &mut Maze, rand: &mut Random, _weights: Vec<u32>, call_back: &mut F) -> ()
    where F: FnMut(&mut Maze) -> () {
    for x in 0..(maze.width() - 1) {
        maze.set_pos(Some(Point::new(x, 0)));
        maze.carve_passage(Right);
        call_back(maze);
    }

    for y in 1..maze.height() {
        let mut run_start = 0;
        for x in 0..(maze.width() - 1) {
            if rand.rand_usize(0, 2) == 1 {
                maze.set_pos(Some(Point::new(x, y)));
                maze.carve_passage(Right);
            } else {
                maze.set_pos(Some(Point::new(rand.rand_usize(run_start, x + 1), y)));
                maze.carve_passage(Up);
                run_start = x + 1;
            }
            call_back(maze);
        }
        maze.set_pos(Some(Point::new(rand.rand_usize(run_start, maze.width()), y)));
        maze.carve_passage(Up);
        call_back(maze);
    }

    maze.set_pos(None);
}
