use genetic::helpers::binary_vector_to_decimal;

use std::f32;

#[derive(PartialOrd, PartialEq, Debug)]
pub enum FitnessType {
    Integer(i32),
    Real(f32),
}

pub trait HasFitness<T> {
    fn fitness(&self, f: &Fn(&T) -> FitnessType) -> FitnessType;
}

impl<T> HasFitness<T> for T {
    fn fitness(&self, f: &Fn(&T) -> FitnessType) -> FitnessType {
        f(&self)
    }
}

///////////////////////
// Fitness functions //
///////////////////////

pub fn alternating_binary_fitness(genome: &Vec<u8>) -> FitnessType {
    let mut was_zero = genome[0] == 0;
    let mut fitness = 0;

    for gene in genome {
        let is_zero = *gene == 0;
        if was_zero != is_zero {
            fitness += 1;
        }
        was_zero = is_zero;
    }

    FitnessType::Integer(fitness)
}

pub fn alternating_even_odd_fitness(genome: &Vec<i32>) -> FitnessType {
    let mut was_even = genome[0] == 0;
    let mut fitness = 0;

    for gene in genome {
        let is_even = *gene % 2 == 0;
        if was_even != is_even {
            fitness += 1;
        }
        was_even = is_even;
    }

    FitnessType::Integer(fitness)
}

pub fn real_fitness(genome: &Vec<f32>) -> FitnessType {
    let mut fitness = 0.0;

    for gene in genome {
        fitness += gene * gene;
    }
    //let fitness = genome.iter().fold(0.0, |acc, &gene| acc + gene * gene);

    FitnessType::Real(fitness)
}

// Parps Fitness
pub fn parps_function(x: f32) -> f32 {
    (x * 20.0).cos() - x.abs() / 2.0 + (x * x * x) / 4.0
}

pub fn parps_fitness(binary_genome: &Vec<u8>) -> FitnessType {
    let decimal: i32 = binary_vector_to_decimal(binary_genome);
    let limit = (2i32.pow(16) - 1) as f32;
    let f = -2.0 + (4.0 / limit) * (decimal as f32);
    let fitness = parps_function(f) + 4.0;
    FitnessType::Real(fitness)
}
