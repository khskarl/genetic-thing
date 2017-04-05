extern crate rand;

use self::rand::distributions::Range;
use self::rand::SeedableRng;
use self::rand::distributions::IndependentSample;

/////////////////////////
// Crossover functions //
/////////////////////////

pub fn one_point_crossover<T>(dad_genome: &mut Vec<T>, mom_genome: &mut Vec<T>)
    where T: Copy
{
    let range = Range::new(1, dad_genome.len() - 1);
    let mut rng = rand::thread_rng();
    let point = range.ind_sample(&mut rng);

    let old_left_slice_dad = Vec::from(&dad_genome[0..point + 1]);
    dad_genome[0..point + 1].copy_from_slice(&mom_genome[0..point + 1]);
    mom_genome[0..point + 1].copy_from_slice(&old_left_slice_dad);
}
