extern crate rand;

use rand::Rng;

pub fn roll(n: i32, size: i32) -> i32{
    let mut rng = rand::thread_rng();
    let mut result = 0;
    for i in 1..=n {
        result += rng.gen_range(1,size+1);
    }
    return result;
}
