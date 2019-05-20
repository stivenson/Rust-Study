fn main() {

	let a_string_number = "42";
	let guess: u32 = a_string_number.parse().expect("Not a number!");
	println!("Guess: {:?}", guess);


	let u8_value: u8 = 4;
	println!("u8 value: {:?}", u8_value);



	let u16_value: u16 = 30000;
	println!("u16 value: {:?}", u16_value);


	let u_32_value: u32 = 3000000000;
	println!("u32 value: {:?}", u_32_value);

	let i_32_value_negative: i32 = -25000000;
	println!("i32 value negative: {:?}", i_32_value_negative);


	let u_64_value: u64 = 3000000000000000000;
	println!("u64 value: {:?}", u_64_value);


	let arch_value: isize = 3000000000000000000;
	println!("arch value: {:?}", arch_value);


	let f_32_value: f32 = 3.4;
	println!("f32 value: {:?}", f_32_value);

  let ch_heart_eyed_cat = "🤣";
  println!("emotic's character: {:?}", ch_heart_eyed_cat);

  let str_value = "Stivenson";
  println!("A String value: {:?}", str_value);


	println!("------------------------");
	// literals

	let literal_decimal_value = 24565;
	let literal_octal_value = 0o77;
	let literal_byte_value = b'X'; // Support u8 values
	let literal_float_value = 2.0;
	let _z = 'ℤ';

	println!("decimal, octal, float, and byte values: {:?}, {:?}, {:?}, {:?}",literal_decimal_value, literal_octal_value, literal_float_value, literal_byte_value);

	// more info: https://doc.rust-lang.org/book/ch03-02-data-types.html
}

