extern crate rand;

use std::fmt;
use std::cmp;
use std::collections::HashSet;

pub use self::rand::distributions::Range;
use self::rand::distributions::IndependentSample;

use genetic::fitness::HasFitness;
use genetic::mutation::Mutation;

use genetic::helpers::SimpleStepRange;

// Individual Stuff
#[derive(Debug, Clone)]
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

     // Population Stuff
     pub struct Population<T> {
         pub individuals: Vec<Individual<T>>,
         pub fitnesses: Vec<f32>,
         pub best_individual_in_generation: Vec<Individual<T>>,
         pub best_fitness_in_generation: Vec<f32>,
         pub average_fitness_in_generation: Vec<f32>,
         genome_length: usize,
         crossover_probability: f32,
         mutation_probability: f32,
         has_elitism: bool,
         range: Range<T>,

         fitness_function: fn(&Vec<T>) -> f32,
         crossover_function: fn(&Vec<T>, &Vec<T>) -> (Vec<T>, Vec<T>),
         mutation_function: fn(&mut Vec<T>, f32),
     }

     impl<T> Population<T>
     where T: Copy
     {
         pub fn new(size: usize,
                    genome_size: usize,
                    crossover_probability: f32,
                    mutation_probability: f32,
                    range: Range<T>,
                    has_elitism: bool,
                    fitness_function: fn(&Vec<T>) -> f32,
                    crossover_function: fn(&Vec<T>, &Vec<T>) -> (Vec<T>, Vec<T>),
                    mutation_function: fn(&mut Vec<T>, f32))
                    -> Population<T>
             where T: rand::Rand + rand::distributions::range::SampleRange
         {
             let mut individuals: Vec<Individual<T>> = Vec::new();
             let mut fitnesses: Vec<f32> = Vec::new();

             for i in 0..size {
                 individuals.push(Individual::<T>::new(genome_size, &range));
                 fitnesses.push(individuals[i].genome.fitness(&fitness_function));
             }

             Population::<T> {
                 individuals: individuals,
                 fitnesses: fitnesses,
                 best_individual_in_generation: Vec::<Individual<T>>::new(),
                 best_fitness_in_generation: Vec::<f32>::new(),
                 average_fitness_in_generation: Vec::<f32>::new(),
                 range: range,
                 crossover_probability: crossover_probability,
                 mutation_probability: mutation_probability,
                 genome_length: genome_size,
                 has_elitism: has_elitism,

                 fitness_function: fitness_function,
                 crossover_function: crossover_function,
                 mutation_function: mutation_function,
             }
         }

         pub fn iterate_generation(&mut self) {
             let fittest_index = self.get_fittest_individual();
             let fittest_individual = self.individuals[fittest_index].clone();
             let fittest_fitness = self.fitnesses[fittest_index];

             let mut new_individuals = self.individuals.clone();
             for _ in SimpleStepRange(0, self.individuals.len(), 2) {
                 if rand::random::<f32>() > self.crossover_probability {
                     continue;
                 }

                 let dad_index = self.select_fit_individual();
                 let mom_index = self.select_fit_individual_except(dad_index);

                 //println!("dad: {}, mom: {}", dad_index, mom_index);
                 let (boy_genome, girl_genome) = self.crossover(dad_index, mom_index);
                 let (worst_index, second_worst_index) = self.get_weakest_couple();
                 new_individuals[worst_index].genome.clone_from(&boy_genome);
                 new_individuals[second_worst_index].genome.clone_from(&girl_genome);
             }
             self.individuals.clone_from(&new_individuals);

             for individual in &mut self.individuals {
                 individual.genome.mutate(&self.mutation_function, self.mutation_probability);
             }

             self.compute_fitnesses();
             // FIXME: Maybe getting the wrong worst individual
             if self.has_elitism {
                 let (weakest_index, _) = self.get_weakest_couple();
                 self.individuals[weakest_index] = fittest_individual.clone();
                 self.fitnesses[weakest_index] = fittest_fitness;
             }

             // Save average and best fitness in this generation
             {
                 let mut best_fitness = 0.0;
                 let mut best_individual = self.individuals[0].clone();
                 let mut sum_fitnesses = 0.0;
                 for i in 0..self.fitnesses.len() {
                     sum_fitnesses += self.fitnesses[i];
                     if self.fitnesses[i] > best_fitness {
                         best_fitness = self.fitnesses[i];
                         best_individual = self.individuals[i].clone();
                     }
                 }
                 let avg_fitness = sum_fitnesses / (self.fitnesses.len() as f32);

                 self.average_fitness_in_generation.push(avg_fitness);
                 self.best_individual_in_generation.push(best_individual);
                 self.best_fitness_in_generation.push(best_fitness);
             }


         }

         fn select_fit_individual(&self) -> usize {
             self.roulette()
         }

         fn select_fit_individual_except(&self, dad_index: usize) -> usize {
             let mut mom_index: usize;

             loop {
                 mom_index = self.roulette();

                 if mom_index != dad_index {
                     break;
                 }
             }
             mom_index
         }

         fn get_fittest_individual(&self) -> usize {
             let mut fittest_index = 0;

             for i in 1..self.individuals.len() {
                 if self.fitnesses[i] > self.fitnesses[fittest_index] {
                     fittest_index = i;
                 }
             }
             fittest_index
         }

         fn get_weakest_couple(&self) -> (usize, usize) {
             let mut weakest_index = 0;
             let mut second_weakest_index = 0;

             for i in 1..self.individuals.len() {
                 if self.fitnesses[i] < self.fitnesses[weakest_index] {
                     second_weakest_index = weakest_index;
                     weakest_index = i;
                 }
             }
             (weakest_index, second_weakest_index)
         }

         fn compute_fitnesses(&mut self) {
             for i in 0..self.individuals.len() {
                 self.fitnesses[i] = self.individuals[i].genome.fitness(&self.fitness_function);
             }
         }

         fn crossover(&mut self, index_dad: usize, index_mom: usize) -> (Vec<T>, Vec<T>) {
             let max_index = cmp::max(index_dad, index_mom);
             let min_index = cmp::min(index_dad, index_mom);

             let (split_left, split_right) = self.individuals.split_at_mut(max_index);
             let (dad, mom) = (&mut split_left[min_index], &mut split_right[0]);

             let (boy_genome, girl_genome) = (self.crossover_function)(&dad.genome, &mom.genome);

             (boy_genome, girl_genome)
         }

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
