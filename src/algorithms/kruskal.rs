use std::collections::HashMap;

use crate::point::Point;
use crate::point::Direction;
use crate::point::Direction::*;
use crate::maze::Maze;
use crate::random::Random;

// Areana structure inspired from:
// https://rust-leipzig.github.io/architecture/2016/12/20/idiomatic-trees-in-rust/
// Licensed under the MIT License:
// Copyright (c) 2018 Sascha Grunert

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct NodeId {
    index: usize,
}

#[derive(Debug)]
struct Node<T> {
    _value: T,
    parent: Option<NodeId>,
    children: Vec<NodeId>,
}

#[derive(Debug)]
struct Arena<T> {
    nodes: Vec<Node<T>>
}

impl<T> Arena<T> {
    pub fn new() -> Arena<T> {
        Arena { nodes: vec![] }
    }

    pub fn new_node(&mut self, value: T) -> NodeId {
        let next_index = self.nodes.len();

        self.nodes.push(Node {
            _value: value,
            parent: None,
            children: vec![],
        });

        NodeId { index: next_index }
    }

    pub fn tree_union(&mut self, parent: NodeId, child: NodeId) -> () {
        let child_root = self.get_root(child);
        self.nodes[parent.index].children.push(child_root);
        self.nodes[child_root.index].parent = Some(parent);
    }

    pub fn get_root(&self, node: NodeId) -> NodeId {
        let mut n = node;
        while let Some(p) = self.nodes[n.index].parent {
            n = p;
        }
        n
    }

    pub fn is_family(&self, node1: NodeId, node2: NodeId) -> bool {
        let root1 = self.get_root(node1);
        let root2 = self.get_root(node2);
        root1 == root2
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Wall {
    point: Point,
    direction: Direction,
}

impl Wall {
    pub fn new(p: Point, dir: Direction) -> Wall {
        Wall {
            point: p,
            direction: dir,
        }
    }

    pub fn point(&self) -> Point {
        self.point
    }

    pub fn neighbor(&self, width: usize, height: usize) -> Point {
        self.point.point_in_direction(self.direction, width, height).unwrap()
    }
}

fn kruskal_init(maze: &mut Maze, rand: &mut Random) -> (Arena<Point>, HashMap<Point, NodeId>, Vec<Wall>) {
    let mut arena = Arena::new();
    let mut point_nodes = HashMap::new();
    for y in 0..maze.height() {
        for x in 0..maze.width() {
            let p = Point::new(x, y);
            point_nodes.insert(p, arena.new_node(p));
        }
    }

    let mut edges: Vec<Wall> = Vec::with_capacity(maze.width() * maze.height() * 2 - maze.width() - maze.height());
    let dirs = [Right, Down];
    for y in 0..(maze.height() - 1) {
        for x in 0..(maze.width() - 1) {
            for d in dirs {
                edges.push(Wall::new(Point::new(x, y), d));
            }
        }
        edges.push(Wall::new(Point::new(maze.width() - 1, y), Down));
    }
    for x in 0..(maze.width() - 1) {
        edges.push(Wall::new(Point::new(x, maze.height() - 1), Right));
    }
    rand.shuffle(&mut edges);

    (arena, point_nodes, edges)
}

pub fn kruskal<F>(maze: &mut Maze, rand: &mut Random, call_back: &mut F) -> ()
    where F: FnMut(&mut Maze) -> () {
    let (mut arena, point_nodes, edges) = kruskal_init(maze, rand);

    for i in edges {
        let p = i.point();
        maze.set_pos(Some(p));
        let p1 = *point_nodes.get(&p).unwrap();
        let p2 = *point_nodes.get(&i.neighbor(maze.width(), maze.height())).unwrap();
        if !arena.is_family(p1, p2) {
            maze.carve_passage(i.direction);
            arena.tree_union(p1, p2);
        }
        call_back(maze);
    }
    maze.set_pos(None);
}
