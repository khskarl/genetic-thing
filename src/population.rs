extern crate rand;
use self::rand::distributions::IndependentSample;
pub use self::rand::distributions::Range;
use std::cmp;

// Individual Stuff
#[derive(Debug)]
pub struct Individual<T> {
    genome: Vec<T>,
}

impl<T> Individual<T> {
    pub fn new(size: usize, range: &Range<T>) -> Individual<T>
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
    genome_length: usize,
}

impl<T> Population<T>
    where T: Copy
{
    pub fn new(size: u32, genome_size: usize, range: Range<T>) -> Population<T>
        where T: rand::Rand + rand::distributions::range::SampleRange
    {
        let mut individuals: Vec<Individual<T>> = Vec::new();

        for _ in 0..size {
            individuals.push(Individual::<T>::new(genome_size, &range));
        }

        Population::<T> {
            individuals: individuals,
            range: range,
            genome_length: genome_size,
        }
    }


    pub fn quick(&mut self) {
        self.crossover(0, 1);
    }

    // TODO: Optimize this function to make temporary copy the shorter old slice
    //currently it only the left slice regardless of length.
    fn crossover(&mut self, index_dad: usize, index_mom: usize) {
        let mut rng = rand::thread_rng();
        let range = Range::new(1, self.genome_length - 1);
        let point = range.ind_sample(&mut rng);

        let max_index = cmp::max(index_dad, index_mom);
        let min_index = cmp::min(index_dad, index_mom);

        let (split_left, split_right) = self.individuals.split_at_mut(max_index);
        let (dad, mom) = (&mut split_left[min_index], &mut split_right[0]);

        let old_left_slice_dad = Vec::from(&dad.genome[0..point + 1]);
        dad.genome[0..point + 1].copy_from_slice(&mom.genome[0..point + 1]);
        mom.genome[0..point + 1].copy_from_slice(&old_left_slice_dad);
    }
}
