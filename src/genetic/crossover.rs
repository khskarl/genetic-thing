extern crate rand;

use self::rand::distributions::Range;
use self::rand::SeedableRng;
use self::rand::distributions::IndependentSample;

/////////////////////////
// Crossover functions //
/////////////////////////

pub fn one_point_crossover<T>(dad_genome: &Vec<T>, mom_genome: &Vec<T>) -> (Vec<T>, Vec<T>)
    where T: Copy
{
    let range = Range::new(1, dad_genome.len() - 1);
    let mut rng = rand::thread_rng();
    let point = range.ind_sample(&mut rng);

    let mut boy_genome = dad_genome.clone();
    let mut girl_genome = mom_genome.clone();
    boy_genome[0..point + 1].copy_from_slice(&mom_genome[0..point + 1]);
    girl_genome[0..point + 1].copy_from_slice(&dad_genome[0..point + 1]);

    (boy_genome, girl_genome)
}
