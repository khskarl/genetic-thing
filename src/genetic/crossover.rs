extern crate rand;
use self::rand::Rng;

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

// Permutation crossovers
// TODO: Test the dank out of this function
pub fn partially_matched_crossover<T>(dad_genome: &Vec<T>, mom_genome: &Vec<T>) -> (Vec<T>, Vec<T>)
    where T: Copy + PartialEq
{
    let start_index = rand::thread_rng().gen_range(1, dad_genome.len() / 2);
    let end_index = rand::thread_rng().gen_range(dad_genome.len() / 2, dad_genome.len() - 1);
    
    let mut boy_genome = dad_genome.clone();
    let mut girl_genome = mom_genome.clone();
    boy_genome[start_index..end_index + 1].copy_from_slice(&mom_genome[start_index..end_index + 1]);
    girl_genome[start_index..end_index + 1].copy_from_slice(&dad_genome[start_index..end_index + 1]);

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
    
    (boy_genome, girl_genome)
}
