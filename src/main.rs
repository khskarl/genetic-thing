extern crate rand;

use rand::distributions::{IndependentSample, Range};

// Individual Stuff
#[derive(Debug)]
struct Individual<T> {
    genome: Vec<T>,
}

impl<T> Individual<T> {
    fn new(size: u32, range: &Range<T>) -> Individual<T>
        where T: rand::Rand + rand::distributions::range::SampleRange
    {
        let mut genome: Vec<T> = Vec::new();
        let mut rng = rand::thread_rng();

        for _ in 0..size {
            let value = range.ind_sample(&mut rng);
            genome.push(value);
        }

        Individual::<T> { genome: genome }
    }
}

// Population Stuff
struct Population<T> {
    individuals: Vec<Individual<T>>,
    range: Range<T>,
}

impl<T> Population<T> {
    fn new(size: u32, genome_size: u32, range: Range<T>) -> Population<T>
        where T: rand::Rand + rand::distributions::range::SampleRange
    {
        let mut individuals: Vec<Individual<T>> = Vec::new();

        for _ in 0..size {
            individuals.push(Individual::<T>::new(genome_size, &range));
        }

        Population::<T> {
            individuals: individuals,
            range: range,
        }
    }

    fn genome_size(&self) -> usize {
        self.individuals.len()
    }
}

// Everything else
fn main() {
    let population_size = 100;
    let genome_size = 10;
    let range = Range::new(-5, 5);
    let population = Population::<i32>::new(population_size, genome_size, range);

    println!("{:?}", population.individuals[0]);
    println!("Length: {}", population.genome_size());
}
