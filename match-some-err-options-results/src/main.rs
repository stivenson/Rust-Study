// Return result or None
fn get_slice_or_possible_none(phrase: String) -> Option<String> {
    if &phrase == "" {
        None
    } else {
        let first_letters = &phrase[0..2];
        Some(first_letters.to_string())
    }
}

// Return result or error
fn get_div_or_possible_error(operators: (u32, u32)) -> Result<u32, &'static str>{
    if operators.1 == 0 {
        Err("Second operator is zero")
    } else {
        Ok(operators.0 / operators.1)
    }
}

fn main() {

    // Omit check of possible None
    println!("First Letters of Empty string: {:?}",get_slice_or_possible_none("hi! Stivenson".to_string()).unwrap());
    println!("First Letters of string: {:?}",get_slice_or_possible_none("".to_string()));



    // Manage possible "None" or successful
    let res = get_slice_or_possible_none("hi! Stivenson".to_string());
    match res {
        Some(letters) => {
            println!("First Letters of string: {:?}", letters);
        },
        None => {
            println!("Empty String");
        }
    }
    let res = get_slice_or_possible_none("".to_string()); // overwriting to res
    match res {
        Some(letters) => {
            println!("First Letters of string: {:?}", letters);
        },
        None => {
            println!("Empty String");
        }
    }


    // Manage possible error or successful
    let possible_error = get_div_or_possible_error((10,2));
    match possible_error {
        Ok(result_div) => {
            println!("Div without error: {:?}", result_div);
        },
        Err(message) => {
            println!("Div with error: {:?}", message);
        }
    }
    let possible_error = get_div_or_possible_error((10,0)); // overwriting to possible_error
    match possible_error {
        Ok(result_div) => {
            println!("Div without error: {:?}", result_div);
        },
        Err(message) => {
            println!("Div with error: {:?}", message);
        }
    }
}
