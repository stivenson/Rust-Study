fn main() {
    // String to number parsing
    let a_string_number = "42";
    let guess: u32 = a_string_number.parse().expect("Not a number!");
    println!("Guess: {}", guess);

    // Integer types
    let u8_value: u8 = 4;
    println!("u8 value: {}", u8_value);

    let u16_value: u16 = 30_000;
    println!("u16 value: {}", u16_value);

    let u32_value = 3_000_000_000u32;
    println!("u32 value: {}", u32_value);

    let i32_value_negative = -25_000_000i32;
    println!("i32 value negative: {}", i32_value_negative);

    let u64_value = 3_000_000_000_000_000_000u64;
    println!("u64 value: {}", u64_value);

    // Architecture-dependent integer (isize/usize)
    let arch_value: isize = 3_000_000_000_000_000_000;
    println!("arch value: {}", arch_value);

    // Floating point types
    let f32_value = 3.4f32;
    println!("f32 value: {}", f32_value);

    // Characters and strings
    let ch_heart_eyed_cat = "🤣";
    println!("emoji character: {}", ch_heart_eyed_cat);

    let str_value = "Stivenson";
    println!("A String value: {}", str_value);

    println!("------------------------");
    // Number literals

    let literal_decimal_value = 24_565;
    let literal_octal_value = 0o77;
    let literal_byte_value = b'X'; // u8 values only
    let literal_float_value = 2.0;
    let _z = 'ℤ'; // Unicode character

    println!(
        "decimal, octal, float, and byte values: {}, {}, {}, {}",
        literal_decimal_value, literal_octal_value, literal_float_value, literal_byte_value
    );

    // More info: https://doc.rust-lang.org/book/ch03-02-data-types.html
}

