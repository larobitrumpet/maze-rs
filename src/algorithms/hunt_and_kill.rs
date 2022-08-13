use crate::random::Random;
use crate::maze::Maze;
use crate::point::Point;

fn hunt_and_kill_walk<F>(maze: &mut Maze, rand: &mut Random, start: Point, call_back: &mut F) -> ()
    where F: FnMut(&mut Maze) -> () {
    let mut p = start;
    let p_pointer = &mut p;
    maze.set_pos(Some(*p_pointer));
    maze.set_visited(*p_pointer);
    call_back(maze);
    while super::valid_neighbors(
        rand, maze, *p_pointer, false, true,
        &mut |maze, _p_orig, p_new, dir, _rand| {
            maze.carve_passage(dir);
            *p_pointer = p_new;
        }
        ) {
        maze.set_pos(Some(*p_pointer));
        maze.set_visited(*p_pointer);
        call_back(maze);
    }
}

fn hunt_and_kill_hunt<F>(maze: &mut Maze, rand: &mut Random, start: Point, call_back: &mut F) -> Option<Point>
    where F: FnMut(&mut Maze) -> () {
    let mut p = start;
    let p_ptr = &mut p;
    maze.set_pos(Some(*p_ptr));
    let mut new_start: Option<Point> = None;
    let new_start_ptr = &mut new_start;
    call_back(maze);
    loop {
        if !maze.get_visited(*p_ptr) {
            let n = super::valid_neighbors(
            rand, maze, *p_ptr, true, true,
            &mut |maze, p_orig, _p_new, dir, _rand| {
                maze.carve_passage(dir);
                maze.set_visited(p_orig);
                *new_start_ptr = Some(p_orig);
            });
            if  n {
                *new_start_ptr = Some(*p_ptr);
                break;
            }
        }
        match p_ptr.next(maze.width(), maze.height()) {
            Some(p_new) => *p_ptr = p_new,
            None => {
                *new_start_ptr = None;
                break;
            },
        }
        maze.set_pos(Some(*p_ptr));
        call_back(maze);
    }

    new_start
}

pub fn hunt_and_kill<F>(maze: &mut Maze, rand: &mut Random, _weights: Vec<u32>, call_back: &mut F) -> ()
    where F: FnMut(&mut Maze) -> (){
    let mut p: Point = rand.rand_point(maze.width(), maze.height());
    let mut start: Option<Point> = None;
    loop {
        hunt_and_kill_walk(maze, rand, p, call_back);
        let new_p = hunt_and_kill_hunt(
            maze, rand,
            match start {
                Some(s) => s,
                None => Point::new(0, 0),
            },
            call_back);
        match new_p {
            Some(new_p) => {
                p = new_p;
                match start {
                    Some(_) => {
                        start = p.next(maze.width(), maze.height());
                    },
                    None => {
                        if maze.get_visited(Point::new(0, 0)) {
                            start = Some(Point::new(0, 0));
                        }
                    },
                }
            },
            None => break,
        }
    }

    maze.set_pos(None);
}
