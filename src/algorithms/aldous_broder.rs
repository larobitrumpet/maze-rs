use crate::random::Random;
use crate::maze::Maze;

pub fn aldous_broder<F>(maze: &mut Maze, rand: &mut Random, call_back: &mut F) -> ()
    where F: FnMut(&mut Maze) -> () {
    let mut remaining: usize = maze.width() * maze.height();
    let mut p = rand.rand_point(maze.width(), maze.height());
    maze.set_pos(Some(p));
    maze.set_visited(p);
    remaining -= 1;

    while remaining > 0 {
        let dirs = super::RandomDirection::new(rand);
        for &i in dirs.dirs().iter() {
            let p_new = p.point_in_direction(i, maze.width(), maze.height());
            if let Ok(p_new) = p_new {
                p = p_new;
                if !maze.get_visited(p) {
                    maze.carve_passage(i);
                    maze.set_visited(p);
                    remaining -= 1;
                }
                maze.set_pos(Some(p));
                call_back(maze);
                break;
            }
        }
    }

    maze.set_pos(None);
}
