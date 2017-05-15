extern crate rand;
pub use self::rand::distributions;
use self::rand::distributions::IndependentSample;
use self::rand::{thread_rng, Rng};

extern crate num;
use self::num::{Num, Zero, One};
use self::num::traits::pow;

use std::fmt;
use std::cmp;
use std::ops::{Add, Sub, Mul};
use std::collections::HashSet;

use genetic::fitness::HasFitness;
use genetic::mutation::Mutation;

use genetic::helpers::{SimpleStepRange, Range};

// Individual Stuff
#[derive(Debug, Clone)]
pub struct Individual<T> {
    pub genome: Vec<T>,
}

impl<T> Individual<T>
    where T: Copy + PartialOrd
{
    pub fn new(size: usize, range: &Range<T>) -> Individual<T>
        where T: rand::Rand + rand::distributions::range::SampleRange + Add + Num
    {
        let mut genome: Vec<T> = Vec::new();
        let mut rng = rand::thread_rng();
        let rangeDist = distributions::Range::new(range.start, range.end + T::one());
        for _ in 0..size {
            let value = rangeDist.ind_sample(&mut rng);
            genome.push(value);
        }

        Individual::<T> { genome: genome }
    }

    pub fn new_ordered(size: usize) -> Individual<i32>
        where T: rand::Rand + rand::distributions::range::SampleRange + Add + Num
    {
        let mut genome: Vec<i32> = Vec::new();
        for i in 0..size {
            genome.push(i as i32);
        }
        let mut rng = rand::thread_rng();
        rng.shuffle(&mut genome);

        Individual::<i32> { genome: genome }
    }
}

// Population Stuff
pub struct Population<T>
    where T: PartialOrd
{
    pub individuals: Vec<Individual<T>>,
    pub fitnesses: Vec<f32>,
    pub best_individual_in_generation: Vec<Individual<T>>,
    pub best_fitness_in_generation: Vec<f32>,
    pub average_fitness_in_generation: Vec<f32>,
    pub diversity_in_generation: Vec<f32>,
    genome_length: usize,
    crossover_probability: f32,
    mutation_probability: f32,
    has_elitism: bool,
    range: Range<T>,

    diversity_function: fn(&Vec<T>, &Vec<T>) -> f32,
    fitness_function: fn(&Vec<T>, &Range<T>) -> f32,
    crossover_function: fn(&Vec<T>, &Vec<T>) -> (Vec<T>, Vec<T>),
    mutation_function: fn(&mut Vec<T>, f32, &Range<T>),
}

impl<T> Population<T>
    where T: Copy + PartialOrd + Num
{
    pub fn new(size: usize,
               genome_size: usize,
               crossover_probability: f32,
               mutation_probability: f32,
               range: Range<T>,
               has_elitism: bool,
               diversity_function: fn(&Vec<T>, &Vec<T>) -> f32,
               fitness_function: fn(&Vec<T>, &Range<T>) -> f32,
               crossover_function: fn(&Vec<T>, &Vec<T>) -> (Vec<T>, Vec<T>),
               mutation_function: fn(&mut Vec<T>, f32, &Range<T>))
               -> Population<T>
        where T: rand::Rand + rand::distributions::range::SampleRange
    {
        let mut individuals: Vec<Individual<T>> = Vec::new();
        let mut fitnesses: Vec<f32> = Vec::new();

        for i in 0..size {
            individuals.push(Individual::<T>::new(genome_size, &range));
            fitnesses.push(individuals[i].genome.fitness(&fitness_function, &range));
        }
    
        Population::<T> {
            individuals: individuals,
            fitnesses: fitnesses,
            best_individual_in_generation: Vec::<Individual<T>>::new(),
            best_fitness_in_generation: Vec::<f32>::new(),
            average_fitness_in_generation: Vec::<f32>::new(),
            diversity_in_generation: Vec::<f32>::new(),
            range: range,
            crossover_probability: crossover_probability,
            mutation_probability: mutation_probability,
            genome_length: genome_size,
            has_elitism: has_elitism,

            diversity_function: diversity_function,
            fitness_function: fitness_function,
            crossover_function: crossover_function,
            mutation_function: mutation_function,
        }
    }

    pub fn new_ordered(size: usize,
                       genome_size: usize,
                       crossover_probability: f32,
                       mutation_probability: f32,
                       range: Range<i32>,
                       has_elitism: bool,
                       diversity_function: fn(&Vec<i32>, &Vec<i32>) -> f32,
                       fitness_function: fn(&Vec<i32>, &Range<i32>) -> f32,
                       crossover_function: fn(&Vec<i32>, &Vec<i32>) -> (Vec<i32>, Vec<i32>),
                       mutation_function: fn(&mut Vec<i32>, f32, &Range<i32>))
                       -> Population<i32>
        where T: rand::Rand + rand::distributions::range::SampleRange
    {
        let mut population = Population::<i32>::new(size,
                                                    genome_size,
                                                    crossover_probability,
                                                    mutation_probability,
                                                    range,
                                                    has_elitism,
                                                    diversity_function,
                                                    fitness_function,
                                                    crossover_function,
                                                    mutation_function);

        {
            let mut individuals = &mut population.individuals;
            let mut fitnesses = &mut population.fitnesses;
            for i in 0..individuals.len() {
                individuals[i] = Individual::<i32>::new_ordered(genome_size);
                fitnesses[i] = individuals[i].genome.fitness(&fitness_function, &range);
            }
        }
        population
            
    }

    pub fn iterate_generation(&mut self, curr_generation: usize, total_generations: usize) {
        let fittest_index = self.get_fittest_individual();
        let fittest_individual = self.individuals[fittest_index].clone();
        let fittest_fitness = self.fitnesses[fittest_index];

        let mut new_individuals = Vec::new();

        for _ in 0..self.individuals.len() {
            let fit_index = self.select_fit_individual();
            new_individuals.push(self.individuals[fit_index].clone());
        } 
        
        for _ in SimpleStepRange(0, self.individuals.len(), 2) {
            if rand::random::<f32>() > self.crossover_probability {
                continue;
            }

            let dad_index = self.select_fit_individual();
            let mom_index = self.select_fit_individual_except(dad_index);

            //println!("dad: {}, mom: {}", dad_index, mom_index);
            let (boy_genome, girl_genome) = self.crossover(dad_index, mom_index);
            new_individuals[dad_index].genome.clone_from(&boy_genome);
            new_individuals[mom_index].genome.clone_from(&girl_genome);
        }
        self.individuals.clone_from(&new_individuals);

        for individual in &mut self.individuals {
            individual.genome.mutate(&self.mutation_function,
                                     self.mutation_probability,
                                     &self.range);
        }

        self.compute_fitnesses();
        // let factor_current_run = curr_generation as f32 / total_generations as f32;  
        // let c = 1.2 + 0.8 * factor_current_run; 
        // self.linear_scaling(c);
        
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
            let diversity = self.calculate_diversity();
            self.diversity_in_generation.push(diversity);
        }


    }

    fn select_fit_individual(&self) -> usize {
        self.tournament(2)
    }

    fn select_fit_individual_except(&self, dad_index: usize) -> usize {
        let mut mom_index: usize;
        loop {
            mom_index = self.select_fit_individual();

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
            self.fitnesses[i] = self.individuals[i].genome.fitness(&self.fitness_function,
                                                                   &self.range);
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
        let mut biggest: usize = rand::thread_rng().gen_range(0, self.individuals.len());
        let mut processed_candidates = HashSet::<usize>::new();
        processed_candidates.insert(biggest); 
        let mut rng = rand::thread_rng();

        while processed_candidates.len() < k - 1 {
            let picked = rng.gen_range(0, self.individuals.len());

            if processed_candidates.contains(&picked) {
                continue;
            }

            processed_candidates.insert(picked);

            if self.fitnesses[picked] >= self.fitnesses[biggest] {
                biggest = picked;
            }
        }

        biggest
    }

    fn calculate_diversity(&self) -> f32 {
        let mut total_diversity = 0.0;

        for i in 0..self.individuals.len() {
            for j in i..self.individuals.len() {
                total_diversity += (self.diversity_function)(&self.individuals[i].genome,
                                                             &self.individuals[j].genome);
            }
        }

        total_diversity
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

    fn linear_scaling(&mut self, c: f32) {
        let sum = self.fitnesses.iter().fold(0.0, |acc, &x| acc + x);
        let average = sum / self.fitnesses.len() as f32;

        let mut min = self.fitnesses[0];
        let mut max = self.fitnesses[0];        
        for fitness in &self.fitnesses {
            if *fitness < min {
                min = *fitness;
            }

            if *fitness > max {
                max = *fitness;
            }
        }

        let decider = (c * (average - max)) / (c - 1.0);
        let alpha: f32;
        let beta: f32;

        if min > decider {
            alpha = (average * (c - 1.0)) / (max - average);
            beta  = (average * (max - c * average)) / (max - average); 
        } else {
            alpha = average / (average - min);
            beta = (-min * average) / (average - min);
        }

        for i in 0..self.fitnesses.len() {
            self.fitnesses[i] = alpha * self.fitnesses[i] + beta;
        }        
    } 
    

    pub fn print(&self)
        where T: fmt::Debug
    {
        for i in 0..self.individuals.len() {
            let individual = &self.individuals[i];
            let fitness = self.fitnesses[i];
            println!("{:?} : {}", individual.genome, fitness);
            // println!("Best vector: {:?}", population.best_fitness_in_generation);
            // println!("Average vector: {:?}", population.average_fitness_in_generation);
            // if let Some(best_fitness) = population.best_fitness_in_generation.last() {
            //     println!("The Best: {}", best_fitness);
            // }
        }

    }

    pub fn print_best_individual_diagnostic(&self)
        where T: fmt::Debug
    {
        if let Some(best_individual) = self.best_individual_in_generation.last() {
            if let Some(best_fitness) = self.best_fitness_in_generation.last() {
                println!("Best genome: {:?} : {}", best_individual.genome, best_fitness);
            }

            let mut is_valid = true;

            for gene in &best_individual.genome {
                if *gene > self.range.end || *gene < self.range.start {
                    is_valid = false;
                }
            }

            if is_valid == false {
                println!("***Invalid best individual***");
            }
            
        }

        //println!("Diversity test! {}", self.compute_diversity());
    } 
}

