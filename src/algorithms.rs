mod recursive_backtracking;
mod eller;
mod kruskal;
mod prim;
mod recursive_division;
mod aldous_broder;

pub use recursive_backtracking::recursive_backtracking;
pub use eller::eller;
pub use kruskal::kruskal;
pub use prim::prim;
pub use recursive_division::recursive_division;
pub use aldous_broder::aldous_broder;

use crate::random::Random;
use crate::point::Direction;
use crate::point::Direction::*;
use crate::maze::Maze;
use crate::point::Point;

struct RandomDirection {
    dirs: Vec<Direction>,
}

impl RandomDirection {
    fn new(rand: &mut Random) -> RandomDirection {
        let mut dirs = vec![Up, Right, Down, Left];
        rand.shuffle(&mut dirs);
        RandomDirection{dirs}
    }

    fn dirs(&self) -> &Vec<Direction> {
        &self.dirs
    }
}

fn valid_neighbors<F>(rand: &mut Random, maze: &mut Maze, p: Point, visited: bool, once: bool, f: &mut F) -> ()
    where F: FnMut(&mut Maze, Point, Point, Direction, &mut Random) -> () {
    let dirs = RandomDirection::new(rand);
    for &i in dirs.dirs().iter() {
        let p_new = p.point_in_direction(i, maze.width(), maze.height());
        if let Ok(p_new) = p_new {
            if maze.get_visited(p_new) == visited {
                f(maze, p, p_new, i, rand);
                if once {
                    return;
                }
            }
        }
    }
}
