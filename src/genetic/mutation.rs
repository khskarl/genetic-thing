extern crate rand;
use self::rand::Rng;

use std::f32;

pub trait Mutation<T> {
    fn mutate(&mut self, f: &Fn(&mut Vec<T>, f32), probability: f32);
}

impl<T> Mutation<T> for Vec<T> {
    fn mutate(&mut self, f: &Fn(&mut Vec<T>, f32), probability: f32) {
        f(self, probability)
    }
}

////////////////////////
// Mutation functions //
////////////////////////

pub fn bit_flip(genome: &mut Vec<u8>, probability: f32) {
    for gene in genome.iter_mut() {
        if rand::random::<f32>() > probability {
            continue;
        }

        *gene ^= 1;
    }
}

pub fn delta_mutation(genome: &mut Vec<f32>, probability: f32) {
    for gene in genome.iter_mut() {
        if rand::random::<f32>() > probability {
            continue;
        }

        let delta = rand::random::<f32>() * 0.1;
        *gene += delta;
    }
}

pub fn random_int(genome: &mut Vec<i32>, probability: f32) {
    for gene in genome.iter_mut() {
        if rand::random::<f32>() > probability {
            continue;
        }

        *gene = rand::random::<i32>();
    }
}

pub fn swap_position(genome: &mut Vec<i32>, probability: f32) {
    for i in 0..genome.len() {
        if rand::random::<f32>() > probability {
            continue;
        }

        let pair_index = rand::thread_rng().gen_range(0, genome.len());
        
        let old_value = genome[i];
        genome[i] = genome[pair_index];
        genome[pair_index] = old_value;        
    }
}

fn gaussian(mean: f32, deviation: f32) -> f32 {
    let mut x1 = rand::random::<f32>();
    let mut x2 = rand::random::<f32>();

    if x1 == 0.0 {
        x1 = 1.0;
    }
    if x2 == 0.0 {
        x2 = 1.0;
    }

    let y1 = (-2.0 * x1.ln()).sqrt() * (2.0 * f32::consts::PI * x2).cos();
    y1 * deviation + mean
}

// TODO: Test the dank out of this function
fn gaussian_mutation(genome: &mut Vec<f32>, probability: f32) {
    for i in 0..genome.len() {
        if rand::random::<f32>() > probability {
            continue;
        }

        genome[i] = gaussian(genome[i], 0.1);
    }
}
