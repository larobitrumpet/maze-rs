use crate::point::Point;
use crate::point::Direction;
use crate::random::Random;
use crate::maze::Maze;

fn vec_index<T: std::cmp::PartialEq>(v: &Vec<T>, element: &T) -> Option<usize> {
    for (index, value) in v.iter().enumerate() {
        if *value == *element {
            return Some(index);
        }
    }
    None
}

fn vec_remove<T: std::cmp::PartialEq>(v: &mut Vec<T>, element: &T) -> () {
    if let Some(index) = vec_index(v, element) {
        v.remove(index);
    }
}

#[derive(Debug)]
struct WilsonPath {
    points: Vec<Point>,
    dirs: Vec<Direction>,
    len: usize,
}

impl WilsonPath {
    fn new() -> WilsonPath {
        WilsonPath {
            points: vec![],
            dirs: vec![],
            len: 0,
        }
    }

    fn push(&mut self, maze: &mut Maze, p: Point, dir: Direction) -> () {
        let index = vec_index(&self.points, &p);
        match index {
            Some(index) => {
                for i in (index + 1)..(self.len) {
                    maze.clear_special(self.points[i]);
                }
                for _ in index..self.len {
                    self.points.pop();
                    self.dirs.pop();
                }
                self.points.push(p);
                self.dirs.push(dir);
                self.len = index + 1;
            },
            None => {
                self.points.push(p);
                self.dirs.push(dir);
                self.len += 1;
                maze.set_special(p);
            },
        }
    }

    pub fn get<F>(maze: &mut Maze, rand: &mut Random, not_in_maze: &Vec<Point>, start: Point, call_back: &mut F) -> WilsonPath
        where F: FnMut(&mut Maze) -> () {
        let mut wil_path = WilsonPath::new();
        let path = &mut wil_path;
        let mut p = start;
        let p_pointer = &mut p;
        maze.set_pos(Some(*p_pointer));
        call_back(maze);
        while let Some(_) = vec_index(not_in_maze, p_pointer) {
            super::valid_neighbors(
                rand, maze, *p_pointer, false, true,
                &mut |maze, p, p_new, dir, _rand| {
                    path.push(maze, p, dir);
                    *p_pointer = p_new;
                    maze.set_pos(Some(*p_pointer));
                }
            );
            call_back(maze);
        }

        wil_path
    }

    pub fn follow<F>(&self, maze: &mut Maze, not_in_maze: &mut Vec<Point>, call_back: &mut F) -> ()
        where F: FnMut(&mut Maze) -> () {
        for i in 0..self.len {
            let p = self.points[i];
            let dir = self.dirs[i];
            maze.set_pos(Some(p));
            maze.carve_passage(dir);
            maze.clear_special(p);
            vec_remove(not_in_maze, &p);
            call_back(maze);
        }
    }
}

pub fn wilson<F>(maze: &mut Maze, rand: &mut Random, call_back: &mut F) -> ()
    where F: FnMut(&mut Maze) -> () {
    let mut not_in_maze: Vec<Point> = Vec::with_capacity(maze.width() * maze.height());
    for y in 0..maze.height() {
        for x in 0..maze.width() {
            not_in_maze.push(Point::new(x, y));
        }
    }
    rand.shuffle(&mut not_in_maze);
    not_in_maze.pop();
    while not_in_maze.len() > 0 {
        let start = not_in_maze[not_in_maze.len() - 1];
        WilsonPath::get(maze, rand, &not_in_maze, start, call_back).follow(maze, &mut not_in_maze, call_back);
    }

    maze.set_pos(None);
}
