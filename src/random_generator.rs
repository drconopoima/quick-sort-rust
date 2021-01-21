use std::num::Wrapping;
/// Pseudo-random number generator based on Lehmer algorithm
/// Source https://lemire.me/blog/2019/03/19/the-fastest-conventional-random-number-generator-that-can-pass-big-crush/
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref RG: Mutex<RandGen> = Mutex::new(RandGen::new(34052));
}

pub fn rand(max: usize) -> usize {
    RG.lock().unwrap().lehmer(max)
}

pub struct RandGen {
    current: usize,
    multiplier: Wrapping<u128>,
    big_int: Wrapping<u128>,
}

impl RandGen {
    pub fn new(current: usize) -> Self {
        RandGen {
            current,
            multiplier: Wrapping(0xa3b195354a39b70du128),
            big_int: Wrapping(0xda942042e4dd58b5u128),
        }
    }
    pub fn lehmer(&mut self, max_value: usize) -> usize {
        self.big_int = self.big_int * self.multiplier;
        self.current = (self.big_int.0 >> 64) as usize;
        self.current % max_value
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_rands_printout() {
        use super::*;
        let mut rand_gen = RandGen::new(12);
        let mut value;
        for _ in 0..100 {
            value = rand_gen.lehmer(100);
            // println!("{:?}", value);
            assert!(value < 100)
        }
        // panic!()
    }
}
