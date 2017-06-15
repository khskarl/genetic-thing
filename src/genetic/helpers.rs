#[derive(Copy, Clone)]
pub struct Range<T> {
    pub start: T,
    pub end: T
}

impl<T> Range<T> {
    pub fn new(start: T, end: T) -> Range<T> {
        Range::<T> {
            start: start,
            end: end
        }
    }
}

pub fn binary_vector_to_decimal(binary_vector: &Vec<u8>) -> i32 {
    let mut decimal: i32 = 0;
    let size = binary_vector.len() as u32 - 1;
    for i in 0..binary_vector.len() {
        if binary_vector[i] == 1 {
            decimal += 2i32.pow(size  - i as u32);
        }
    }
    decimal
}

pub fn hamming_distance(genome_one: &Vec<u8>, genome_two: &Vec<u8>, range: &Range<u8>) -> f32 {
    let mut total_distance = 0;
    for i in 0..genome_one.len() {
        if genome_one[i] != genome_two[i] {
            total_distance += 1;
        }
    }
    let normalized_distance = total_distance as f32 / genome_two.len() as f32;
    normalized_distance
}

pub fn euclidean_distance_int(genome_one: &Vec<i32>, genome_two: &Vec<i32>, range: &Range<i32>) -> f32
{
    let mut total_distance = 0;

    for i in 0..genome_one.len() {
        total_distance += (genome_one[i] - genome_two[i]).pow(2);
    }
    let max_distance = genome_one.len() as i32 * (range.end - range.start).pow(2);
    (total_distance as f32 / max_distance as f32).sqrt() 
}

pub fn euclidean_distance_float(genome_one: &Vec<f32>, genome_two: &Vec<f32>, range: &Range<f32>) -> f32
{
    let mut total_distance: f32 = 0.0;

    for i in 0..genome_one.len() {
        total_distance += (genome_one[i] - genome_two[i]).powf(2.0);
    }

    let max_distance = genome_one.len() as f32 * ((range.end - range.start).powf(2.0) as f32).sqrt();
    (total_distance).sqrt() / max_distance
}


// Tack sa mycket stackoverflow
// http://stackoverflow.com/questions/27893223/how-do-i-iterate-over-a-range-with-a-custom-step
pub struct SimpleStepRange(pub usize, pub usize, pub usize);  // start, end, and step

impl Iterator for SimpleStepRange {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<usize> {
        if self.0 < self.1 {
            let v = self.0;
            self.0 = v + self.2;
            Some(v)
        } else {
            None
        }
    }
}
