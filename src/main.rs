extern crate rand;

use rand::Rng;
use std::env;

// Individual Stuff
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

// Population Stuff
struct Population<T> {
    individuals: Vec<Individual<T>>,
}

impl<T> Population<T> {
    fn new(size: u32, genome_size: u32) -> Population<T>
        where T: rand::Rand
    {
        let mut individuals: Vec<Individual<T>> = Vec::new();

        for _ in 0..size {
            individuals.push(Individual::<T>::new(genome_size));
        }

        Population::<T> { individuals: individuals }
    }

    fn genome_size(&self) -> usize {
        self.individuals.len()
    }
}

// Everything else
fn main() {
    let population_size = 100;
    let genome_size = 10;
    let population = Population::<bool>::new(population_size, genome_size);

    println!("{:?}", population.individuals[0]);
    println!("Length: {}", population.genome_size());
}
