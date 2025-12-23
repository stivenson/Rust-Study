fn main() {
    println!("Mutation Exercises");
    let mut x = 5;
    x += 10;
    let y = &mut x;
    *y += 5; // Modify through mutable reference
    println!("The value of 'y' (reference) is {}", y);
    println!("The value of 'x' is {}", x);
}
