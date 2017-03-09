extern crate rand;

use rand::Rng;
use std::env;

#[derive(Debug)]
struct Individual<T> {
    genome: Vec<T>,
}

impl<T> Individual<T> {
    fn new(size: u32) -> Individual<T>
        where T: rand::Rand
    {
        let mut genome: Vec<T> = Vec::new();
        let mut rng = rand::thread_rng();

        for _ in 0..size {
            let value = rng.gen::<T>();
            genome.push(value);
        }

        Individual::<T> { genome: genome }
    }
}

fn main() {
    let populationSize = 10;
    let individual = Individual::<i32>::new(10);
    
    println!("{:?}", individual);
}
