use crate::random::Random;
use crate::maze::Maze;

struct Set<T: Copy> {
    s: Vec<T>,
    weights: Vec<u32>,
    r: Option<usize>,
    c: usize,
}

impl<T: Copy> Set<T> {
    pub fn new(weights: Vec<u32>) -> Set<T> {
        assert_eq!(weights.len(), 5);
        Set {
            s: vec![],
            weights,
            r: None,
            c: 4,
        }
    }

    fn peak_newest(&self) -> Option<T> {
        if self.s.len() > 0 {
            Some(self.s[self.s.len() - 1])
        } else {
            None
        }
    }

    fn peak_random(&mut self, rand: &mut Random) -> Option<T> {
        if self.s.len() > 0 {
            self.r = Some(rand.rand_usize(0, self.s.len()));
            Some(self.s[self.r.expect("This should never panic")])
        } else {
            self.r = None;
            None
        }
    }

    fn peak_oldest(&self) -> Option<T> {
        if self.s.len() > 0 {
            Some(self.s[0])
        } else {
            None
        }
    }

    fn peak_middle(&self) -> Option<T> {
        if self.s.len() > 0 {
            Some(self.s[self.s.len() / 2])
        } else {
            None
        }
    }

    pub fn peak(&mut self, rand: &mut Random) -> Option<T> {
        let c = rand.rand_weights(&self.weights);
        self.c = c;
        match c {
            0 => self.peak_newest(),
            1 => self.peak_random(rand),
            2 => self.peak_oldest(),
            3 => self.peak_middle(),
            _ => panic!("Got an invalid value from weights. This should be unreachable."),
        }
    }

    pub fn push(&mut self, v: T) -> () {
        self.s.push(v);
    }

    fn pop_newest(&mut self) -> Option<T> {
        self.s.pop()
    }

    fn pop_index(&mut self, index: usize) -> Option<T> {
        if self.s.len() > 0 {
            let v = self.s[index];
            self.s.remove(index);
            Some(v)
        } else {
            None
        }
    }

    fn pop_random(&mut self) -> Option<T> {
        match self.r {
            Some(r) => self.pop_index(r),
            None => panic!("Random number not initialized. `self.peak_random()` to initialize."),
        }
    }

    fn pop_oldest(&mut self) -> Option<T> {
        self.pop_index(0)
    }

    fn pop_middle(&mut self) -> Option<T> {
        self.pop_index(self.s.len() / 2)
    }

    pub fn pop(&mut self) -> Option<T> {
        let v = match self.c {
            0 => self.pop_newest(),
            1 => self.pop_random(),
            2 => self.pop_oldest(),
            3 => self.pop_middle(),
            4 => panic!("`self.peak()` to choose from weights"),
            _ => panic!("Got an invalid value from weights. This should be unreachable."),
        };
        self.c = 4;

        v
    }
}

pub fn growing_tree<F>(maze: &mut Maze, rand: &mut Random, weights: Vec<u32>, call_back: &mut F) -> ()
    where F: FnMut(&mut Maze) -> () {
    let p_start = rand.rand_point(maze.width(), maze.height());
    let mut set = Set::new(weights);
    set.push(p_start);
    maze.set_special(p_start);
    maze.set_visited(p_start);
    call_back(maze);
    loop {
        let p = set.peak(rand);
        match p {
            Some(p) => {
                if !super::valid_neighbors(
                    rand, maze, p, false, true,
                    &mut |maze, p_orig, p_new, dir, _rand| {
                        maze.set_pos(Some(p_orig));
                        maze.carve_passage(dir);
                        maze.set_special(p_new);
                        maze.set_visited(p_new);
                        set.push(p_new);
                        call_back(maze);
                    }) {
                    maze.set_pos(Some(p));
                    maze.clear_special(p);
                    set.pop();
                    call_back(maze);
                }
            },
            None => break,
        }
    }
    maze.set_pos(None);
}
