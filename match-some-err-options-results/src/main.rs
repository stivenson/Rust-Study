// Return Option<String> - modern signature using &str
fn get_slice_or_possible_none(phrase: &str) -> Option<String> {
    if phrase.is_empty() {
        None
    } else if phrase.len() >= 2 {
        Some(phrase[0..2].to_string())
    } else {
        Some(phrase.to_string())
    }
}

// Return Result - modern error handling
fn get_div_or_possible_error(operators: (u32, u32)) -> Result<u32, &'static str> {
    if operators.1 == 0 {
        Err("Second operator is zero")
    } else {
        Ok(operators.0 / operators.1)
    }
}

fn main() {
    // Using unwrap (panics if None)
    println!(
        "First Letters: {}",
        get_slice_or_possible_none("hi! Stivenson").unwrap()
    );
    println!(
        "Empty string result: {:?}",
        get_slice_or_possible_none("")
    );

    // Pattern matching with Option
    if let Some(letters) = get_slice_or_possible_none("hi! Stivenson") {
        println!("First Letters using if let: {}", letters);
    }

    // Full match expression
    match get_slice_or_possible_none("") {
        Some(letters) => println!("First Letters: {}", letters),
        None => println!("Empty String"),
    }

    // Using unwrap_or for default values
    let result = get_slice_or_possible_none("ab").unwrap_or_else(|| "default".to_string());
    println!("Result with unwrap_or: {}", result);

    // Pattern matching with Result
    match get_div_or_possible_error((10, 2)) {
        Ok(result_div) => println!("Division result: {}", result_div),
        Err(message) => println!("Division error: {}", message),
    }

    match get_div_or_possible_error((10, 0)) {
        Ok(result_div) => println!("Division result: {}", result_div),
        Err(message) => println!("Division error: {}", message),
    }

    // Using ? operator alternative with unwrap_or
    let safe_div = get_div_or_possible_error((20, 4)).unwrap_or(0);
    println!("Safe division result: {}", safe_div);
}
