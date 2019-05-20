fn main() {
  println!("Mutation Exercises");
  let mut x = 5;
	x = x + 10;
	let y = &mut x;
	println!("The value of 'y' is {}", y);
	println!("The value of 'x' is {}", x);
}
