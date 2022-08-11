extern crate rand;

use rand::rngs::ThreadRng;
use rand::thread_rng;
use rand::Rng;

use crate::point::Point;

pub struct Random {
    rng: ThreadRng,
}

impl Random {
    pub fn new() -> Random {
        Random {rng: thread_rng()}
    }

    pub fn rand_usize(&mut self, lo: usize, hi: usize) -> usize {
        let r: f64 = self.rng.gen();
        (r * ((hi - lo) as f64)) as usize + lo
    }

    pub fn rand_point(&mut self, width: usize, height: usize) -> Point {
        Point::new(self.rand_usize(0, width), self.rand_usize(0, height))
    }

    pub fn shuffle<T: Copy>(&mut self, v: &mut Vec<T>) {
        let l = v.len();
        for i in 0..l {
            let j: usize = self.rand_usize(i, l);
            let temp = v[j];
            v[j] = v[i];
            v[i] = temp;
        }
    }
}
