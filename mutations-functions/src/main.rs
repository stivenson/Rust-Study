fn main() {
    let s1 = String::from("Stivenson Rincon");
    print_ref_string(&s1); // Sent by reference (borrowing)
    println!("s1: {}", s1); // Still valid after borrowing

    let mut s2_mutable = "Stivenson Rincon".to_string();
    print_ref_mutable_string(&mut s2_mutable); // Sent by mutable reference
    println!("s2_mutable after mutation: {}", s2_mutable);

    let s3_value = String::from("Stiven");
    print_value(s3_value); // Ownership moved to function
    // println!("s3_value: {}", s3_value); // Error: value moved
}

// Better: use &str when you don't need ownership
fn print_ref_string(s: &str) {
    println!("print_ref_string: {}", s);
}

// Mutable reference allows modification
fn print_ref_mutable_string(s: &mut String) {
    s.push_str(" Mora");
    println!("print_ref_mutable_string: {}", s);
}

// Takes ownership and consumes the value
fn print_value(mut s: String) {
    s.push_str(" Sigal");
    println!("print_value (owned): {}", s);
    // s is dropped here
}
