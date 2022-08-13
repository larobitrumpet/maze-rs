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

    /// Returns a random value based on weights.
    ///
    /// `v` is a vector of weights and the last
    /// element is the sum of the weights.
    pub fn rand_weights(&mut self, v: &Vec<u32>) -> usize {
        if v[v.len() - 1] == 1 {
            for (index, &value) in v.iter().enumerate() {
                if value == 1 {
                    return index
                }
            }
        }
        let r = self.rand_usize(0, v[v.len() - 1] as usize);
        let mut sum = 0;
        for (index, value) in v[..(v.len() - 2)].iter().enumerate() {
            if r < (value + sum) as usize {
                return index;
            }
            sum += value;
        }

        v.len() - 2 as usize
    }
}
