use crate::point::Point;
use crate::maze::Maze;
use crate::random::Random;

fn in_vec<T: std::cmp::PartialEq>(v: &Vec<T>, e: &T) -> bool {
    for i in v {
        if i == e {
            return true;
        }
    }
    false
}

fn add_to_frontier(frontier: &mut Vec<Point>, p: Point) -> () {
    match in_vec(frontier, &p) {
        true => {},
        false => frontier.push(p),
    }
}

fn add_unvisited_neighbors_to_frontier(maze: &mut Maze, rand: &mut Random, p: Point, frontier: &mut Vec<Point>) {
    super::valid_neighbors(
        rand, maze, p, false, false,
        &mut |maze, _p, p_new, _dir, _rand| {
            add_to_frontier(frontier, p_new);
            maze.set_special(p_new);
        }
    );
}

fn join_to_maze(maze: &mut Maze, rand: &mut Random, p: Point) {
    super::valid_neighbors(
        rand, maze, p, true, true,
        &mut |maze, p, _p_new, dir, _rand| {
            maze.set_pos(Some(p));
            maze.carve_passage(dir);
            maze.clear_special(p);
        }
    );
}

pub fn prim<F>(maze: &mut Maze, rand: &mut Random, _weights: Vec<u32>, call_back: &mut F) -> ()
    where F: FnMut(&mut Maze) -> () {
    let mut frontier: Vec<Point> = vec![];
    let p = rand.rand_point(maze.width(), maze.height());
    maze.set_visited(p);
    add_unvisited_neighbors_to_frontier(maze, rand, p, &mut frontier);
    call_back(maze);
    while frontier.len() > 0 {
        rand.shuffle(&mut frontier);
        let p = frontier.pop().unwrap();
        maze.set_visited(p);
        add_unvisited_neighbors_to_frontier(maze, rand, p, &mut frontier);
        join_to_maze(maze, rand, p);
        call_back(maze);
    }
}
