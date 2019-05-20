fn main() {
  let mut array_7_floats: [f64;7] = [0.1;7];
  array_7_floats[0] = 4.66;
  array_7_floats[1] = 453.5;
  array_7_floats[2] = 3.6454;

  let mut vector_dynamic_i32_numbers: Vec<i32> = vec!();
  vector_dynamic_i32_numbers.push(4);
  vector_dynamic_i32_numbers.push(3);
  vector_dynamic_i32_numbers.push(2);
  vector_dynamic_i32_numbers.push(7);

  println!("second element of array: {:?}", array_7_floats[1]);
  println!("fourth element of array: {:?}", array_7_floats[3]);
  println!("second element of vector: {:?}", vector_dynamic_i32_numbers[1]);
}
