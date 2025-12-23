fn main() {
    // Arrays have fixed size known at compile time
    let mut array_7_floats = [0.1; 7];
    array_7_floats[0] = 4.66;
    array_7_floats[1] = 453.5;
    array_7_floats[2] = 3.6454;

    // Vectors are dynamic and can grow at runtime
    let mut vector_dynamic_i32_numbers = vec![4, 3, 2, 7];
    
    // You can also add more elements
    vector_dynamic_i32_numbers.push(10);

    println!("second element of array: {}", array_7_floats[1]);
    println!("fourth element of array: {}", array_7_floats[3]);
    println!("second element of vector: {}", vector_dynamic_i32_numbers[1]);
    println!("full array: {:?}", array_7_floats);
    println!("full vector: {:?}", vector_dynamic_i32_numbers);
}
