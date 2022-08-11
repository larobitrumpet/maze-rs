use crate::point::Point;
use crate::maze::Maze;
use crate::random::Random;
use crate::point::Direction::*;

enum Orientation {
    Horizontal,
    Vertical,
}

use Orientation::*;

fn pick_orientation(rand: &mut Random, width: usize, height: usize) -> Orientation {
    if width > height {
        Vertical
    } else if width < height {
        Horizontal
    } else {
        if rand.rand_usize(0, 2) == 1 {
            Horizontal
        } else {
            Vertical
        }
    }
}

fn divide<F>(maze: &mut Maze, rand: &mut Random, p: Point, width: usize, height: usize, call_back: &mut F) -> ()
    where F: FnMut(&mut Maze) -> () {
    if width < 2 || height < 2 {
        return;
    }
    match pick_orientation(rand, width, height) {
        Horizontal => {
            let d = rand.rand_usize(p.y(), p.y() + height - 2);
            for i in p.x()..(p.x() + width) {
                maze.set_pos(Some(Point::new(i, d)));
                maze.fill_passage(Down);
            }
            let c = rand.rand_usize(p.x(), p.x() + width - 1);
            maze.set_pos(Some(Point::new(c, d)));
            maze.carve_passage(Down);
            call_back(maze);
            divide(maze, rand, p, width, d + 1 - p.y(), call_back);
            divide(maze, rand, Point::new(p.x(), d + 1), width, height + p.y() - d - 1, call_back);
        },
        Vertical => {
            let d = rand.rand_usize(p.x(), p.x() + width - 2);
            for i in p.y()..(p.y() + height) {
                maze.set_pos(Some(Point::new(d, i)));
                maze.fill_passage(Right);
            }
            let c = rand.rand_usize(p.y(), p.y() + height - 1);
            maze.set_pos(Some(Point::new(d, c)));
            maze.carve_passage(Right);
            call_back(maze);
            divide(maze, rand, p, d + 1 - p.x(), height, call_back);
            divide(maze, rand, Point::new(d + 1, p.y()), width + p.x() - d - 1, height, call_back);
        },
    }
}

pub fn recursive_division<F>(maze: &mut Maze, rand: &mut Random, call_back: &mut F) -> ()
    where F: FnMut(&mut Maze) -> () {
    divide(maze, rand, Point::new(0, 0), maze.width(), maze.height(), call_back);
    maze.set_pos(None);
}
