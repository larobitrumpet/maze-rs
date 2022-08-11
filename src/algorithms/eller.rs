use std::collections::HashMap;

use crate::random::Random;
use crate::point::Point;
use crate::point::Direction::*;
use crate::maze::Maze;

fn same_set(row: &mut Vec<u32>, x1: usize, x2: usize) -> bool {
    row[x1] == row[x2]
}

fn join_set(row: &mut Vec<u32>, next_row: Option<&mut Vec<u32>>, set_from: u32, set_to: u32, len: usize) -> () {
    for i in 0..len {
        if row[i] == set_from {
            row[i] = set_to;
        }
    }
    if let Some(next_row) = next_row {
        for i in 0..len {
            if next_row[i] == set_from {
                next_row[i] = set_to;
            }
        }
    }
}

fn get_set_indexes(row: &mut Vec<u32>, len: usize) -> HashMap<u32, Vec<usize>> {
    let mut sets: HashMap<u32, Vec<usize>> = HashMap::new();
    for x in 0..len {
        let i = sets.entry(row[x]).or_insert(vec![]);
        i.push(x);
    }
    sets
}

fn join_right(maze: &mut Maze, row: &mut Vec<u32>, x: usize) -> () {
    maze.carve_passage(Right);
    join_set(row, None, row[x + 1], row[x], maze.width());
}

fn join_down(maze: &mut Maze, row: &mut Vec<u32>, next_row: &mut Vec<u32>, x: usize) -> () {
    let set_from = next_row[x];
    let set_to = row[x];
    maze.carve_passage(Down);
    join_set(row, Some(next_row), set_from, set_to, maze.width());
}

fn eller_row<F>(maze: &mut Maze, rand: &mut Random, row: &mut Vec<u32>, set_num: &mut u32, y: usize, call_back: &mut F) -> Vec<u32>
    where F: FnMut(&mut Maze) -> () {
    for x in 0..(maze.width() - 1) {
        maze.set_pos(Some(Point::new(x, y)));
        if !same_set(row, x, x + 1) && rand.rand_usize(0, 2) == 1 {
            join_right(maze, row, x);
        }
        call_back(maze);
    }

    let mut sets = get_set_indexes(row, maze.width());
    let mut next_row = vec![0; maze.width()];

    for i in 0..maze.width() {
        next_row[i] = *set_num;
        *set_num += 1;
    }
    
    for (_, value) in &mut sets {
        let num = rand.rand_usize(1, value.len() + 1);
        rand.shuffle(value);
        for i in 0..num {
            maze.set_pos(Some(Point::new(value[i], y)));
            join_down(maze, row, &mut next_row, value[i]);
            call_back(maze);
        }
    }

    next_row
}

fn eller_last_row<F>(maze: &mut Maze, row: &mut Vec<u32>, y: usize, call_back: &mut F)
    where F: FnMut(&mut Maze) -> () {
    for x in 0..(maze.width() - 1) {
        maze.set_pos(Some(Point::new(x, y)));
        if !same_set(row, x, x + 1) {
            join_right(maze, row, x);
        }
        call_back(maze);
    }
}

pub fn eller<F>(maze: &mut Maze, rand: &mut Random, call_back: &mut F) -> ()
    where F: FnMut(&mut Maze) -> () {
    let mut set_num = 1;
    let mut row: Vec<u32> = vec![0; maze.width()];

    for i in 0..maze.width() {
        row[i] = set_num;
        set_num += 1;
    }
    
    for y in 0..(maze.height() - 1) {
        row = eller_row(maze, rand, &mut row, &mut set_num, y, call_back);
    }

    eller_last_row(maze, &mut row, maze.height() - 1, call_back);

    maze.set_pos(None);
}
