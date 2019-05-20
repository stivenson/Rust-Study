fn main() {
	let s1 = String::from("Stivenson Rincon");
	print_ref_string(&s1); // Sent by reference
	println!("s1: {:?}", s1);

	let mut s2_mutable = "Stivenson Rincon".to_string();
	print_ref_mutable_string(&mut s2_mutable); // Sent by mutable reference
	println!("s2_mutable: {:?}", s2_mutable);

	let s2_mutable_value = String::from("Stiven");
	print_value(s2_mutable_value); // Sent by value
	// println!("s2_mutable_value: {:?}", s2_mutable_value); // this line is not possible after send variable by value
}


fn print_ref_string(s: &String) {
	println!("print_ref_string s1: {:?}", s);
}


fn print_ref_mutable_string(s: &mut String) {
	s.push_str(" Mora");
	println!("print_ref_mutable_string s:{:?}", s);
}


fn print_value(mut s: String) {
	s.push_str(" Sigal");
	println!("print_value: {:?}", s);
}
