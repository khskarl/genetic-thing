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

struct Point {
    x: usize,
    y: usize
}

fn walk(genome: &Vec<i32>, from: Point, to: Point) -> (usize, usize, usize, usize) {
    let maze = vec![[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
                    [0,0,1,1,0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,0,1,1,1,0,0],
                    [0,1,1,1,1,1,1,1,1,0,1,0,1,0,1,0,0,0,0,0,0,0,1,1,0],
                    [0,1,0,0,0,0,0,0,1,0,1,0,1,0,1,0,0,0,0,0,0,0,1,0,0],
                    [0,1,1,1,1,1,1,0,1,0,1,0,1,0,1,0,1,1,1,0,1,1,1,1,0],
                    [0,1,0,0,0,0,1,0,1,0,1,0,1,0,1,0,1,0,0,0,0,0,0,1,0],
                    [0,1,0,0,0,0,1,0,1,1,1,1,1,1,1,0,1,1,1,1,1,0,1,1,0],
                    [0,1,0,0,0,0,1,0,0,0,0,0,0,0,1,0,1,0,1,0,1,0,0,1,0],
                    [0,1,1,1,1,1,1,1,0,1,1,0,1,1,1,0,1,0,1,0,1,0,1,1,0],
                    [0,0,0,0,0,0,1,1,0,1,1,0,1,1,1,0,1,0,1,0,1,0,0,1,0],
                    [0,1,1,1,1,0,1,0,0,1,1,0,1,0,0,0,1,0,1,0,1,0,1,1,0],
                    [0,1,0,0,1,0,1,0,0,1,1,0,1,0,0,0,1,0,1,0,1,1,1,1,0],
                    [0,1,0,0,1,0,1,0,0,1,0,0,1,0,0,0,1,0,1,0,0,0,0,1,0],
                    [0,1,0,0,1,0,1,1,0,1,0,1,1,1,1,1,1,0,1,0,1,1,1,1,0],
                    [0,1,0,0,1,0,1,1,0,1,0,0,0,0,0,0,0,0,1,0,1,0,0,1,0],
                    [0,1,1,0,1,0,0,1,1,1,0,0,0,0,0,1,1,1,1,0,1,0,0,1,0],
                    [0,1,1,0,1,0,0,1,1,1,1,1,1,1,1,1,0,0,0,0,1,0,1,1,0],
                    [0,0,1,0,1,0,1,1,0,0,0,0,0,0,0,1,1,1,1,0,1,0,1,0,0],
                    [0,1,1,0,0,0,1,1,0,1,1,1,1,0,0,0,0,0,1,0,1,1,1,1,0],
                    [0,1,1,1,1,1,1,1,0,1,1,1,1,1,1,1,1,1,1,0,1,0,0,1,0],
                    [0,0,0,0,1,0,0,0,0,1,1,0,1,1,1,0,1,0,1,0,1,1,0,1,0],
                    [0,1,1,1,1,0,1,1,1,1,1,0,1,0,1,0,1,0,1,0,0,1,0,1,0],
                    [0,1,1,0,1,0,1,0,0,0,1,0,1,0,1,0,1,0,1,0,1,1,0,1,0],
                    [0,1,1,0,1,0,1,0,0,0,1,0,1,0,1,0,1,0,1,0,1,0,0,1,0],
                    [0,1,1,0,1,0,1,0,0,0,1,0,1,0,1,0,1,0,1,0,1,1,0,1,0],
                    [0,1,1,0,1,0,1,0,0,0,1,0,1,0,1,0,1,0,0,0,0,1,0,1,0],
                    [0,0,0,0,1,0,1,1,1,1,1,1,1,1,1,0,1,0,0,1,1,1,1,1,0],
                    [0,1,1,1,1,0,1,0,0,0,0,0,0,0,0,0,1,0,0,1,0,0,1,0,0],
                    [0,1,1,0,1,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
                    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]];

    let (mut curr_x, mut curr_y) = (from.x, from.y);
    let (last_x, last_y) = (to.x, to.y);

    let mut visited_positions = vec![(curr_x, curr_y)];
    let mut num_steps = 0;
    let mut num_repeated_steps = 0;
    let mut num_bad_steps = 0;
    let mut max_consecutive_steps = 0;
    let mut num_consecutive_steps = 0;
    for i in 0..genome.len() {
        num_steps += 1;
        let direction = genome[i];
        match direction {
            0 => { if curr_y > 0  { curr_y -= 1 } },
            1 => { if curr_x < 24 { curr_x += 1 } },
            2 => { if curr_y < 28 { curr_y += 1 } },
            3 => { if curr_x > 0  { curr_x -= 1 } },
            x => panic!("Unexpected invalid value {:?}", x)            
        }
        
        if visited_positions.contains(&(curr_x, curr_y)) {
            num_repeated_steps += 1;
        }
        visited_positions.push((curr_x, curr_y));
        
        let value = maze[curr_y][curr_x];
        match value {
            0 => {
                num_bad_steps += 1;
                if max_consecutive_steps < num_consecutive_steps {
                    max_consecutive_steps = num_consecutive_steps;
                }
                num_consecutive_steps = 0;
            },
            1 => {
                num_consecutive_steps += 1;
            },
            x => panic!("Unexpected invalid value {:?}", x)
        }

        if curr_x == last_x && curr_y == last_y {
            break;
        }
        
    }

    if curr_x != last_x && curr_y != last_y {
        num_steps = genome.len();
    }

    (num_steps, num_repeated_steps, num_bad_steps, max_consecutive_steps)
}

pub fn path_fitness(genome: &Vec<i32>, range: &Range<i32>) -> f32 {
    let start_position = Point {x:1, y:10};
    let end_position = Point {x:21, y:1};
    let (num_steps,
         num_repeated_steps,
         num_bad_steps,
         num_consecutive_steps) = walk(genome, start_position, end_position);

    let num_max_steps = genome.len();
    let penalty = (num_steps + num_repeated_steps + num_bad_steps) as f32;
    let fitness = (num_consecutive_steps * 3 + num_max_steps * 3) as f32;
    (fitness - penalty) as f32
}
