extern crate rand;

use std::fmt;
use std::cmp;
use std::collections::HashSet;

pub use self::rand::distributions::Range;
use self::rand::SeedableRng;
use self::rand::distributions::IndependentSample;

// Individual Stuff
#[derive(Debug)]
pub struct Individual<T> {
    pub genome: Vec<T>,
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

// impl fmt::Display for Individual<bool> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         for i in self.genome {
//             write!(f, "{}", i)
//         }
//     }
// }

// Population Stuff
pub struct Population<T> {
    pub individuals: Vec<Individual<T>>,
    fitnesses: Vec<f32>,
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
        let mut fitnesses: Vec<f32> = Vec::new();

        for _ in 0..size {
            individuals.push(Individual::<T>::new(genome_size, &range));
            fitnesses.push(0.0);
        }

        Population::<T> {
            individuals: individuals,
            fitnesses: fitnesses,
            range: range,
            genome_length: genome_size,
        }
    }

    fn iterate_generation(&self) {}

    // TODO: Optimize this function to make temporary copy the shorter old slice
    //currently it only the left slice regardless of length.
    fn crossover(&mut self, index_dad: usize, index_mom: usize) {
        let range = Range::new(1, self.genome_length - 1);
        let mut rng = rand::thread_rng();
        let point = range.ind_sample(&mut rng);

        let max_index = cmp::max(index_dad, index_mom);
        let min_index = cmp::min(index_dad, index_mom);

        let (split_left, split_right) = self.individuals.split_at_mut(max_index);
        let (dad, mom) = (&mut split_left[min_index], &mut split_right[0]);

        let old_left_slice_dad = Vec::from(&dad.genome[0..point + 1]);
        dad.genome[0..point + 1].copy_from_slice(&mom.genome[0..point + 1]);
        mom.genome[0..point + 1].copy_from_slice(&old_left_slice_dad);
    }

    // FIXME: This may not be working 100%
    fn tournament(&self, k: usize) -> usize {
        let range = Range::new(0, self.individuals.len());

        let mut biggest: usize = 0;
        let mut processed_candidates = HashSet::<usize>::new();
        let mut rng = rand::thread_rng();
        while processed_candidates.len() < k {
            let picked = range.ind_sample(&mut rng);

            if processed_candidates.contains(&picked) {
                continue;
            }

            processed_candidates.insert(picked);

            if self.fitnesses[picked] > self.fitnesses[biggest] {
                biggest = picked;
            }
        }

        biggest
    }

    fn roulette(&self) -> usize {
        let chance = rand::random::<f32>();
        let sum = self.fitnesses.iter().fold(0.0, |acc, &x| acc + x);
        
        let mut winner: usize = 0;
        let mut last_probability = 0.0;
        for i in 0..self.fitnesses.len() {
            let fitness = self.fitnesses[i];
            let probability = fitness / sum + last_probability;

            if chance < probability {
                winner = i;
                break;
            }
            
            last_probability = probability; 
        }
        winner

    }
}
