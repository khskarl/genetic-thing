extern crate rand;
use self::rand::distributions::IndependentSample;
pub use self::rand::distributions::Range;

// Individual Stuff
#[derive(Debug)]
pub struct Individual<T> {
    genome: Vec<T>,
}

impl<T> Individual<T> {
    pub fn new(size: u32, range: &Range<T>) -> Individual<T>
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
pub struct Population<T> {
    pub individuals: Vec<Individual<T>>,
    range: Range<T>,
}

impl<T> Population<T> {
    pub fn new(size: u32, genome_size: u32, range: Range<T>) -> Population<T>
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

    pub fn genome_size(&self) -> usize {
        self.individuals.len()
    }
}
