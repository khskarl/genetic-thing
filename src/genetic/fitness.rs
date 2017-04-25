use genetic::helpers::Range;
use genetic::helpers::binary_vector_to_decimal;
use genetic::helpers::hamming_distance;

pub trait HasFitness<T> {
    fn fitness(&self, f: &fn(&Vec<T>, &Range<T>) -> f32, range: &Range<T>)  -> f32;
}

impl <T> HasFitness<T> for Vec<T> {
    fn fitness(&self, f: &fn(&Vec<T>, &Range<T>) -> f32, range: &Range<T>) -> f32 {
        f(&self, range)
    }
}

///////////////////////
// Fitness functions //
///////////////////////

pub fn max_alternating_bits(genome: &Vec<u8>, range: &Range<u8>) -> f32 {
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

pub fn max_alternating_even_odd(genome: &Vec<i32>, range: &Range<i32>) -> f32 {
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

pub fn min_dejong(genome: &Vec<f32>, range: &Range<f32>) -> f32 {
    let mut fitness = 0.0;
    let mut maximum_value = 0.0;
    
    for gene in genome {
        maximum_value += range.end * range.end;
        fitness += gene * gene;
    }

    1.0 - (fitness / maximum_value)
}

// Parps Fitness
fn parps_function(x: f32) -> f32 {
    (x * 20.0).cos() - x.abs() / 2.0 + (x * x * x) / 4.0
}

pub fn parps_fitness(binary_genome: &Vec<u8>, range: &Range<f32>) -> f32 {
    let decimal: i32 = binary_vector_to_decimal(binary_genome);
    let limit = (2i32.pow(16) - 1) as f32;
    let f = -2.0 + (4.0 / limit) * (decimal as f32);
    let fitness = parps_function(f) + 4.0;
    fitness
}

pub fn pattern_recognition(genome: &Vec<u8>, range: &Range<u8>) -> f32 {
    let pattern: Vec<u8> = [0,1,0,0,0,0,
                            0,1,0,1,1,0,
                            0,1,0,1,0,0,
                            0,0,0,0,1,0,
                            0,1,1,1,0,0,
                            0,0,0,0,1,0].to_vec();
    
    let fit: f32 = hamming_distance(&pattern, &genome);
    36.0 - fit
}

pub fn n_queens(genome: &Vec<i32>, range: &Range<i32>) -> f32 {
    let mut num_diagonal_collisions: usize = 0;

    for i in 0..genome.len() {
        for j in (i+1)..genome.len() {
            
            if (genome[i] - genome[j]).abs() as usize == (j - i) {
                //println!("({},{}) ({},{})", i, genome[i], j, genome[j]);

                num_diagonal_collisions += 1;
                break;
            }
        }
    }

    
    let board_size = genome.len();
    //println!("Boardsize: {} - Num Collisions: {}", board_size, num_diagonal_collisions);
    (board_size - num_diagonal_collisions) as f32 / genome.len() as f32
}
