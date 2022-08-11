use crate::random::Random;
use crate::point::Point;
use crate::point::Direction;
use crate::maze::Maze;

fn carve_passage_from<F>(maze: &mut Maze, p: Point, rand: &mut Random, call_back: &mut F) -> ()
    where F: FnMut(&mut Maze) -> () {
    maze.set_pos(Some(p));
    maze.set_special(p);
    maze.set_visited(p);
    call_back(maze);
    super::valid_neighbors(
        rand, maze, p, false, false,
        &mut |maze: &mut Maze, p: Point, p_new: Point, dir: Direction, rand: &mut Random| {
            maze.carve_passage(dir);
            carve_passage_from(maze, p_new, rand, call_back);
            maze.set_pos(Some(p));
            call_back(maze);
        }
    );
    maze.clear_special(p);
}

pub fn recursive_backtracking<F>(maze: &mut Maze, rand: &mut Random, call_back: &mut F) -> ()
    where F: FnMut(&mut Maze) -> () {
    carve_passage_from(maze, Point::new(0, 0), rand, call_back);
    maze.set_pos(None);
}
