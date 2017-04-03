
pub fn BinaryVectorToDecimal(binary_vector: &Vec<u8>) -> i32 {
    let mut decimal: i32 = 0;
    let size = binary_vector.len() as u32 - 1;
    for i in 0..binary_vector.len() {
        if binary_vector[i] == 1 {
            decimal += 2i32.pow(size  - i as u32);
        }
    }
    decimal
}
