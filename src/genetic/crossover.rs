extern crate rand;
use self::rand::Rng;


use genetic::helpers::SimpleStepRange;

extern crate num;
use self::num::{Num};

use std::ops::{Add, Div};
use std::fmt::Debug;

/////////////////////////
// Crossover functions //
/////////////////////////

pub fn one_point_crossover<T>(dad_genome: &Vec<T>, mom_genome: &Vec<T>) -> (Vec<T>, Vec<T>)
    where T: Copy
{
    let point = rand::thread_rng().gen_range(1, dad_genome.len() - 1);

    let mut boy_genome = dad_genome.clone();
    let mut girl_genome = mom_genome.clone();
    boy_genome[0..point + 1].copy_from_slice(&mom_genome[0..point + 1]);
    girl_genome[0..point + 1].copy_from_slice(&dad_genome[0..point + 1]);

    (boy_genome, girl_genome)
}

pub fn one_point_crossover_3<T>(dad_genome: &Vec<T>, mom_genome: &Vec<T>) -> (Vec<T>, Vec<T>)
    where T: Copy
{
    let mut point = 1;
    while point % 3 != 0 {
        point = rand::thread_rng().gen_range(2, dad_genome.len() - 3);
    }
    
    let mut boy_genome = dad_genome.clone();
    let mut girl_genome = mom_genome.clone();
    boy_genome[0..point + 1].copy_from_slice(&mom_genome[0..point + 1]);
    girl_genome[0..point + 1].copy_from_slice(&dad_genome[0..point + 1]);

    (boy_genome, girl_genome)
}


pub fn partially_matched_crossover<T>(dad_genome: &Vec<T>, mom_genome: &Vec<T>) -> (Vec<T>, Vec<T>)
    where T: Copy + PartialEq + Debug
{
    let start_index = rand::thread_rng().gen_range(1, dad_genome.len() / 2); 
    let end_index = rand::thread_rng().gen_range(dad_genome.len() / 2, dad_genome.len() - 1);

    let mut boy_genome = dad_genome.clone();
    let mut girl_genome = mom_genome.clone();    
    boy_genome[start_index..end_index].copy_from_slice(&mom_genome[start_index..end_index]);
    girl_genome[start_index..end_index].copy_from_slice(&dad_genome[start_index..end_index]); 

    let mut is_valid_permutation = false;
    while is_valid_permutation == false {
        for i in 0..start_index {
            for j in start_index..end_index {
                if boy_genome[i] == boy_genome[j] {
                    boy_genome[i] = girl_genome[j];
                }

                if girl_genome[i] == girl_genome[j] {
                    girl_genome[i] = boy_genome[j];
                }
            }
        }

        for i in end_index..mom_genome.len() {
            for j in start_index..end_index {
                if boy_genome[i] == boy_genome[j] {
                    boy_genome[i] = girl_genome[j];
                }

                if girl_genome[i] == girl_genome[j] {
                    girl_genome[i] = boy_genome[j];
                }
            } 
        }
        
        is_valid_permutation = true;
        for i in 0..dad_genome.len() {
            for j in (i+1)..dad_genome.len() {
                if boy_genome[i] == boy_genome[j] || girl_genome[i] == girl_genome[j] {
                    is_valid_permutation = false;
                }
            }
        }
    }

    (boy_genome, girl_genome)
}

// TODO: Test the dank out of this function
pub fn uniform_crossover<T>(dad_genome: &Vec<T>, mom_genome: &Vec<T>) -> (Vec<T>, Vec<T>)
    where T: Copy + PartialEq + Num + Div<Output = T> + Add<Output = T>
{
    let mix_ratio = 0.5;

    let mut boy_genome = dad_genome.clone();
    let mut girl_genome = mom_genome.clone();

    for i in 0..mom_genome.len() {
        if rand::random::<f32>() > mix_ratio {
            boy_genome[i] = mom_genome[i];
            girl_genome[i] = dad_genome[i]
        } 
    }

    (boy_genome, girl_genome)
}

// TODO: Test the dank out of this function
pub fn uniform_average_crossover<T>(dad_genome: &Vec<T>, mom_genome: &Vec<T>) -> (Vec<T>, Vec<T>)
    where T: Copy + PartialEq + Num + Div<Output = T> + Add<Output = T>
{
    let mix_ratio = 0.5;

    let mut boy_genome = dad_genome.clone();
    let mut girl_genome = mom_genome.clone();

    for i in 0..mom_genome.len() {
        let average = (dad_genome[i] + mom_genome[i]) / (T::one() + T::one());

        if rand::random::<f32>() > mix_ratio {
            boy_genome[i] = average;
        } else {
            girl_genome[i] = average;
        }
    }

    (boy_genome, girl_genome)
}

// TODO: Test the dank out of this function
pub fn blend_crossover(dad_genome: &Vec<f32>, mom_genome: &Vec<f32>) -> (Vec<f32>, Vec<f32>) {
    let alpha = 0.5;

    let mut boy_genome = Vec::<f32>::new();
    let mut girl_genome = Vec::<f32>::new();

    for i in 0..dad_genome.len() {
        let min: f32;
        let max: f32;
        
        match dad_genome[i] < mom_genome[i] {
            true => {
                min = dad_genome[i];
                max = mom_genome[i];
            }
            false => {
                min = mom_genome[i];
                max = dad_genome[i];
            }
        }

        let d = (dad_genome[i] - mom_genome[i]).abs();
        boy_genome[i] = rand::thread_rng().gen_range(min - alpha * d, max + alpha * d);
        girl_genome[i] = rand::thread_rng().gen_range(min - alpha * d, max + alpha * d);
    }

    (boy_genome, girl_genome)
}

// TODO: Test the dank out of this function
pub fn uniform_crossover_3<T>(dad_genome: &Vec<T>, mom_genome: &Vec<T>) -> (Vec<T>, Vec<T>)
    where T: Copy + PartialEq + Num + Div<Output = T> + Add<Output = T>
{
    let mix_ratio = 0.5;

    let mut boy_genome = dad_genome.clone();
    let mut girl_genome = mom_genome.clone();

    for i in SimpleStepRange(0, mom_genome.len(), 3) {
        if rand::random::<f32>() > mix_ratio {
            boy_genome[i  ] = mom_genome[i  ];
            boy_genome[i+1] = mom_genome[i+1];
            boy_genome[i+2] = mom_genome[i+2];
            
            girl_genome[i  ] = dad_genome[i  ];
            girl_genome[i+1] = dad_genome[i+1];
            girl_genome[i+2] = dad_genome[i+2];
        } 
    }

    (boy_genome, girl_genome)
}
