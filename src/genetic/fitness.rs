use genetic::helpers::binary_vector_to_decimal;

use std::f32;
use std::ops::Add;

pub trait HasFitness<T> {
    fn fitness(&self, f: &Fn(&T) -> f32) -> f32;
}

impl<T> HasFitness<T> for T {
    fn fitness(&self, f: &Fn(&T) -> f32) -> f32 {
        f(&self)
    }
}

///////////////////////
// Fitness functions //
///////////////////////

pub fn max_alternating_bits(genome: &Vec<u8>) -> f32 {
    let mut was_zero = genome[0] == 0;
    let mut fitness = 0;

    for gene in genome {
        let is_zero = *gene == 0;
        if was_zero != is_zero {
            fitness += 1;
        }
        was_zero = is_zero;
    }

    fitness as f32
}

pub fn max_alternating_even_odd(genome: &Vec<i32>) -> f32 {
    let mut was_even = genome[0] == 0;
    let mut fitness = 0;

    for gene in genome {
        let is_even = *gene % 2 == 0;
        if was_even != is_even {
            fitness += 1;
        }
        was_even = is_even;
    }

    fitness as f32
}

pub fn min_algebraic_function(genome: &Vec<f32>) -> f32 {
    let mut fitness = 0.0;

    for gene in genome {
        fitness += gene * gene;
    }

    fitness
}

// Parps Fitness
fn parps_function(x: f32) -> f32 {
    (x * 20.0).cos() - x.abs() / 2.0 + (x * x * x) / 4.0
}

pub fn parps_fitness(binary_genome: &Vec<u8>) -> f32 {
    let decimal: i32 = binary_vector_to_decimal(binary_genome);
    let limit = (2i32.pow(16) - 1) as f32;
    let f = -2.0 + (4.0 / limit) * (decimal as f32);
    let fitness = parps_function(f) + 4.0;
    fitness
}
